// use crate::client::LLMClient;
// use crate::errors::DSRSError;
// use crate::signatures::DSPySignature;

// // Basic Predict module
// pub struct Predict<S: DSPySignature> {
//     signature: S,
// }

// impl<S: DSPySignature> Predict<S> {
//     pub fn new(signature: S) -> Self {
//         Self { signature }
//     }

//     pub async fn forward(
//         &self,
//         client: &LLMClient,
//         input: S::Input,
//     ) -> Result<S::Output, DSRSError> {
//         let prompt = self.signature.generate_prompt(&input);
//         let response = client
//             .complete(&prompt, "gpt-3.5-turbo", Some(1000), None)
//             .await?;
//         self.signature
//             .parse_output(&response)
//             .map_err(|e| DSRSError::ApiError(e.to_string()))
//     }
// }
