# GPTerm
GPTerm is a yet another command-line chat GPT frontend written in Rust.

![demo](https://user-images.githubusercontent.com/39631552/232221182-d0d8409f-ff76-4bad-b909-77c9ff44740b.gif)


<!-- [![Example Usage](https://asciinema.org/a/VGv3l7UmZ1kiSQF1d5wwVAJvk.svg)](https://asciinema.org/a/VGv3l7UmZ1kiSQF1d5wwVAJvk) -->

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


## Configuration
GPTerm can be configured in various ways. See below the different config options it supports.

```
API_KEY=sk-...
CONTEXT=true/false
```

