use derive_more::derive::From;
use serde::Serialize;

#[derive(Debug, Serialize, From)]
pub enum TextDecompressionError {
    Custom(String),
}
