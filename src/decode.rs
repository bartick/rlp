use crate::types::RLPItem;

fn decode_length(input: &[u8]) -> (usize, usize, u8) {
    let length = input.len();
    let prefix = input[0] as usize;

    if prefix <= 0x7f {
        return (0, 1, 1);
    }
    if prefix <= 0xb7 && length > prefix - 0x80 {
        let str_len = prefix - 0x80;
        return (1, str_len, 1);
    }
    if prefix <= 0xbf && length > prefix - 0xb7 {
        let len_of_str_len = prefix - 0xb7;
        if length > len_of_str_len {
            let str_len = input[1..len_of_str_len + 1]
                .iter()
                .fold(0, |acc, &x| (acc << 8) + x as usize);
            return (1 + len_of_str_len, str_len, 1);
        }
    }
    if prefix <= 0xf7 && length > prefix - 0xc0 {
        let list_len = prefix - 0xc0;
        return (1, list_len, 2);
    }
    if prefix <= 0xff && length > prefix - 0xf7 {
        let len_of_list_len = prefix - 0xf7;
        if length > len_of_list_len {
            let list_len = input[1..len_of_list_len + 1]
                .iter()
                .fold(0, |acc, &x| (acc << 8) + x as usize);
            return (1 + len_of_list_len, list_len, 2);
        }
    }

    (0, 0, 1)
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

fn decode_list(input: &[u8], offset: usize) -> Vec<RLPItem> {
    if offset >= input.len() {
        return Vec::new();
    }

    let (offset_inner, length, rlp_type) = decode_length(&input[offset..]);
    let item = if rlp_type == 1 {
        decode_string(&input[offset + offset_inner..offset + offset_inner + length])
    } else {
        decode_list(input, offset + offset_inner)
            .into_iter()
            .collect::<Vec<RLPItem>>()
            .into()
    };

    let mut rest = decode_list(input, offset + offset_inner + length);
    rest.insert(0, item);

    rest
}

pub fn decode(input: &[u8]) -> RLPItem {
    if input.is_empty() {
        return "".into();
    }

    let (offset, length, rlp_type) = decode_length(input);
    if rlp_type == 1 {
        decode_string(&input[offset..offset + length])
    } else {
        decode_list(input, offset)
            .into_iter()
            .collect::<Vec<RLPItem>>()
            .into()
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

        assert_eq!(decode(&input), expected);
    }
}