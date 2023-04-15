# gpterm
Yet another command-line chat GPT frontend written in Rust.

![Example Usage](https://user-images.githubusercontent.com/39631552/232221182-d0d8409f-ff76-4bad-b909-77c9ff44740b.gif)


<!-- [![Example Usage](https://asciinema.org/a/VGv3l7UmZ1kiSQF1d5wwVAJvk.svg)](https://asciinema.org/a/VGv3l7UmZ1kiSQF1d5wwVAJvk) -->

## Build
1. Clone the repository to your local machine.
2. Navigate to the directory where the repository was cloned.
3. Run the following command to build the application:

```bash
$ cargo build --release
```

Quickly run 
```bash
$ cargo run
```

## Basic Usage
To get the most basic of setups up and running you need to create a `.env` file and populate it with your api key

```bash
API_KEY=sk-...
```

To exit the program simply Ctrl + C twice.


## Configuration Options

- API_KEY: (Mandatory) Set this to your chat gpt api key
- CONTEX:  (Optional) Set this to `true` if you want to keep context in your conversation. Default `false`.
- HISTORY: (Optional) Set this to `true` if you want to store your chat history. Default `false`.

