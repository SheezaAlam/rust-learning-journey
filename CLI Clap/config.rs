
```
tokio_playground/
│
├── Cargo.toml
└── src/
    ├── main.rs
    └── config.rs   <-- our teaching file
```

---

###  `src/config.rs`

This file only cares about **config setup** (flags + defaults).

```rust
use clap::Parser;

/// CLI arguments (flags)
#[derive(Parser, Debug)]
pub struct Cli {
    /// IP address to bind (default: 127.0.0.1)
    #[arg(long, default_value = "127.0.0.1")]
    pub bind: String,

    /// Port to listen on (default: 7878)
    #[arg(long, default_value_t = 7878)]
    pub port: u16,
}

/// ServerConfig = final cleaned config
/// This is what the server will actually use.
#[derive(Debug)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
}

impl ServerConfig {
    /// Build config from CLI
    pub fn from_cli() -> Self {
        let cli = Cli::parse();
        ServerConfig {
            bind: cli.bind,
            port: cli.port,
        }
    }
}
```

---

###  `src/main.rs`

This is the entry point.
It **imports config**, builds it, and uses it in the server.

```rust
mod config; // import our config.rs

use config::ServerConfig;

fn main() {
    // Step 1: Load config from CLI
    let cfg = ServerConfig::from_cli();

    // Step 2: Print the final settings
    println!("Server starting on {}:{}", cfg.bind, cfg.port);

    // Step 3: Later, you’ll pass cfg.bind & cfg.port to your tokio server
}
```

---

### Usage Examples

```powershell
# With defaults
PS C:\tokio_playground> cargo run
Server starting on 127.0.0.1:7878

# With flags
PS C:\tokio_playground> cargo run -- --bind 192.168.1.10 --port 9000
Server starting on 192.168.1.10:9000
```

---

 **Teaching Notes**

* `Cli` = **raw user input** (comes from Clap).
* `ServerConfig` = **clean config struct** your server will use.
* `config.rs` keeps all config logic in **one place**.
* `main.rs` stays clean: it only starts the server.

