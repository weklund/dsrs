pub mod client; // LLMClient
pub mod errors; // DSRSError
pub mod modules;
pub mod signatures; // Signature trait and metas // Predict and other modules

pub use client::LLMClient;
pub use errors::DSRSError;
// pub use signatures::{DSPySignature, FieldMeta};
// pub use modules::Predict;
