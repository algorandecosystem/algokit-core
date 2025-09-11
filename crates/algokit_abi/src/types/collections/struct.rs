use crate::arc56_contract::{
    StructField as Arc56StructField, StructFieldType as Arc56StructFieldType,
};
use crate::{ABIError, ABIType, ABIValue};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Represents an ABI struct type with named fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ABIStruct {
    /// The name of the struct type
    pub name: String,
    /// The fields of the struct in order
    pub fields: Vec<StructField>,
}

/// Represents the type of a struct field
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructFieldType {
    Type(ABIType),
    Fields(Vec<StructField>),
}

/// Represents a field in a struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub field_type: StructFieldType,
}

impl ABIStruct {
    pub(crate) fn get_abi_struct_type(
        struct_name: &str,
        structs: &HashMap<String, Vec<Arc56StructField>>,
    ) -> Result<Self, ABIError> {
        let arc56_fields = structs
            .get(struct_name)
            .ok_or_else(|| ABIError::ValidationError {
                message: format!("Struct '{}' not found in ARC-56 definition", struct_name),
            })?;

        let mut fields = Vec::new();
        for arc56_field in arc56_fields {
            let field_type = Self::resolve_field_type(&arc56_field.field_type, structs)?;
            fields.push(StructField {
                name: arc56_field.name.clone(),
                field_type,
            });
        }

        Ok(Self {
            name: struct_name.to_string(),
            fields: fields,
        })
    }

    fn resolve_field_type(
        field_type: &Arc56StructFieldType,
        structs: &HashMap<String, Vec<Arc56StructField>>,
    ) -> Result<StructFieldType, ABIError> {
        match field_type {
            Arc56StructFieldType::Value(type_str) => {
                // Check if this is a reference to another struct
                if structs.contains_key(type_str) {
                    let nested_struct = Self::get_abi_struct_type(type_str, structs)?;
                    Ok(StructFieldType::Type(ABIType::Struct(nested_struct)))
                } else {
                    // Parse as regular ABI type
                    let abi_type = ABIType::from_str(type_str)?;
                    Ok(StructFieldType::Type(abi_type))
                }
            }
            Arc56StructFieldType::Nested(nested_fields) => {
                // Handle anonymous nested struct fields
                let mut resolved_fields = Vec::new();
                for nested_field in nested_fields {
                    let field_type = Self::resolve_field_type(&nested_field.field_type, structs)?;
                    resolved_fields.push(StructField {
                        name: nested_field.name.clone(),
                        field_type,
                    });
                }
                Ok(StructFieldType::Fields(resolved_fields))
            }
        }
    }

    pub fn to_tuple_type(&self) -> ABIType {
        Self::fields_to_tuple_type(&self.fields)
    }

    fn fields_to_tuple_type(fields: &[StructField]) -> ABIType {
        let child_types: Vec<ABIType> = fields
            .iter()
            .map(|field| match &field.field_type {
                StructFieldType::Fields(nested_fields) => Self::fields_to_tuple_type(nested_fields),
                StructFieldType::Type(ABIType::Struct(struct_type)) => struct_type.to_tuple_type(),
                StructFieldType::Type(other_type) => other_type.clone(),
            })
            .collect();
        ABIType::Tuple(child_types)
    }

    /// Encode struct value using tuple encoding
    pub fn encode(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match value {
            ABIValue::Struct(value) => {
                let tuple_values = self.value_to_tuple_values(value)?;
                let tuple_type = self.to_tuple_type();
                tuple_type.encode(&ABIValue::Array(tuple_values))
            }
            _ => Err(ABIError::ValidationError {
                message: format!("Cannot encode non-struct value as struct '{}'", self.name),
            }),
        }
    }

    /// Decode bytes using tuple decoding
    pub fn decode(&self, bytes: &[u8]) -> Result<ABIValue, ABIError> {
        let tuple_type = self.to_tuple_type();
        let decoded_tuple = tuple_type.decode(bytes)?;

        match decoded_tuple {
            ABIValue::Array(tuple_values) => {
                let value = self.get_value_from_tuple_values(tuple_values)?;
                Ok(ABIValue::Struct(value))
            }
            _ => Err(ABIError::DecodingError {
                message: format!(
                    "Expected array from tuple decode for struct '{}'",
                    self.name
                ),
            }),
        }
    }

    /// Convert a struct value (HashMap) to a tuple value (Vec) for encoding
    pub fn value_to_tuple_values(
        &self,
        value: &HashMap<String, ABIValue>,
    ) -> Result<Vec<ABIValue>, ABIError> {
        Self::field_values_to_tuple_values(&self.fields, value, &self.name)
    }

    fn field_values_to_tuple_values(
        fields: &[StructField],
        struct_value: &HashMap<String, ABIValue>,
        struct_name: &str,
    ) -> Result<Vec<ABIValue>, ABIError> {
        fields
            .iter()
            .map(|field| {
                let value =
                    struct_value
                        .get(&field.name)
                        .ok_or_else(|| ABIError::ValidationError {
                            message: format!(
                                "Missing field '{}' in struct '{}'",
                                field.name, struct_name
                            ),
                        })?;

                match (&field.field_type, value) {
                    (
                        StructFieldType::Fields(nested_fields),
                        ABIValue::Struct(nested_struct_value),
                    ) => {
                        let nested_tuple_values = Self::field_values_to_tuple_values(
                            nested_fields,
                            nested_struct_value,
                            "anonymous",
                        )?;
                        Ok(ABIValue::Array(nested_tuple_values))
                    }
                    (
                        StructFieldType::Type(ABIType::Struct(nested_struct)),
                        ABIValue::Struct(nested_struct_value),
                    ) => {
                        let nested_tuple_values =
                            nested_struct.value_to_tuple_values(nested_struct_value)?;
                        Ok(ABIValue::Array(nested_tuple_values))
                    }
                    _ => Ok(value.clone()),
                }
            })
            .collect()
    }

    pub fn get_value_from_tuple_values(
        &self,
        tuple_values: Vec<ABIValue>,
    ) -> Result<HashMap<String, ABIValue>, ABIError> {
        if tuple_values.len() != self.fields.len() {
            return Err(ABIError::ValidationError {
                message: format!(
                    "Tuple length {} doesn't match struct '{}' field count {}",
                    tuple_values.len(),
                    self.name,
                    self.fields.len()
                ),
            });
        }

        Self::get_field_values(&self.fields, tuple_values)
    }

    fn get_field_values(
        fields: &[StructField],
        values: Vec<ABIValue>,
    ) -> Result<HashMap<String, ABIValue>, ABIError> {
        fields
            .iter()
            .zip(values.into_iter())
            .map(|(field, value)| {
                let processed_value = match (&field.field_type, value) {
                    (StructFieldType::Fields(nested_fields), ABIValue::Array(nested_tuple)) => {
                        let nested_map = Self::get_field_values(nested_fields, nested_tuple)?;
                        ABIValue::Struct(nested_map)
                    }
                    (
                        StructFieldType::Type(ABIType::Struct(nested_struct)),
                        ABIValue::Array(nested_tuple),
                    ) => {
                        let nested_value =
                            nested_struct.get_value_from_tuple_values(nested_tuple)?;
                        ABIValue::Struct(nested_value)
                    }
                    (_, other_value) => other_value,
                };
                Ok((field.name.clone(), processed_value))
            })
            .collect()
    }
}

impl Display for ABIStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}
