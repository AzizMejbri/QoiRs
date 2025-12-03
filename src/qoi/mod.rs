pub mod cli;
pub mod decoder;
pub mod encoder;
pub mod types;
pub mod types16;

pub use cli::cli;
pub use decoder::{decode, decode_to_p6_8_bit};
pub use encoder::encode;
