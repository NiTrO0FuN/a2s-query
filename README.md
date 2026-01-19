# A2S Query

A Rust library and CLI tool for querying Valve Source Engine servers using the [A2S protocol](https://developer.valvesoftware.com/wiki/Server_queries).

![Code Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FNiTrO0FuN%2Fa2s-query%2Fmain%2FCargo.toml&query=%24.package.version&prefix=v&label=Version)
[![Build status](https://img.shields.io/github/actions/workflow/status/NiTrO0FuN/a2s-query/test.yml)](https://github.com/NiTrO0FuN/a2s-query/actions/workflows/test.yml)
![License](https://img.shields.io/github/license/NiTrO0FuN/a2s-query)

## Installation

### As a Library

Use `cargo add`:

```bash
cargo add --git https://github.com/NiTrO0FuN/a2s-query
```

Or manually add to your `Cargo.toml`:

```toml
[dependencies]
a2s_query = { git = "https://github.com/NiTrO0FuN/a2s-query" }
```

### Building the Binary

```bash
git clone https://github.com/NiTrO0FuN/a2s-query
cd a2s-query
cargo build --release
```

## Binary Usage

The `a2s-query` binary allows you to query Source servers from the command line and get the results in JSON format.

### Syntax

```bash
a2s-query --host <HOST> [--port <PORT>] <COMMAND>
```

**Arguments:**

- `--host <HOST>`: IP address or hostname of the Source server
- `--port <PORT>`: Port number (default: 27015)

**Commands:**

- `info`: Get server information (name, map, player count, etc.)
- `players`: Get list of connected players
- `rules`: Get server rules and configuration

### Examples

#### Get Server Info

```bash
a2s-query --host play.example.com info
```

Response:

```json
{
    "protocol": 17,
    "name": "Example Server",
    "map": "de_dust2",
    "folder": "csgo",
    "game": "Counter-Strike: Global Offensive",
    "app_id": 730,
    "players": 12,
    "max_players": 32,
    "bots": 0,
    "server_type": "Dedicated",
    "environment": "Linux",
    "password": false,
    "vac": true,
    "version": "2025.03.26"
}
```

## Library Usage

### Basic Example

```rust
use a2s_query::{A2S, errors::Error, info::Info, players::Player, rules::Rule};

fn main() -> Result<(), Error> {
    // Create a new A2S query instance
    let a2s = A2S::new("play.example.com:27015");

    // Query server information
    let info: Info = a2s.info()?;

    // Query player list
    let players: Vec<Player> = a2s.players()?;

    // Query server rules
    let rules: Vec<Rule> = a2s.rules()?;

    Ok(())
}
```
