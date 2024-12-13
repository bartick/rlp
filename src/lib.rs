// This is a library file that contains the RLP encoding implementation.

mod encode;
mod decode;
pub mod types;

pub use encode::encode;
pub use decode::decode;
