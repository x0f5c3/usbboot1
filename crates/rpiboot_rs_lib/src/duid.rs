// DUID decoding logic for rpiboot_rs_lib

pub fn duid_decode_c40(str_of_words: &str) -> Result<String, ()> {
    let mut c40_list = Vec::new();
    let mut i = 0;
    let mut word_strs = str_of_words.split('_');
    while let Some(word_str) = word_strs.next() {
        let word = match u32::from_str_radix(word_str, 16) {
            Ok(w) => w,
            Err(_) => break,
        };
        decode_half_word((word & 0xFFFF) as u16, &mut c40_list, &mut i);
        let msig = (word >> 16) as u16;
        if msig > 0 {
            decode_half_word(msig, &mut c40_list, &mut i);
        }
    }
    let mut c40_str = String::new();
    for c in 0..i {
        match c40_to_char(c40_list[c]) {
            Some(ch) => c40_str.push(ch),
            None => return Err(()),
        }
    }
    Ok(c40_str)
}

fn char_to_c40(val: char) -> Option<i32> {
    let v = val as u8;
    if v >= b'a' && v <= b'z' {
        return char_to_c40((v - 32) as char);
    }
    if v >= b'0' && v <= b'9' {
        Some(4 + (v - b'0') as i32)
    } else if v >= b'A' && v <= b'Z' {
        Some(14 + (v - b'A') as i32)
    } else {
        None
    }
}

fn c40_to_char(val: i32) -> Option<char> {
    if let Some(zero) = char_to_c40('0') {
        if val >= zero && val <= zero + 9 {
            return Some((b'0' + (val - zero) as u8) as char);
        }
    }
    if let Some(a) = char_to_c40('A') {
        if val >= a && val <= a + 25 {
            return Some((b'A' + (val - a) as u8) as char);
        }
    }
    None
}

fn decode_half_word(half_word: u16, c40_list: &mut Vec<i32>, index: &mut usize) {
    c40_list.push(((half_word - 1) / 1600) as i32);
    *index += 1;
    let mut hw = half_word - (c40_list[*index - 1] as u16 * 1600);
    c40_list.push(((hw - 1) / 40) as i32);
    *index += 1;
    hw -= (c40_list[*index - 1] as u16 * 40);
    c40_list.push((hw - 1) as i32);
    *index += 1;
}
