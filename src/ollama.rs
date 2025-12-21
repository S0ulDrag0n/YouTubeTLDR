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
            Self::Request(_) => write!(f, "Failed to send request to the Ollama API"),
            Self::Api { status, body } => {
                write!(f, "Ollama API returned an error (status {status}): {body}")
            }
            Self::Json(_) => write!(f, "Failed to parse a response from the Ollama API"),
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

#[derive(Deserialize)]
struct OllamaResponse {
    response: Option<String>,
    message: Option<Message>,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

#[derive(Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    system: &'a str,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    stream: bool,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

pub fn summarize(
    base_url: &str,
    api_key: Option<&str>,
    model: &str,
    system_prompt: &str,
    transcript: &str,
) -> Result<String, Error> {
    // Try chat endpoint first (newer API)
    match summarize_chat(base_url, api_key, model, system_prompt, transcript) {
        Ok(response) => Ok(response),
        Err(_) => {
            // Fallback to generate endpoint (older API)
            summarize_generate(base_url, api_key, model, system_prompt, transcript)
        }
    }
}

fn summarize_chat(
    base_url: &str,
    api_key: Option<&str>,
    model: &str,
    system_prompt: &str,
    transcript: &str,
) -> Result<String, Error> {
    let req_url = format!("{}/api/chat", base_url.trim_end_matches('/'));

    let request_body = OllamaChatRequest {
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
        stream: false,
    };

    let body_str = json::to_vec(&request_body);

    let mut request = minreq::post(req_url)
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

    let reply: OllamaResponse = json::from_slice(response.as_bytes()).map_err(Error::Json)?;

    reply
        .message
        .map(|m| m.content)
        .ok_or(Error::NoTextInResponse)
}

fn summarize_generate(
    base_url: &str,
    api_key: Option<&str>,
    model: &str,
    system_prompt: &str,
    transcript: &str,
) -> Result<String, Error> {
    let req_url = format!("{}/api/generate", base_url.trim_end_matches('/'));

    let request_body = OllamaGenerateRequest {
        model,
        prompt: transcript,
        system: system_prompt,
        stream: false,
    };

    let body_str = json::to_vec(&request_body);

    let mut request = minreq::post(req_url)
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

    let reply: OllamaResponse = json::from_slice(response.as_bytes()).map_err(Error::Json)?;

    reply.response.ok_or(Error::NoTextInResponse)
}
