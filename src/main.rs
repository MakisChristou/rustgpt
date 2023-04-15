use std::env;
use std::io::Read;
use std::io::{self, Write};
use std::process::exit;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use dotenv::dotenv;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

mod client;

use crate::client::{send_gpt_request, Message};

async fn start_chat_loop(api_key: &str, typing_delay: Duration, running: Arc<AtomicBool>) {
    println!("Welcome to gpterm!");

    let mut messages: Vec<Message> = Vec::new();
    
    loop {
        let mut assistant_response = String::from("");

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

        messages.push(Message { role: String::from("user"), content: input });

        send_gpt_request(messages.clone(), api_key, API_URL, typing_delay, &running, &mut assistant_response).await;

        messages.push(Message { role: String::from("assistant"), content: assistant_response })

    }
}



#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = match env::var("API_KEY") {
        Ok(value) => value,
        Err(_) => panic!("API_KEY must be set"),
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
    }).expect("Error setting Ctrl+C handler");


    start_chat_loop(&api_key, typing_delay, running).await ;
}
