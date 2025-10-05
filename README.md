# AiCommit

This repo is a learning project to get familiar with Rust.
Any similarities to existing projects are purely intentional.

## Features
* The tool is only usable with DeepSeek AI
* It does look into the environment variables for the API key named `DEEPSEEK_API_KEY`
* It can be used to generate commit messages based on the diff of the staged files

## Development
You need to have Rust installed. You can get it from [here](https://www.rust-lang.org/tools/install).

1. Clone the repository:
   ```bash
   git clone git@github.com:Katalam/aicommit.git
   ```
2. Navigate to the project directory:
   ```bash
    cd aicommit
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the project:
   ```bash
    cargo run --package aicommit --bin aicommit
    ```