use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::Read;
use std::io::{self, Write};
use std::process::exit;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Deserialize, Serialize)]
struct GptRequest {
    stream: bool,
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct GptResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Choice {
    delta: Delta,
    index: usize,
    finish_reason: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Delta {
    content: Option<String>,
    role: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

async fn send_gpt_request(prompt: &str, api_key: &str) {
    let client = Client::new();
    let url = API_URL;

    let gpt_request = GptRequest {
        stream: true,
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        temperature: 0.7,
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&gpt_request)
        .send()
        .await
        .expect("Failed to get response");

    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                let response_text = String::from_utf8_lossy(&bytes).to_string();

                println!("Hello: ({})", response_text);
                let gpt_response = serde_json::from_str::<GptResponse>(&response_text);
                println!("Debug: {:?}", gpt_response.unwrap());
                
            }
            Err(e) => {
                eprintln!("Error reading chunk: {}", e);
                break;
            }
        }
    }

    // let response_text = response.text().await.unwrap();

    // println!("response_text: {}", &response_text);

    // let gpt_response = serde_json::from_str::<GptResponse>(&response_text);

    // println!("Debug: {:?}", gpt_response);
}

#[tokio::main]
async fn main() {
    let api_key = "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";

    loop {
        print!("〉");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let mut buffer = [0; 1];
        loop {
            io::stdin().read_exact(&mut buffer).unwrap();
            let c = buffer[0] as char;
            if c == '\n' {
                break;
            }
            input.push(c);
        }

        input = input.trim().to_string();

        if input.to_lowercase() == "exit" {
            break;
        }

        send_gpt_request(&input, api_key).await;
    }
}
