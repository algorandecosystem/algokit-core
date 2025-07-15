use num_bigint::BigUint;

pub fn big_uint_to_bytes(value: BigUint, byte_len: usize) -> Vec<u8> {
    let mut value_bytes = value.to_bytes_be();
    let mut bytes = vec![0u8; byte_len];

    for i in 0..byte_len {
        if let Some(byte) = value_bytes.pop() {
            bytes[byte_len - 1 - i] = byte;
        } else {
            break;
        }
    }

    bytes
}
