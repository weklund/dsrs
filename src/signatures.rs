// use serde::{Deserialize, Serialize};
// use std::error::Error as StdError;

// // Field metadata for input/output fields (like dspy.InputField/OutputField)
// #[derive(Clone, Debug)]
// pub struct FieldMeta {
//     pub desc: Option<&'static str>,
//     pub constraints: Option<Vec<&'static str>>, // For Literal-like enums
// }

// // Trait for DSPy-style signatures
// pub trait DSPySignature {
//     type Input: Serialize;  // Input data structure
//     type Output: for<'de> Deserialize<'de> + Clone;  // Output for parsing from LM

//     fn generate_prompt(&self, input: &Self::Input) -> String;
//     fn parse_output(&self, response: &str) -> Result<Self::Output, Box<dyn StdError>>;
// }
