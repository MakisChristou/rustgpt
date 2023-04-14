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


#[derive(Debug, Deserialize)]
struct GptError {
    error: GptErrorResponse,
}

#[derive(Debug, Deserialize)]
struct GptErrorResponse {
    message: Option<String>,
    #[serde(rename = "type")]
    error_type: Option<String>,
    param: Option<String>,
    code: Option<String>,
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
                let mut split = buffer.split("data:");
                let mut remaining = String::new();

                for mut line in split {
                    let cleaned_line = line.trim();

                    if !cleaned_line.is_empty() {
                        if cleaned_line == "[DONE]" {
                            println!("");
                            break;
                        }
                        match serde_json::from_str::<Value>(&cleaned_line) {
                            Ok(json) => {
                                let gpt_response = serde_json::from_value::<GptResponse>(json.clone());//.expect("Failed to decode GptResponse");
                                
                                match gpt_response {
                                    Ok(gpt_response) => {
                                        let msg = &gpt_response.choices[0].delta.content;
                                        match  msg{
                                            Some(msg) => print_as_typing(msg, typing_delay).await,
                                            None => (),
                                        }
                                    },
                                    Err(e) => {
                                        // println!("Cannot decode to GptResponse {}", e);
                                        let gpt_error = serde_json::from_value::<GptError>(json);
                                        match gpt_error {
                                            Ok(gpt_error) => {
                                                match gpt_error.error.error_type {
                                                    Some(err_type) => println!("{}",err_type),
                                                    None => println!("An error occured"),
                                                }
                                                exit(0);
                                            },
                                            Err(e) => {
                                                println!("Cannot decode to GptError {:?}",e);
                                                exit(1);
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error reading JSON: {}", e);
                                eprintln!("Response: {}", line);
                                exit(1);
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
    let api_key = "";

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
