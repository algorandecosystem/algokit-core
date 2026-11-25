//! The msgpack subset Algorand's canonical encoding uses: decoding into a
//! value tree (for state proofs), and encoding helpers for the canonical
//! (sorted-key, empty-fields-omitted) structs that get hashed.

use alloc::string::String;
use alloc::vec::Vec;

/// A decoded msgpack value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Nil,
    Bool(bool),
    Uint(u64),
    Int(i64),
    Bytes(Vec<u8>),
    Str(String),
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
}

/// Why decoding failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeError;

/// Deepest nesting accepted (a state proof nests ~6 levels).
const MAX_DEPTH: usize = 32;

/// Decodes exactly one value spanning all of `bytes`.
pub fn decode(bytes: &[u8]) -> Result<Value, DecodeError> {
    let mut r = Reader { bytes, pos: 0 };
    let value = r.value(0)?;
    if r.pos != bytes.len() {
        return Err(DecodeError);
    }
    Ok(value)
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        let end = self.pos.checked_add(n).ok_or(DecodeError)?;
        let out = self.bytes.get(self.pos..end).ok_or(DecodeError)?;
        self.pos = end;
        Ok(out)
    }

    fn uint(&mut self, n: usize) -> Result<u64, DecodeError> {
        Ok(self
            .take(n)?
            .iter()
            .fold(0u64, |acc, b| (acc << 8) | u64::from(*b)))
    }

    fn len(&mut self, n: usize) -> Result<usize, DecodeError> {
        usize::try_from(self.uint(n)?).map_err(|_| DecodeError)
    }

    fn value(&mut self, depth: usize) -> Result<Value, DecodeError> {
        if depth > MAX_DEPTH {
            return Err(DecodeError);
        }
        let tag = self.take(1)?[0];
        Ok(match tag {
            0x00..=0x7f => Value::Uint(u64::from(tag)),
            0x80..=0x8f => self.map(usize::from(tag & 0x0f), depth)?,
            0x90..=0x9f => self.array(usize::from(tag & 0x0f), depth)?,
            0xa0..=0xbf => self.str(usize::from(tag & 0x1f))?,
            0xc0 => Value::Nil,
            0xc2 => Value::Bool(false),
            0xc3 => Value::Bool(true),
            0xc4 => {
                let n = self.len(1)?;
                Value::Bytes(self.take(n)?.to_vec())
            }
            0xc5 => {
                let n = self.len(2)?;
                Value::Bytes(self.take(n)?.to_vec())
            }
            0xc6 => {
                let n = self.len(4)?;
                Value::Bytes(self.take(n)?.to_vec())
            }
            0xcc => Value::Uint(self.uint(1)?),
            0xcd => Value::Uint(self.uint(2)?),
            0xce => Value::Uint(self.uint(4)?),
            0xcf => Value::Uint(self.uint(8)?),
            0xd0 => Value::Int(i64::from(self.uint(1)? as u8 as i8)),
            0xd1 => Value::Int(i64::from(self.uint(2)? as u16 as i16)),
            0xd2 => Value::Int(i64::from(self.uint(4)? as u32 as i32)),
            0xd3 => Value::Int(self.uint(8)? as i64),
            0xd9 => {
                let n = self.len(1)?;
                self.str(n)?
            }
            0xda => {
                let n = self.len(2)?;
                self.str(n)?
            }
            0xdb => {
                let n = self.len(4)?;
                self.str(n)?
            }
            0xdc => {
                let n = self.len(2)?;
                self.array(n, depth)?
            }
            0xdd => {
                let n = self.len(4)?;
                self.array(n, depth)?
            }
            0xde => {
                let n = self.len(2)?;
                self.map(n, depth)?
            }
            0xdf => {
                let n = self.len(4)?;
                self.map(n, depth)?
            }
            0xe0..=0xff => Value::Int(i64::from(tag as i8)),
            // Floats and extensions never appear in these structures.
            _ => return Err(DecodeError),
        })
    }

    fn str(&mut self, n: usize) -> Result<Value, DecodeError> {
        let bytes = self.take(n)?;
        Ok(Value::Str(String::from(
            core::str::from_utf8(bytes).map_err(|_| DecodeError)?,
        )))
    }

    fn array(&mut self, n: usize, depth: usize) -> Result<Value, DecodeError> {
        // Every element takes at least one byte: bounds allocation by input.
        if n > self.bytes.len() - self.pos {
            return Err(DecodeError);
        }
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.value(depth + 1)?);
        }
        Ok(Value::Array(out))
    }

    fn map(&mut self, n: usize, depth: usize) -> Result<Value, DecodeError> {
        if n > (self.bytes.len() - self.pos) / 2 {
            return Err(DecodeError);
        }
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            let key = self.value(depth + 1)?;
            let value = self.value(depth + 1)?;
            out.push((key, value));
        }
        Ok(Value::Map(out))
    }
}

impl Value {
    /// The field `key` of a struct encoded as a string-keyed map. A missing
    /// field is `None` (it was omitted for being empty).
    pub fn field(&self, key: &str) -> Result<Option<&Value>, DecodeError> {
        let Value::Map(entries) = self else {
            return Err(DecodeError);
        };
        Ok(entries
            .iter()
            .find(|(k, _)| matches!(k, Value::Str(s) if s == key))
            .map(|(_, v)| v))
    }

    pub fn as_u64(&self) -> Result<u64, DecodeError> {
        match self {
            Value::Uint(v) => Ok(*v),
            Value::Int(v) if *v >= 0 => Ok(*v as u64),
            _ => Err(DecodeError),
        }
    }

    pub fn as_bytes(&self) -> Result<&[u8], DecodeError> {
        match self {
            Value::Bytes(b) => Ok(b),
            Value::Str(s) => Ok(s.as_bytes()),
            _ => Err(DecodeError),
        }
    }

    pub fn as_array(&self) -> Result<&[Value], DecodeError> {
        match self {
            Value::Array(a) => Ok(a),
            Value::Nil => Ok(&[]),
            _ => Err(DecodeError),
        }
    }

    pub fn as_map(&self) -> Result<&[(Value, Value)], DecodeError> {
        match self {
            Value::Map(m) => Ok(m),
            Value::Nil => Ok(&[]),
            _ => Err(DecodeError),
        }
    }
}

/// Field accessors defaulting omitted fields to their empty value.
pub fn u64_field(v: &Value, key: &str) -> Result<u64, DecodeError> {
    v.field(key)?.map_or(Ok(0), Value::as_u64)
}

pub fn bytes_field(v: &Value, key: &str) -> Result<Vec<u8>, DecodeError> {
    v.field(key)?
        .map_or(Ok(Vec::new()), |b| b.as_bytes().map(<[u8]>::to_vec))
}

/// A fixed-size byte array field; omitted means all zeros.
pub fn array_field<const N: usize>(v: &Value, key: &str) -> Result<[u8; N], DecodeError> {
    let bytes = bytes_field(v, key)?;
    if bytes.is_empty() {
        return Ok([0; N]);
    }
    bytes.try_into().map_err(|_| DecodeError)
}

// --- Canonical encoding -------------------------------------------------

pub fn write_map_header(out: &mut Vec<u8>, n: usize) {
    if n < 16 {
        out.push(0x80 | n as u8);
    } else {
        out.push(0xde);
        out.extend_from_slice(&(n as u16).to_be_bytes());
    }
}

pub fn write_str(out: &mut Vec<u8>, s: &str) {
    debug_assert!(s.len() < 32);
    out.push(0xa0 | s.len() as u8);
    out.extend_from_slice(s.as_bytes());
}

pub fn write_bin(out: &mut Vec<u8>, b: &[u8]) {
    if b.len() < 256 {
        out.push(0xc4);
        out.push(b.len() as u8);
    } else if b.len() < 65536 {
        out.push(0xc5);
        out.extend_from_slice(&(b.len() as u16).to_be_bytes());
    } else {
        out.push(0xc6);
        out.extend_from_slice(&(b.len() as u32).to_be_bytes());
    }
    out.extend_from_slice(b);
}

pub fn write_uint(out: &mut Vec<u8>, v: u64) {
    if v < 128 {
        out.push(v as u8);
    } else if v < 1 << 8 {
        out.push(0xcc);
        out.push(v as u8);
    } else if v < 1 << 16 {
        out.push(0xcd);
        out.extend_from_slice(&(v as u16).to_be_bytes());
    } else if v < 1 << 32 {
        out.push(0xce);
        out.extend_from_slice(&(v as u32).to_be_bytes());
    } else {
        out.push(0xcf);
        out.extend_from_slice(&v.to_be_bytes());
    }
}

/// One field of a canonical struct; `None` = empty, so omitted.
pub enum Field<'a> {
    Uint(u64),
    Bin(&'a [u8]),
}

impl Field<'_> {
    fn is_empty(&self) -> bool {
        match self {
            Field::Uint(v) => *v == 0,
            // Covers both empty slices and all-zero fixed arrays
            // (`omitempty,omitemptyarray`).
            Field::Bin(b) => b.iter().all(|x| *x == 0),
        }
    }
}

/// go-codec's canonical encoding of a struct tagged
/// `omitempty,omitemptyarray`: a map of the non-empty fields, keys sorted.
/// `fields` must already be sorted by key.
pub fn encode_struct(fields: &[(&str, Field<'_>)]) -> Vec<u8> {
    debug_assert!(fields.windows(2).all(|w| w[0].0 < w[1].0));
    let present: Vec<_> = fields.iter().filter(|(_, f)| !f.is_empty()).collect();
    let mut out = Vec::new();
    write_map_header(&mut out, present.len());
    for (key, field) in present {
        write_str(&mut out, key);
        match field {
            Field::Uint(v) => write_uint(&mut out, *v),
            Field::Bin(b) => write_bin(&mut out, b),
        }
    }
    out
}
