use super::{
    utils::{get_int16, get_int32, set_int32},
    TextDecompressionError,
};

pub fn decomp_text(src: &[u8]) -> Result<Vec<u8>, TextDecompressionError> {
    std::panic::catch_unwind(|| decomp_text_unsafe(src))
        .map_err(|e| TextDecompressionError::Custom(format!("{:?}", e)))
}

fn decomp_text_unsafe(src: &[u8]) -> Vec<u8> {
    let mut _total = 0;
    let asmpchar = get_int32(src, 0x38578) - 0x8000000;
    let asmptext = get_int32(src, 0x385DC) - 0x8000000;
    let mut chardata = get_int32(src, asmpchar as usize) - 0x08000000;
    let mut charpntrs = get_int32(src, (asmpchar + 4) as usize) - 0x08000000;

    let mut max_letter = 0;
    let mut c_tree_size = 0;
    let mut max_depth = 0;

    // Pre-scan character tables
    let mut char1 = -1;
    while char1 + 1 <= max_letter {
        char1 += 1;
        if char1 & 0xFF == 0 {
            chardata = get_int32(src, (asmpchar + (char1 >> 8) * 8) as usize) - 0x08000000;
            charpntrs = get_int32(src, (asmpchar + (char1 >> 8) * 8 + 4) as usize) - 0x08000000;
        }

        let cmp = get_int16(src, charpntrs);
        if cmp == 0x8000 {
            charpntrs += 2;
            continue;
        }

        let mut char_tree = (chardata + get_int16(src, charpntrs)) << 3;
        charpntrs += 2;
        let mut char_slot = char_tree - 12;
        let mut depth = 0;

        loop {
            loop {
                if ((src[(char_tree >> 3) as usize] >> (char_tree & 7)) & 1) != 0 {
                    char_tree += 1;
                    break;
                }
                depth += 1;
                c_tree_size += 1;
                char_tree += 1;
            }
            let char_slot_pos = char_slot >> 3;
            let letter = (get_int16(src, char_slot_pos) >> (char_slot & 7)) & 0xFFF;
            char_slot -= 12;
            _total += 1;
            c_tree_size += 1;

            if letter > max_letter {
                max_letter = letter;
            }
            if depth > max_depth {
                max_depth = depth;
            }
            if depth <= 0 {
                break;
            }
            depth -= 1;
        }
    }

    // Initialize arrays
    let mut ct_offsets = vec![0; (max_letter + 1) as usize];
    let mut c_tree = vec![0; c_tree_size as usize];
    let mut node_offsets = vec![0; max_depth as usize];

    let mut pos = 0;
    for char1 in 0..=max_letter {
        if char1 & 0xFF == 0 {
            chardata = get_int32(src, (asmpchar + (char1 >> 8) * 8) as usize) - 0x08000000;
            charpntrs = get_int32(src, (asmpchar + (char1 >> 8) * 8 + 4) as usize) - 0x08000000;
        }

        if get_int16(src, charpntrs) == 0x8000 {
            charpntrs += 2;
            continue;
        }

        let mut char_tree = (chardata + get_int16(src, charpntrs)) << 3;
        charpntrs += 2;
        let mut char_slot = char_tree - 12;
        let mut depth = 0;

        ct_offsets[char1 as usize] = pos;
        loop {
            while (src[(char_tree >> 3) as usize] >> (char_tree & 7)) & 1 == 0 {
                node_offsets[depth] = pos;
                depth += 1;
                pos += 1;
                char_tree += 1;
            }
            char_tree += 1;
            let char_slot_pos = char_slot >> 3;
            assert_ne!(char_slot_pos, -1);
            c_tree[pos] = -((get_int16(src, char_slot_pos) >> (char_slot & 7)) & 0xFFF) as i32;
            pos += 1;
            char_slot -= 12;

            if depth <= 0 {
                break;
            }
            depth -= 1;
            c_tree[node_offsets[depth] as usize] = pos as i32;
        }
    }

    // Decompression
    let mut des = vec![0u8; 0x800000];
    let mut des_entry = 0;
    let mut des_pos = 0xC300;

    let mut text_tree = 0;
    let mut text_len_addr = 0;

    for src_i in 0..12461 {
        set_int32(&mut des, des_entry as usize, (des_pos - 0xC300) as i32);
        des_entry += 4;

        if src_i & 0xFF == 0 {
            text_tree = get_int32(src, (asmptext + (src_i >> 8) * 8) as usize) - 0x08000000;
            text_len_addr = get_int32(src, (asmptext + (src_i >> 8) * 8 + 4) as usize) - 0x08000000;
        } else {
            loop {
                let c_len = src[text_len_addr as usize];
                text_len_addr += 1;
                text_tree += c_len as i32;
                if c_len != 0xFF {
                    break;
                }
            }
        }

        let mut init_char = 0;
        let mut text_tree2 = text_tree << 3;

        loop {
            pos = ct_offsets[init_char as usize];
            while c_tree[pos as usize] > 0 {
                if (src[(text_tree2 >> 3) as usize] >> (text_tree2 & 7)) & 1 == 0 {
                    pos += 1;
                } else {
                    pos = c_tree[pos] as usize;
                }
                text_tree2 += 1;
            }
            init_char = -c_tree[pos as usize];
            des[des_pos as usize] = init_char as u8;
            des_pos += 1;

            if init_char == 0 {
                break;
            }
        }
    }

    des
}
