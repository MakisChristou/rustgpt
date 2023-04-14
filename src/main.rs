use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Deserializer, StreamDeserializer, Value};
use tokio::time::sleep;
use std::io::Read;
use std::io::{self, Write};
use std::process::exit;
use std::time::Duration;
use tokio::runtime::Runtime;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";


async fn print_as_typing(s: &str, delay: Duration) {
    for c in s.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap();
        sleep(delay).await;
    }
}

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
    let mut buffer = String::new();

    let typing_delay = Duration::from_millis(10);

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                buffer.push_str(&String::from_utf8_lossy(&bytes));
                let mut split = buffer.split('\n');
                let mut remaining = String::new();

                for line in split {
                    if !line.is_empty() {
                        
                        let line = &line[6..];

                        if line == "[DONE]" {
                            println!("");
                            break;
                        }
                        match serde_json::from_str::<Value>(&line) {
                            Ok(json) => {
                                let gpt_response = serde_json::from_value::<GptResponse>(json).expect("Failed to decode GptResponse");
                                // println!("{:?}", gpt_response.choices[0].delta.content);
                                let msg = &gpt_response.choices[0].delta.content;
                                match  msg{
                                    Some(msg) => print_as_typing(msg, typing_delay).await,
                                    None => (),
                                }
                            }
                            Err(e) => {
                                eprintln!("Error reading JSON: {}", e);
                            }
                        }
                    } else {
                        remaining.push_str(line);
                    }
                }

                buffer = remaining;
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
