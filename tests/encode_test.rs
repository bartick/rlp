use rlp::{encode, types::RLPItem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_dog() {
        let item: RLPItem = "dog".into();
        let encoded = encode(item);
        let expected = vec![0x83, b'd', b'o', b'g'];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_list_cat_dog() {
        let item: RLPItem = vec!["cat".into(), "dog".into()].into();
        let encoded = encode(item);
        let expected = vec![
            0xc8, // List prefix for length 8
            0x83, b'c', b'a', b't', // "cat"
            0x83, b'd', b'o', b'g', // "dog"
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_empty_string() {
        let item: RLPItem = "".into();
        let encoded = encode(item);
        let expected = vec![0x80];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_empty_list() {
        let item: RLPItem = vec![].into();
        let encoded = encode(item);
        let expected = vec![0xc0];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_zero() {
        let item: RLPItem = 0x00.into();
        let encoded = encode(item);
        let expected = vec![0x00];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_fifteen() {
        let item = 0x0f.into();
        let encoded = encode(item);
        let expected = vec![0x0f];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_1024() {
        let item: RLPItem = (&vec![0x04, 0x00][..]).into(); // 1024 in big-endian bytes
        let encoded = encode(item);
        let expected = vec![0x82, 0x04, 0x00];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_nested_lists() {
        // Represents [ [], [[]], [ [], [[]] ] ]
        let item = RLPItem::List(vec![
            RLPItem::List(vec![]),
            RLPItem::List(vec![
                RLPItem::List(vec![])
            ]),
            RLPItem::List(vec![
                RLPItem::List(vec![]),
                RLPItem::List(vec![
                    RLPItem::List(vec![])
                ]),
            ]),
        ]);
        let encoded = encode(item);
        let expected = vec![
            0xc7,
            0xc0,
            0xc1, 0xc0,
            0xc3, 0xc0, 0xc1, 0xc0,
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_long_string() {
        let s = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
        let item: RLPItem = s.into();
        let encoded = encode(item);
        let mut expected = vec![0xb8, 0x38]; // 0xb7 + length of length (1), length (56)
        expected.extend(s.bytes());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn bartick(){
        let input = RLPItem::List(vec![
            RLPItem::Str("hello".to_string()),
            RLPItem::Str("world".to_string()),
        ]);
        let expected = vec![
            0xcc,                   // List prefix: 0xc0 + length (12)
            0x85,                   // String prefix: 0x80 + length (5)
            b'h', b'e', b'l', b'l', b'o', // "hello"
            0x85,                   // String prefix: 0x80 + length (5)
            b'w', b'o', b'r', b'l', b'd', // "world"
        ];

        // Encode the input using the RLP encoding function.
        let encoded = encode(input);

        // Assert that the encoded output matches the expected output.
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_hello_world_list() {
        let input: RLPItem = vec!["hello".into(), "world".into()].into();

        let expected = vec![
            0xcc,                   
            0x85,                  
            b'h', b'e', b'l', b'l', b'o', 
            0x85,                   
            b'w', b'o', b'r', b'l', b'd', 
        ];
        let encoded = encode(input);
        assert_eq!(encoded, expected);
    }
}