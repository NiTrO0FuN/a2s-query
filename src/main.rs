use anyhow;
use clap::{Parser, Subcommand};
use std::collections::HashMap;

use a2s_query::A2S;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// IP address or host name of the source server
    #[arg(long)]
    host: String,

    /// Port used by the source server
    #[arg(long, default_value_t = 27015)]
    port: u16,

    #[command(subcommand)]
    request: A2SRequest,
}

#[derive(Subcommand)]
enum A2SRequest {
    Info,
    Players,
    Rules,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let Args {
        host,
        port,
        request,
    } = args;

    let address = format!("{host}:{port}");

    let a2s = A2S::new(address);

    let data_json = match request {
        A2SRequest::Info => serde_json::to_string_pretty(&a2s.info()?)?,
        A2SRequest::Players => serde_json::to_string_pretty(&a2s.players()?)?,
        A2SRequest::Rules => {
            let rules: HashMap<_, _> = a2s
                .rules()?
                .iter()
                .map(|r| (r.name.to_string(), r.value.to_string()))
                .collect();
            serde_json::to_string_pretty(&rules)?
        }
    };

    println!("{}", data_json);

    Ok(())
}
