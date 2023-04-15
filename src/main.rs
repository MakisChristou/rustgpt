use chrono::{DateTime, Local};
use dotenv::dotenv;
use std::env;
use std::io::Read;
use std::io::{self, Write};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

mod client;
mod utils;

use crate::client::{send_gpt_request, Message};
use crate::utils::{get_log_directory, save_conversation_log};

async fn start_chat_loop(
    api_key: &str,
    typing_delay: Duration,
    running: Arc<AtomicBool>,
    context_mode: bool,
    store_messages: bool,
) {
    println!("Welcome to gpterm!");

    let mut messages: Vec<Message> = Vec::new();

    let log_dir = match get_log_directory() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let conversation_id = "gpterm";

    if store_messages {
        println!("Storing conversations in {:?}", log_dir.to_str());
    }

    if context_mode {
        println!("Using context mode");
    }

    loop {
        let mut assistant_response = String::from("");

        print!("\n〉");
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

        // Remove history if in no-context mode
        if !context_mode {
            messages.clear();
        }

        // Keep some history if in context mode
        if messages.len() > 10 {
            messages.remove(0);
            messages.remove(0);
        }

        if store_messages {
            if let Err(e) = save_conversation_log(
                &log_dir,
                conversation_id,
                &format!(
                    "{} user: {}\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    input
                ),
            ) {
                eprintln!("Error saving conversation log: {}", e);
            }
        }

        messages.push(Message {
            role: String::from("user"),
            content: input,
        });
        send_gpt_request(
            messages.clone(),
            api_key,
            API_URL,
            typing_delay,
            &running,
            &mut assistant_response,
        )
        .await;

        if store_messages {
            if let Err(e) = save_conversation_log(
                &log_dir,
                conversation_id,
                &format!(
                    "{} assistant: {}\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    assistant_response
                ),
            ) {
                eprintln!("Error saving conversation log: {}", e);
            }
        }

        messages.push(Message {
            role: String::from("assistant"),
            content: assistant_response,
        });
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = match env::var("API_KEY") {
        Ok(value) => value,
        Err(_) => panic!("API_KEY must be set"),
    };

    let context_mode = match env::var("CONTEXT") {
        Ok(value) => {
            if value == String::from("true") {
                true
            } else if value == String::from("false") {
                false
            } else {
                panic!("Invalid context option");
            }
        }
        Err(_) => false,
    };

    let store_messages = match env::var("HISTORY") {
        Ok(value) => {
            if value == String::from("true") {
                true
            } else if value == String::from("false") {
                false
            } else {
                panic!("Invalid history option");
            }
        }
        Err(_) => false,
    };

    let typing_delay = Duration::from_millis(10);

    // Set up the signal handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        if !r.load(Ordering::SeqCst) {
            exit(0);
        }

        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    start_chat_loop(&api_key, typing_delay, running, context_mode, store_messages).await;
}
