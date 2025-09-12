#  Rust Tracing 

This repo teaches `tracing` step by step.

## Why `tracing`?
Rust protocols, servers, and async apps fail in subtle ways:
- connections drop
- packets malformed
- retries & timeouts

Without structured logs, debugging = pain.  
`tracing` gives you **timestamps, levels, and spans**.



## Examples
Run them with Cargo:


# Basic difference from println
cargo run --quiet

# Show all levels
cargo run --quiet --example levels

# Show spans
cargo run --quiet --example spans

# Log levels

info! →  normal updates

warn! →  caution

error! →  problem

debug! →  developer details

trace! → very low-level
