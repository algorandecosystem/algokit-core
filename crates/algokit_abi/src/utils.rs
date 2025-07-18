// TODO: maybe refactor this to only fix length for bigint
pub fn extend_bytes_to_length(bytes: &[u8], len: usize) -> Vec<u8> {
    let mut result = vec![0u8; len - bytes.len()];
    result.extend_from_slice(bytes);
    result
}
