use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ApiMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: ApiMessage,
}

#[derive(Deserialize, Debug)]
pub struct APIError {
    pub error: ErrorDetail,
}

#[derive(Deserialize, Debug)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}
