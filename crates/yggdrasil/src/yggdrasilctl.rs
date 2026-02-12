use getopts::Options;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optopt("e", "endpoint", "Admin socket address (default: tcp://localhost:9001)", "URI");
    opts.optflag("j", "json", "Output as raw JSON");
    opts.optflag("h", "help", "Print this help");
    opts.optflag("v", "version", "Print version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("{}", opts.usage("Usage: yggdrasilctl [options] <command> [key=value ...]"));
            std::process::exit(1);
        }
    };

    if matches.opt_present("help") {
        println!("{}", opts.usage("Usage: yggdrasilctl [options] <command> [key=value ...]"));
        println!("Commands: list, getSelf, getPeers, getTree, addPeer, removePeer");
        return Ok(());
    }

    if matches.opt_present("version") {
        println!("yggdrasilctl {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let endpoint = matches.opt_str("endpoint").unwrap_or_else(|| "tcp://localhost:9001".to_string());
    let json_output = matches.opt_present("json");

    let free = matches.free.clone();
    let command = match free.first() {
        Some(c) => c.clone(),
        None => {
            eprintln!("Usage: yggdrasilctl [options] <command> [key=value ...]");
            eprintln!("Commands: list, getSelf, getPeers, getTree, addPeer, removePeer");
            std::process::exit(1);
        }
    };

    // Parse key=value arguments into a JSON object
    let mut arguments = serde_json::Map::new();
    for arg in &free[1..] {
        if let Some((k, v)) = arg.split_once('=') {
            arguments.insert(k.to_string(), serde_json::Value::String(v.to_string()));
        }
    }

    let request = serde_json::json!({
        "request": command,
        "arguments": arguments,
        "keepalive": false,
    });

    let addr = endpoint
        .strip_prefix("tcp://")
        .unwrap_or(&endpoint);

    let stream = TcpStream::connect(addr).await.map_err(|e| {
        format!(
            "Failed to connect to admin socket at {}: {}",
            endpoint, e
        )
    })?;

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    // Send request
    let req_json = serde_json::to_string(&request)?;
    writer.write_all(req_json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    // Read response
    let mut line = String::new();
    reader.read_line(&mut line).await?;

    if line.trim().is_empty() {
        eprintln!("Empty response from admin socket");
        std::process::exit(1);
    }

    let resp: serde_json::Value = serde_json::from_str(line.trim())?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }

    // Check status
    let status = resp
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    if status != "success" {
        let error = resp
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown error");
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    let response = &resp["response"];

    // Pretty-print based on command
    match command.to_lowercase().as_str() {
        "list" => {
            if let Some(list) = response.get("list").and_then(|v| v.as_array()) {
                println!("Available commands:");
                for cmd in list {
                    if let Some(s) = cmd.as_str() {
                        println!("  {}", s);
                    }
                }
            }
        }

        "getself" => {
            print_kv(response, &[
                ("Build name", "build_name"),
                ("Build version", "build_version"),
                ("Public key", "key"),
                ("IPv6 address", "address"),
                ("IPv6 subnet", "subnet"),
                ("Routing entries", "routing_entries"),
            ]);
        }

        "getpeers" => {
            if let Some(peers) = response.get("peers").and_then(|v| v.as_array()) {
                if peers.is_empty() {
                    println!("No peers connected.");
                } else {
                    for (i, peer) in peers.iter().enumerate() {
                        if i > 0 {
                            println!();
                        }
                        print_kv(peer, &[
                            ("URI", "uri"),
                            ("Up", "up"),
                            ("Inbound", "inbound"),
                            ("Public key", "key"),
                            ("IPv6 address", "address"),
                            ("IPv6 subnet", "subnet"),
                            ("Priority", "priority"),
                            ("Bytes received", "bytes_recvd"),
                            ("Bytes sent", "bytes_sent"),
                            ("RX rate", "rx_rate"),
                            ("TX rate", "tx_rate"),
                            ("Uptime", "uptime"),
                            ("Last error", "last_error"),
                        ]);
                    }
                }
            }
        }

        "gettree" => {
            if let Some(tree) = response.get("tree").and_then(|v| v.as_array()) {
                if tree.is_empty() {
                    println!("No tree entries.");
                } else {
                    for (i, entry) in tree.iter().enumerate() {
                        if i > 0 {
                            println!();
                        }
                        print_kv(entry, &[
                            ("Public key", "key"),
                            ("IPv6 address", "address"),
                            ("Parent", "parent"),
                            ("Sequence", "sequence"),
                        ]);
                    }
                }
            }
        }

        _ => {
            // Generic: print the response as pretty JSON
            println!("{}", serde_json::to_string_pretty(response)?);
        }
    }

    Ok(())
}

fn print_kv(obj: &serde_json::Value, fields: &[(&str, &str)]) {
    let max_label = fields.iter().map(|(l, _)| l.len()).max().unwrap_or(0);
    for (label, key) in fields {
        if let Some(val) = obj.get(key) {
            let val_str = match val {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Null => "n/a".to_string(),
                other => other.to_string(),
            };
            println!("  {:width$}  {}", format!("{}:", label), val_str, width = max_label + 1);
        }
    }
}
