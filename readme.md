# Rust API Server 

> Built with Poem Framework Deployable via Shuttle

Build your server with Rust using [poem](https://docs.rs/poem-openapi/5.1.5/poem_openapi/) framework. 


## Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/codeitlikemiley/poem-api.git
   cd poem-api
   ```

2. **Set the Rust Toolchain**:

   ```bash
   rustup update
   ```

3. **Install [Shuttle CLI](https://docs.shuttle.dev/getting-started/installation)**

   ```bash
   cargo install cargo-shuttle
   ```

###  Development


1. **Set up ENV**

Go to your project root directory

```sh
touch Secrets.toml
```

Set `Secrets.toml`

```toml
APP_SECRET="secret"
RUST_LOG="poem=debug"
```

2. **Running Locally:**

   ```bash
   shuttle run
   ```


### Production

1. Deploy To Shuttle

   ```bash
   shuttle deploy
   ```
