use miniserde::{Deserialize, Serialize, json};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Request(minreq::Error),
    Api { status: u16, body: String },
    Json(miniserde::Error),
    NoTextInResponse,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(_) => write!(f, "Failed to send request to the OpenAI-compatible API"),
            Self::Api { status, body } => {
                write!(f, "OpenAI-compatible API returned an error (status {status}): {body}")
            }
            Self::Json(_) => write!(f, "Failed to parse a response from the OpenAI-compatible API"),
            Self::NoTextInResponse => write!(f, "The API response did not contain any text"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Request(e) => Some(e),
            Self::Json(e) => Some(e),
            Self::Api { .. } | Self::NoTextInResponse => None,
        }
    }
}

// OpenAI-compatible response structures
#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Option<Message>,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

// OpenAI-compatible request structures
#[derive(Serialize)]
struct OpenAIChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    temperature: f32,
    stream: bool,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

/// Summarize using an OpenAI-compatible chat completions endpoint.
///
/// Compatible with:
/// - llama.cpp (default: http://localhost:8000)
/// - vllm (default: http://localhost:8000)
/// - Any server implementing the OpenAI `/v1/chat/completions` API
pub fn summarize(
    base_url: &str,
    api_key: Option<&str>,
    model: &str,
    system_prompt: &str,
    transcript: &str,
) -> Result<String, Error> {
    let req_url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));

    let request_body = OpenAIChatRequest {
        model,
        messages: vec![
            ChatMessage {
                role: "system",
                content: system_prompt,
            },
            ChatMessage {
                role: "user",
                content: transcript,
            },
        ],
        temperature: 0.1,
        stream: false,
    };

    let body_str = json::to_vec(&request_body);

    let mut request = minreq::post(&req_url)
        .with_timeout(300)
        .with_body(body_str);

    if let Some(key) = api_key {
        request = request.with_header("Authorization", &format!("Bearer {}", key));
    }

    let response = request.send().map_err(Error::Request)?;

    if !(200..=299).contains(&response.status_code) {
        let body = response.as_str().unwrap_or("No response body").to_string();
        return Err(Error::Api {
            status: response.status_code as u16,
            body,
        });
    }

    let reply: OpenAIResponse = json::from_slice(response.as_bytes()).map_err(Error::Json)?;

    reply
        .choices
        .first()
        .and_then(|c| c.message.as_ref())
        .map(|m| m.content.clone())
        .ok_or(Error::NoTextInResponse)
}
