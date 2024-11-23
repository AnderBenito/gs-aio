pub fn get_int32(buffer: &[u8], pos: usize) -> i32 {
    (buffer[pos] as i32)
        | ((buffer[pos + 1] as i32) << 8)
        | ((buffer[pos + 2] as i32) << 16)
        | ((buffer[pos + 3] as i32) << 24)
}

pub fn get_int16(buffer: &[u8], pos: usize) -> i32 {
    let pos1 = buffer[pos] as i32;
    let pos2 = buffer[pos + 1] as i32;
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

pub fn get_text_short(txt: &[u8], index: usize) -> Result<String, ()> {
    // Get source position
    let mut src_pos = get_int32(txt, index << 2) as usize;
    if get_int32(txt, 0) == 0 {
        src_pos += 0xC300;
    }

    // Read text data
    let mut str_builder = String::new();
    let mut p: u8 = 0; // Previous byte
    loop {
        let n = *txt.get(src_pos).ok_or(())?;
        src_pos += 1;

        if n != 0 {
            if n < 32 || n > 0x7E {
                // Non-printable or special characters
                // str_builder.push_str(format!("[{}]", n).as_str()); // Append printable characters
                str_builder.push(n as char); // Append printable characters

                // Commands with arguments: skip certain conditions
                if (n == 1 || n == 3) && (p < 17 || p > 20) && p != 26 && p != 29 {
                    break; // Skip further processing if this condition matches
                }
            } else {
                str_builder.push(n as char); // Append printable characters
            }
        }

        p = n;
        if n == 0 {
            break; // End of string
        }
    }

    Ok(str_builder)
}
