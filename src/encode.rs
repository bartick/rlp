use crate::types::RLPItem;

fn encode_length(length: usize, offset: u8) -> Vec<u8> {
    let length_bytes = length.to_be_bytes();
    let mut output = vec![offset + length_bytes.len() as u8];
    output.extend(&length_bytes);
    output
}

fn rlp_encode(input: RLPItem) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    match input {
        RLPItem::Str(text) => {
            let text = text.as_bytes();
            if text.len() == 1 && text[0] <= 0x7f {
                output.push(text[0]);
            } else {
                let length = text.len();
                if length < 56 {
                    output.push(0x80 + length as u8);
                } else {
                    output.extend(encode_length(length, 0xb7));
                }
                output.extend(text);
            }
        }
        RLPItem::List(list) => {
            let inner = encode(list);
            let length = inner.len();
            if length < 56 {
                output.push(0xc0 + length as u8);
            } else {
                output.extend(encode_length(length, 0xf7));
            }
        }
    }

    output
}

pub fn encode(input: Vec<RLPItem>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for item in input {
        output.extend(rlp_encode(item));
    }

    // Add the Length Prefix
    let length = output.len();
    if length < 56 {
        output.insert(0, 0xc0 + length as u8);
    } else {
        output = encode_length(length, 0xf7);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input: Vec<RLPItem> = vec!["hello".into(), "world".into()];

        let expected = vec![
            0xcc, 
            0x85, 
            b'h', b'e', b'l', b'l', b'o', 
            0x85, 
            b'w', b'o', b'r', b'l', b'd'];

        assert_eq!(encode(input), expected);
    }
}
