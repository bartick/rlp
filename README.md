# Recursive Length Prefix (RLP) encoding

This is a simple implementation of the RLP encoding in Rust. I am writing this to understand the basics of rlp encoding and decoding. This is not a full implementation of the Ethereum RLP encoding.

## Usage

```rust
use rlp::{encode, decode, types::RLPItem};

fn main() {
    let data: RLPItem = vec![
        "hello".into(), 
        "world".into()
    ].into();
    let encoded = encode(&data);
    let decoded = decode(&encoded);
    assert_eq!(data, decoded);
}
```

I have taken the inspiration from: [https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/)