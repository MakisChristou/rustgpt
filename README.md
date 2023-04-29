# gpterm
Yet another command-line chat GPT frontend written in Rust.

![Example Usage](https://user-images.githubusercontent.com/39631552/232221182-d0d8409f-ff76-4bad-b909-77c9ff44740b.gif)

## Features
* Stream output with typing effect
* Store chat messages/history
* Context aware conversation
* Simple and minimal
* Multiline mode (use `{}` to enter multiline mode)

## Prompt Shortcuts
* Search History `CTRL + R`
* Clear Prompt `CTRL + L`
* Exit `CTRL + C` twice

## Future Features
* ~~Multiline input~~
* Syntax highlighting
* Choose model/role
* Add Jailbreaks

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

## Getting Started
To get the most basic of setups up and running you need to create a `.env` file and populate it with your api key

```bash
touch .env
```

Edit your `.env` file by adding your api key.

```bash
API_KEY=sk-...
```

To exit the program simply Ctrl + C twice.


## Configuration Options

- `API_KEY`: Set this to your chat gpt api key
- `CONTEXT`:  (Optional) Set this to `true` if you want to keep context in your conversation. Default `false`.
- `HISTORY`: (Optional) Set this to `true` if you want to store your chat history. Default `false`.
- `MODEL`: (Optional) Set this to any of the models supported by OpenAI (`gpt-4`, `gpt-4-0314`, `gpt-4-32k`, `gpt-4-32k-0314`, `gpt-3.5-turbo`, `gpt-3.5-turbo-0301`) Default `gpt-3.5-turbo`
- `TYPING_DELAY`: (Optional) Set the delay between each letter in milliseconds (Default `10`)
