use directories::ProjectDirs;
use std::io::Read;
use std::io::{self};
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

pub fn get_user_input() -> String {
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

    input.trim().to_string()
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
