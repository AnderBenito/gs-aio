pub fn get_int32(buffer: &[u8], pos: usize) -> i32 {
    (buffer[pos] as i32)
        | ((buffer[pos + 1] as i32) << 8)
        | ((buffer[pos + 2] as i32) << 16)
        | ((buffer[pos + 3] as i32) << 24)
}

pub fn get_int16(buffer: &[u8], pos: i32) -> i32 {
    let rpos = pos as usize;
    let pos1 = buffer[rpos] as i32;
    let pos2 = buffer[rpos + 1] as i32;
    pos1 | (pos2 << 8)
}

// TODO: Refactor this
pub fn get_string(buffer: &[u8], pos: usize, length: usize) -> String {
    let mut strbuild = Vec::new();
    let mut cursor_pos = pos;
    let mut cursor_length = length;

    while cursor_length > 0 {
        strbuild.push(buffer[cursor_pos] as char);
        cursor_pos += 1;
        cursor_length -= 1;
    }

    strbuild.into_iter().collect()
}

pub fn set_int32(buffer: &mut [u8], pos: usize, value: i32) {
    buffer[pos] = value as u8;
    buffer[pos + 1] = (value >> 8) as u8;
    buffer[pos + 2] = (value >> 16) as u8;
    buffer[pos + 3] = (value >> 24) as u8;
}
