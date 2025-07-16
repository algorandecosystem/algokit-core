pub fn extend_bytes_to_length(bytes: Vec<u8>, len: usize) -> Vec<u8> {
    let result = vec![0u8; len - bytes.len()];
    vec![0u8; len - bytes.len()].extend_from_slice(&bytes);
    result
}
