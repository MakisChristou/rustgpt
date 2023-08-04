use clap::Parser;

/// Yet another command-line chat GPT frontend written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {}
