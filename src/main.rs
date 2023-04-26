use chrono::Local;
use dotenv::dotenv;
use reedline::{FileBackedHistory,Reedline};
use std::env;

use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

mod client;
mod utils;
mod validator;

use crate::client::{send_gpt_request, Message};
use crate::utils::{get_log_directory, get_user_input, save_conversation_log};
use crate::validator::ReplValidator;

async fn start_chat_loop(
    api_key: &str,
    typing_delay: Duration,
    running: Arc<AtomicBool>,
    context_mode: bool,
    store_messages: bool,
) {
    let mut messages: Vec<Message> = Vec::new();

    let log_dir = match get_log_directory() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if store_messages {
        println!("Storing conversations in {:?}", log_dir.display());
    }

    if context_mode {
        println!("Using context mode");
    }

    println!("Welcome to gpterm!");

    let mut got_ctrl_c = false;
    let history = Box::new(
        FileBackedHistory::with_file(1_000_000, log_dir.join("history.txt"))
            .expect("Error configuring history with file"),
    );
    let mut line_editor = Reedline::create()
        .with_history(history)
        .with_quick_completions(true)
        .with_partial_completions(true)
        .with_validator(Box::new(ReplValidator)); // if you want your prompt to support multiline mode it has to add the validator

    loop {
        let mut assistant_response = String::from("");
        let user_input = get_user_input(&mut line_editor);

        if user_input == None {
            if got_ctrl_c {
                println!("Goodbye!");
                break;
            }
            got_ctrl_c = true;
            println!("Click CTRL + C again to exit");
            continue;
        }

        let input = user_input.unwrap();

        // Remove history if in no-context mode
        if !context_mode {
            messages.clear();
        }

        // Keep some history if in context mode
        // This needs to be improved based on the tokens a model can handle
        if messages.len() > 10 {
            messages.remove(0);
            messages.remove(0);
        }

        if store_messages {
            if let Err(e) = save_conversation_log(
                &log_dir,
                "history",
                &format!(
                    "{} user: {}\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    input,
                ),
            ) {
                eprintln!("Error saving conversation log: {}", e);
            }
        }

        messages.push(Message {
            role: String::from("user"),
            content: input,
        });

        // Send user input to OpenAPI Backend
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
                "history",
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
        Err(_) => panic!("API_KEY must be set, create a .env file and set API_KEY=<your_key>"),
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

    start_chat_loop(
        &api_key,
        typing_delay,
        running,
        context_mode,
        store_messages,
    )
    .await;
}
