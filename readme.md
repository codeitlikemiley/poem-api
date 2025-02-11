# Rust - Poem Open API Starter Server

Build your server with Rust using [poem](https://docs.rs/poem-openapi/5.1.5/poem_openapi/) framework. 

## Requirements

To build and run this project, you need the following:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)

The project uses a specific version of Rust, which is managed automatically using [rustup](https://rustup.rs/). Check the `rust-toolchain.toml` file for the exact version.

## Installation

### Prerequisites

1. **Install Rust**: If you don't have Rust installed, you can install it with the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the Repository**:

   ```bash
   git clone https://github.com/codeitlikemiley/poem-api.git
   cd poem-api
   ```

3. **Set the Rust Toolchain**:

   ```bash
   rustup update
   ```

### Development

1. **Cargo Watch** with `cargo-watch`

   - Install cargo-watch

   ```sh
   cargo install cargo-watch
   ```

   - Run Command:

   ```bash
   cargo watch -x run
   ```

   Note: This auto re-compiles the executable and re-run the server

### Running The Server

1. **Running the Server**:

   ```bash
   cargo run
   ```

   > By Default `cargo run` uses `.env.example` ENV Variable

   ```env
   PORT=3000
   HOST=127.0.0.1
   APP_SECRET=secret 
   ```


   > NOTE: If you need to use other `PORT` you can pass in it as an ENV Variable

   ```bash
   PORT=8000 cargo run
   ```

### Running in Production

1. Build Release

   ```bash
   cargo build --release
   ```
2. Run in Production

   ```bash
   APP_ENV=prod PORT=3001 APP_SECRET=SomeRandomString HOST=localhost ./target/release/poem-api
   ```


### Authorizing with JWT on Swagger UI

1. **Open Swagger UI and Test the API**:

   ```bash
   open http://localhost:3000/docs
   ```

2. **Login with username**


```json
{
    "username": "codeitlikemiley@gmail.com"
}
````

<img width="1512" alt="Image" src="https://github.com/user-attachments/assets/5521c2c4-a286-4254-aaa4-3344c4313b8f" />

3. Authorization

 
> Copy Paste the **token** response from **`Login`** 

Then Go to  any Locked Endpoint with a lock Icon 

Click the Lock Icon , it would show a Modal , where you can paste the **token**


<img width="1512" alt="Image" src="https://github.com/user-attachments/assets/9a1fc09d-dcb1-4133-a026-dedc4aa81c29" />


