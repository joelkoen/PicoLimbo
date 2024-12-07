pub fn read_long(bytes: &[u8], index: &mut usize) -> i64 {
    let value = ((bytes[*index] as i64) << 56)
        | ((bytes[*index + 1] as i64) << 48)
        | ((bytes[*index + 2] as i64) << 40)
        | ((bytes[*index + 3] as i64) << 32)
        | ((bytes[*index + 4] as i64) << 24)
        | ((bytes[*index + 5] as i64) << 16)
        | ((bytes[*index + 6] as i64) << 8)
        | (bytes[*index + 7] as i64);
    *index += 8;
    value
}