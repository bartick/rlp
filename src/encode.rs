use crate::types::RLPItem;

fn encode_length(length: usize, offset: u8) -> Vec<u8> {
    let length_bytes = length.to_be_bytes();
    let mut length_bytes_trimmed = length_bytes
        .iter()
        .skip_while(|&&x| x == 0)
        .copied()
        .collect::<Vec<_>>();
    length_bytes_trimmed.insert(0, offset + length_bytes_trimmed.len() as u8);
    length_bytes_trimmed
}

fn encode_str(input: Vec<u8>) -> Vec<u8> {
    if input.len() == 1 && input[0] <= 0x7f {
        vec![input[0]]
    } else {
        let length = input.len();
        let mut output = Vec::new();

        if length < 56 {
            output.push(0x80 + length as u8);
        } else {
            output.extend(encode_length(length, 0xb7));
        }
        output.extend(input);
        output
    }
}

fn encode_list(list: Vec<RLPItem>) -> Vec<u8> {
    let inner: Vec<u8> = list.into_iter().flat_map(encode).collect();
    let length = inner.len();
    let mut output = Vec::new();

    if length < 56 {
        output.push(0xc0 + length as u8);
    } else {
        output.extend(encode_length(length, 0xf7));
    }
    output.extend(inner);
    output
}

pub fn encode(input: RLPItem) -> Vec<u8> {
    match input {
        RLPItem::Bytes(bytes) => encode_str(bytes),
        RLPItem::Str(text) => encode_str(text.into_bytes()),
        RLPItem::List(list) => encode_list(list),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input: RLPItem = vec!["hello".into(), "world".into()].into();

        let expected = vec![
            0xcc, 
            0x85, 
            b'h', b'e', b'l', b'l', b'o', 
            0x85, 
            b'w', b'o', b'r', b'l', b'd'
        ];

        assert_eq!(encode(input), expected);
    }
}
