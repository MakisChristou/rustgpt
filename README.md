# GPTerm
GPTerm is a yet another command-line chat GPT frontend written in Rust.

[![Example Usage](https://asciinema.org/a/VGv3l7UmZ1kiSQF1d5wwVAJvk.svg)](https://asciinema.org/a/VGv3l7UmZ1kiSQF1d5wwVAJvk)

## Installation
1. Clone the repository to your local machine.
2. Navigate to the directory where the repository was cloned.
3. Run the following command to build the application:

```bash
$ cargo build --release
```
## Usage
To get the most basic of setups up and running you need to create a `.env` file and populate it with your api key

```bash
API_KEY=sk-...
```

Then run the GPTerm application using the following command:

```bash
$ cargo run
```

To exit the program simply Ctrl + C twice.

