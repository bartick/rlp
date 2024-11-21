use crate::types::RLPItem;

fn handle_empty(input: u8) -> RLPItem{
    if input == 0x80 {
        return "".into()
    } 
    
    vec![].into()
}

fn decode_length(input: &[u8]) -> (usize, usize, u8) {
    if input.is_empty() {
        panic!("RLP decoding error: Input is empty");
    }

    let length = input.len();
    let prefix = input[0] as usize;

    if prefix <= 0x7f {
        // Single byte, value is the byte itself
        return (0, 1, 1);
    }

    if prefix == 0x80 {
        // Empty string
        return (0, 1, 3);
    }

    if prefix == 0xc0 {
        // Empty list
        return (0, 1, 3);
    }

    if prefix <= 0xb7 {
        // Short string
        let str_len = prefix - 0x80;
        if length >= 1 + str_len {
            return (1, str_len, 1);
        } else {
            panic!("RLP decoding error: Insufficient input length for short string");
        }
    }

    if prefix <= 0xbf {
        // Long string
        let len_of_str_len = prefix - 0xb7;
        if length > len_of_str_len {
            let str_len = input[1..1 + len_of_str_len]
                .iter()
                .fold(0, |acc, &x| (acc << 8) + x as usize);
            if length >= 1 + len_of_str_len + str_len {
                return (1 + len_of_str_len, str_len, 1);
            } else {
                panic!("RLP decoding error: Insufficient input length for long string");
            }
        } else {
            panic!("RLP decoding error: Insufficient input length for long string prefix");
        }
    }

    if prefix <= 0xf7 {
        // Short list
        let list_len = prefix - 0xc0;
        if length >= 1 + list_len {
            return (1, list_len, 2);
        } else {
            panic!("RLP decoding error: Insufficient input length for short list");
        }
    }

    if prefix <= 0xff {
        // Long list
        let len_of_list_len = prefix - 0xf7;
        if length > len_of_list_len {
            let list_len = input[1..1 + len_of_list_len]
                .iter()
                .fold(0, |acc, &x| (acc << 8) + x as usize);
            if length >= 1 + len_of_list_len + list_len {
                return (1 + len_of_list_len, list_len, 2);
            } else {
                panic!("RLP decoding error: Insufficient input length for long list");
            }
        } else {
            panic!("RLP decoding error: Insufficient input length for long list prefix");
        }
    }

    panic!("RLP decoding error: Invalid prefix");
}

fn decode_string(input: &[u8]) -> RLPItem {
    match String::from_utf8(input.to_vec()) {
        Ok(s) => {
            // Check if all characters in the string are printable
            if s.chars().all(|c| !c.is_control()) {
                RLPItem::Str(s)
            } else {
                RLPItem::Bytes(input.to_vec())
            }
        }
        Err(_) => RLPItem::Bytes(input.to_vec()),
    }
}

fn decode_list(input: &[u8]) -> RLPItem {
    let mut list = Vec::new();
    let mut i = 0;

    while i < input.len() {

        let (offset, length, rlp_type) = decode_length(&input[i..]);
        i += offset;
        if rlp_type == 1 {
            list.push(decode_string(&input[i..i + length]));
        } else if rlp_type == 2 {
            list.push(decode_list(&input[i..i + length]));
        } else {
            list.push(handle_empty(input[i]));
        }

        i +=  length;
    }

    list.into()
}

pub fn decode(input: Vec<u8>) -> RLPItem {

    if input.is_empty() {
        return vec![].into();
    }

    let (offset, length, rlp_type) = decode_length(&input);
    if rlp_type == 1 {
        return decode_string(&input[offset..offset + length]);
    } else if rlp_type == 2 {
        return decode_list(&input[offset..offset + length]);
    } else {
        return handle_empty(input[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let input = vec![
            0xcc, 
            0x85, 
            b'h', b'e', b'l', b'l', b'o', 
            0x85, 
            b'w', b'o', b'r', b'l', b'd'
        ];

        let expected: RLPItem = vec!["hello".into(), "world".into()].into();

        assert_eq!(decode(input), expected);
    }
}