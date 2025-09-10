use crate::arc56_contract::{StructField as Arc56StructField, StructFieldType};
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

/// Represents a field in a struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    /// The name of the field
    pub name: String,
    /// The ABI type of the field
    pub abi_type: Box<ABIType>,
}

impl ABIStruct {
    /// Create a new struct type
    pub fn new(name: impl Into<String>, fields: Vec<StructField>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }

    /// Create from ARC-56 structs definition with recursive resolution
    pub fn get_abi_struct_type(
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
            fields.push(StructField::new(&arc56_field.name, field_type));
        }

        Ok(Self::new(struct_name, fields))
    }

    /// Recursively resolve field types from ARC-56 definitions
    fn resolve_field_type(
        field_type: &StructFieldType,
        structs: &HashMap<String, Vec<Arc56StructField>>,
    ) -> Result<ABIType, ABIError> {
        match field_type {
            StructFieldType::Value(type_str) => {
                // Check if this is a reference to another struct
                if structs.contains_key(type_str) {
                    let nested_struct = Self::get_abi_struct_type(type_str, structs)?;
                    Ok(ABIType::Struct(nested_struct))
                } else {
                    // Parse as regular ABI type
                    ABIType::from_str(type_str).map_err(|e| ABIError::ValidationError {
                        message: format!("Failed to parse field type '{}': {}", type_str, e),
                    })
                }
            }
            StructFieldType::Nested(nested_fields) => {
                // Handle anonymous nested struct fields
                let mut resolved_fields = Vec::new();
                for nested_field in nested_fields {
                    let field_type = Self::resolve_field_type(&nested_field.field_type, structs)?;
                    resolved_fields.push(StructField::new(&nested_field.name, field_type));
                }
                Ok(ABIType::Tuple(
                    resolved_fields.into_iter().map(|f| *f.abi_type).collect(),
                ))
            }
        }
    }

    /// Convert this struct type to an equivalent tuple type
    pub fn to_tuple_type(&self) -> ABIType {
        let tuple_types: Vec<ABIType> = self
            .fields
            .iter()
            .map(|field| (*field.abi_type).clone())
            .collect();
        ABIType::Tuple(tuple_types)
    }

    /// Encode struct value using tuple encoding
    pub fn encode(&self, value: &ABIValue) -> Result<Vec<u8>, ABIError> {
        match value {
            ABIValue::Struct(struct_map) => {
                let tuple_values = self.struct_to_tuple(struct_map)?;
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
                let struct_map = self.tuple_to_struct(tuple_values)?;
                Ok(ABIValue::Struct(struct_map))
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
    pub fn struct_to_tuple(
        &self,
        struct_map: &HashMap<String, ABIValue>,
    ) -> Result<Vec<ABIValue>, ABIError> {
        let mut tuple_values = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            let value = struct_map
                .get(&field.name)
                .ok_or_else(|| ABIError::ValidationError {
                    message: format!("Missing field '{}' in struct '{}'", field.name, self.name),
                })?;

            // Recursively handle nested structs
            let processed_value = match (field.abi_type.as_ref(), value) {
                (ABIType::Struct(nested_struct), ABIValue::Struct(nested_map)) => {
                    let nested_tuple = nested_struct.struct_to_tuple(nested_map)?;
                    ABIValue::Array(nested_tuple)
                }
                _ => value.clone(),
            };

            tuple_values.push(processed_value);
        }

        Ok(tuple_values)
    }

    /// Convert a tuple value (Vec) to a struct value (HashMap) after decoding
    pub fn tuple_to_struct(
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

        let mut struct_map = HashMap::with_capacity(self.fields.len());

        for (field, value) in self.fields.iter().zip(tuple_values.into_iter()) {
            // Recursively handle nested structs
            let processed_value = match (field.abi_type.as_ref(), &value) {
                (ABIType::Struct(nested_struct), ABIValue::Array(nested_tuple)) => {
                    let nested_map = nested_struct.tuple_to_struct(nested_tuple.clone())?;
                    ABIValue::Struct(nested_map)
                }
                _ => value,
            };

            struct_map.insert(field.name.clone(), processed_value);
        }

        Ok(struct_map)
    }
}

impl StructField {
    /// Create a new struct field
    pub fn new(name: impl Into<String>, abi_type: ABIType) -> Self {
        Self {
            name: name.into(),
            abi_type: Box::new(abi_type),
        }
    }
}

impl Display for ABIStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi_type::BitSize;
    use num_bigint::BigUint;

    #[test]
    fn test_struct_to_tuple_type() {
        let struct_type = ABIStruct::new(
            "Person",
            vec![
                StructField::new("name", ABIType::String),
                StructField::new("age", ABIType::Uint(BitSize::new(64).unwrap())),
                StructField::new("active", ABIType::Bool),
            ],
        );

        let tuple_type = struct_type.to_tuple_type();
        match tuple_type {
            ABIType::Tuple(types) => {
                assert_eq!(types.len(), 3);
                assert_eq!(types[0], ABIType::String);
                assert_eq!(types[1], ABIType::Uint(BitSize::new(64).unwrap()));
                assert_eq!(types[2], ABIType::Bool);
            }
            _ => panic!("Expected tuple type"),
        }
    }

    #[test]
    fn test_struct_to_tuple_conversion() {
        let struct_type = ABIStruct::new(
            "Point",
            vec![
                StructField::new("x", ABIType::Uint(BitSize::new(32).unwrap())),
                StructField::new("y", ABIType::Uint(BitSize::new(32).unwrap())),
            ],
        );

        let mut struct_map = HashMap::new();
        struct_map.insert("x".to_string(), ABIValue::Uint(BigUint::from(10u32)));
        struct_map.insert("y".to_string(), ABIValue::Uint(BigUint::from(20u32)));

        let tuple_values = struct_type.struct_to_tuple(&struct_map).unwrap();
        assert_eq!(tuple_values.len(), 2);
        assert_eq!(tuple_values[0], ABIValue::Uint(BigUint::from(10u32)));
        assert_eq!(tuple_values[1], ABIValue::Uint(BigUint::from(20u32)));
    }

    #[test]
    fn test_tuple_to_struct_conversion() {
        let struct_type = ABIStruct::new(
            "Point",
            vec![
                StructField::new("x", ABIType::Uint(BitSize::new(32).unwrap())),
                StructField::new("y", ABIType::Uint(BitSize::new(32).unwrap())),
            ],
        );

        let tuple_values = vec![
            ABIValue::Uint(BigUint::from(10u32)),
            ABIValue::Uint(BigUint::from(20u32)),
        ];

        let struct_map = struct_type.tuple_to_struct(tuple_values).unwrap();
        assert_eq!(struct_map.len(), 2);
        assert_eq!(
            struct_map.get("x"),
            Some(&ABIValue::Uint(BigUint::from(10u32)))
        );
        assert_eq!(
            struct_map.get("y"),
            Some(&ABIValue::Uint(BigUint::from(20u32)))
        );
    }

    #[test]
    fn test_missing_field_error() {
        let struct_type = ABIStruct::new(
            "Point",
            vec![
                StructField::new("x", ABIType::Uint(BitSize::new(32).unwrap())),
                StructField::new("y", ABIType::Uint(BitSize::new(32).unwrap())),
            ],
        );

        let mut struct_map = HashMap::new();
        struct_map.insert("x".to_string(), ABIValue::Uint(BigUint::from(10u32)));
        // Missing "y" field

        let result = struct_type.struct_to_tuple(&struct_map);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing field 'y'")
        );
    }

    #[test]
    fn test_nested_struct() {
        // Create inner struct type
        let inner_struct = ABIStruct::new(
            "Address",
            vec![
                StructField::new("street", ABIType::String),
                StructField::new("city", ABIType::String),
            ],
        );

        // Create outer struct with nested struct
        let outer_struct = ABIStruct::new(
            "Person",
            vec![
                StructField::new("name", ABIType::String),
                StructField::new("address", ABIType::Struct(inner_struct.clone())),
            ],
        );

        // Create nested struct value
        let mut address_map = HashMap::new();
        address_map.insert(
            "street".to_string(),
            ABIValue::String("123 Main St".to_string()),
        );
        address_map.insert(
            "city".to_string(),
            ABIValue::String("Springfield".to_string()),
        );

        let mut person_map = HashMap::new();
        person_map.insert("name".to_string(), ABIValue::String("Alice".to_string()));
        person_map.insert("address".to_string(), ABIValue::Struct(address_map));

        // Convert to tuple
        let tuple_values = outer_struct.struct_to_tuple(&person_map).unwrap();
        assert_eq!(tuple_values.len(), 2);
        assert_eq!(tuple_values[0], ABIValue::String("Alice".to_string()));

        // The nested struct should be converted to array for tuple representation
        match &tuple_values[1] {
            ABIValue::Array(nested_array) => {
                assert_eq!(nested_array.len(), 2);
                assert_eq!(nested_array[0], ABIValue::String("123 Main St".to_string()));
                assert_eq!(nested_array[1], ABIValue::String("Springfield".to_string()));
            }
            _ => panic!("Expected nested struct to be converted to array"),
        }
    }
}
