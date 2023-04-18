use directories::ProjectDirs;
use std::borrow::Cow;
use std::io::{self};
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};
use reedline::{DefaultPrompt, Reedline, Signal, Prompt, PromptHistorySearch, PromptEditMode};

struct MyPrompt {}

impl Prompt for MyPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Borrowed("〉")
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _prompt_mode: PromptEditMode) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> Cow<str> {
        Cow::Borrowed("")
    }
}


pub fn get_user_input(line_editor: &mut Reedline) -> Option<String> {

    let prompt = MyPrompt{};

    loop {
        let sig: Result<Signal, io::Error> = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                return Some(buffer);
            }
            Ok(reedline::Signal::CtrlC) | Ok(reedline::Signal::CtrlD) => {
                return None;
            },
            Err(e) => panic!("An error occured: {}", e),
        }
    }
}

pub fn save_conversation_log(
    log_dir: &PathBuf,
    conversation_id: &str,
    content: &str,
) -> Result<(), std::io::Error> {
    let log_file_path = log_dir.join(format!("{}.log", conversation_id));
    let mut log_file = OpenOptions::new()
        .append(true) // Set to append mode
        .create(true) // Create the file if it doesn't exist
        .open(log_file_path)?;
    log_file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn get_log_directory() -> Result<PathBuf, std::io::Error> {
    let project_dirs = ProjectDirs::from("com", "makischristou", "gpterm")
        .expect("Unable to determine log directory");

    let log_dir = project_dirs.data_local_dir().join("logs");
    fs::create_dir_all(&log_dir)?;

    Ok(log_dir)
}
