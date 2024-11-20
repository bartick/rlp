use rlp::{decode, types::RLPItem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_dog() {
        let item = vec![0x83, b'd', b'o', b'g'];
        let encoded = decode(&item);
        let expected: RLPItem = "dog".into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_list_cat_dog() {
        let item = vec![
            0xc8, // List prefix for length 8
            0x83, b'c', b'a', b't', // "cat"
            0x83, b'd', b'o', b'g', // "dog"
        ];
        let encoded = decode(&item);
        let expected: RLPItem = vec!["cat".into(), "dog".into()].into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_empty_string() {
        let item = vec![0x80];
        let encoded = decode(&item);
        let expected: RLPItem = "".into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_empty_list() {
        let item = vec![0xc0];
        let encoded = decode(&item);
        let expected: RLPItem = vec![].into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_zero() {
        let item = vec![0x00];
        let encoded = decode(&item);
        let expected: RLPItem = 0x00.into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_fifteen() {
        let item = vec![0x0f];
        let encoded = decode(&item);
        let expected: RLPItem = 0x0f.into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_1024() {
        let item = vec![0x82, 0x04, 0x00];
        let encoded = decode(&item);
        // Convert 1024 to bytes in big-endian
        let expected: RLPItem = (&1024u16.to_be_bytes()[..]).into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_1024_list() {
        let item = vec![
            0xc9, // List prefix for length 9
            0x82, 0x04, 0x00, // 1024
            0x83, b'c', b'a', b't', // "cat"
        ];
        let encoded = decode(&item);
        let expected: RLPItem = vec![(&1024u16.to_be_bytes()[..]).into(), "cat".into()].into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_1024_list_nested() {
        let item = vec![
            0xca, // List prefix for length 10
            0x82, 0x04, 0x00, // 1024
            0x83, b'c', b'a', b't', // "cat"
            0xc0, // Empty list
        ];
        let encoded = decode(&item);
        let expected: RLPItem = vec![(&1024u16.to_be_bytes()[..]).into(), "cat".into(), vec![].into()].into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encoded_integer_1024_list_nested_nested() {
        let item = vec![
            0xcb, // List prefix for length 11
            0x82, 0x04, 0x00, // 1024
            0x83, b'c', b'a', b't', // "cat"
            0xc0, // Empty list
            0xc0, // Empty list
        ];
        let encoded = decode(&item);
        let expected: RLPItem = vec![(&1024u16.to_be_bytes()[..]).into(), "cat".into(), vec![].into(), vec![].into()].into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_nested_lists() {
        let item = vec![
            0xcc, // List prefix for length 12
            0xc8, // List prefix for length 8
            0x83, b'c', b'a', b't', // "cat"
            0x83, b'd', b'o', b'g', // "dog"
            0xc0, // Empty list
        ];
        let encoded = decode(&item);
        let expected: RLPItem = vec![vec!["cat".into(), "dog".into()].into(), vec![].into()].into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_long_string() {
        let item = vec![
            0xb8, 0x22,
            b't', b'h', b'i', b's', b' ', b'i', b's', b' ', b'a', b' ', b'l', b'o', b'n', b'g', b' ', b's', b't', b'r', b'i', b'n', b'g', b' ', b't', b'o', b' ', b't', b'e', b's', b't', b' ', b'h', b'e', b'r', b'e',
        ];
        let encoded = decode(&item);
        let expected: RLPItem = "this is a long string to test here".into();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_long_list() {
        let item = vec![
            0xf8, 0x22,
            0x83, b'c', b'a', b't', // "cat"
            0x83, b'd', b'o', b'g', // "dog"
            0xc0, // Empty list
        ];
        let encoded = decode(&item);
        let expected: RLPItem = vec!["cat".into(), "dog".into(), vec![].into()].into();
        assert_eq!(encoded, expected);
    }
}