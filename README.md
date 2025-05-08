# Coinbase API JWT Generator

This Rust program generates JSON Web Tokens (JWTs) for authenticating requests to the Coinbase API using an ECDSA private key.

## Overview

The program provides a `create_jwt` function to generate JWTs for Coinbase API authentication. It uses the ES256 algorithm and constructs a token with a custom payload including issuer, subject, nonce, and URI.

## Features

- Generates JWTs for Coinbase API authentication
- Uses PKCS#8 formatted ECDSA private keys
- Creates random nonces for each token
- Supports configurable HTTP methods and API paths

## Prerequisites

- Rust (stable)
- Cargo (Rust's package manager)

### Required Crates

- `jsonwebtoken`
- `rand`
- `hex`
- `serde_json`

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
jsonwebtoken = "8"
rand = "0.8"
hex = "0.4"
serde_json = "1"
```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/your-repo.git
   ```
2. Navigate to the project directory:
   ```bash
   cd your-repo
   ```
3. Build the project:
   ```bash
   cargo build
   ```

## Usage

The `create_jwt` function generates a JWT for a given HTTP method and API path:

```rust
fn create_jwt(method: &str, path: &str) -> Result<String, Box<dyn std::error::Error>>
```

### Example

```rust
fn main() {
    let method = "GET";
    let path = "/api/v3/brokerage/accounts";
    match create_jwt(method, path) {
        Ok(jwt) => println!("Generated JWT: {}", jwt),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Key Components

- **Private Key**: Must be in PKCS#8 format (starts with `-----BEGIN PRIVATE KEY-----`).
- **Key Name**: Format: `organizations/{org_id}/apiKeys/{key_id}`.
- **Nonce**: Randomly generated for each JWT.
- **URI**: Constructed as `{method} {host}{path}` (e.g., `GET api.coinbase.com/api/v3/brokerage/accounts`).
- **Payload**: Includes:
  - `iss`: Issuer ("cdp")
  - `nbf`: Not before (current timestamp)
  - `exp`: Expiration (current timestamp + 120 seconds)
  - `sub`: Subject (key name)
  - `uri`: Request URI
  - `nonce`: Random nonce

## Important Notes

### Private Key Format

The program expects an ECDSA private key in **PKCS#8** format, starting with:

```
-----BEGIN PRIVATE KEY-----
```

If your key starts with `-----BEGIN EC PRIVATE KEY-----`, you must convert it to PKCS#8 format. You can use OpenSSL to convert:

```bash
openssl pkcs8 -topk8 -nocrypt -in ec_private_key.pem -out private_key_pkcs8.pem
```

Then use the converted key in the `PRIVATE_KEY` constant.

### Coinbase API

- **Host**: Set to `api.coinbase.com`.
- Ensure the `KEY_NAME` and `PRIVATE_KEY` constants match your Coinbase API credentials.
- Refer to the [Coinbase API documentation](https://docs.cloud.coinbase.com/) for valid methods and paths.

## Running the Program

1. Update the `KEY_NAME` and `PRIVATE_KEY` constants with your Coinbase API credentials.
2. Run the program:
   ```bash
   cargo run
   ```

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for bugs, features, or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
