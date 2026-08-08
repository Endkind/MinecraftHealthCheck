use std::env;
use tokio::time::sleep;
use std::time::Duration;
use rust_mc_status::ping_java;
use std::io::{self, Write};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 25565;
const DEFAULT_ATTEMPTS: u32 = 5;
const DEFAULT_ATTEMPT_DELAY: f64 = 5.0;
const DEFAULT_DEBUG_MESSAGES: bool = false;

#[tokio::main]
async fn main() {
    let host = env::var("HEALTHCHECK_HOST")
        .unwrap_or_else(|_| DEFAULT_HOST.to_string());
    let port = env::var("HEALTHCHECK_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);
    let attempts = env::var("HEALTHCHECK_ATTEMPTS")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(DEFAULT_ATTEMPTS);
    let delay = env::var("HEALTHCHECK_DELAY")
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(DEFAULT_ATTEMPT_DELAY);
    let debug_messages = env::var("HEALTHCHECK_DEBUG_MESSAGES")
        .ok()
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(DEFAULT_DEBUG_MESSAGES);

    match is_minecraft_server_reachable(
        &host,
        port,
        attempts,
        Duration::from_secs_f64(delay),
        debug_messages,
    ).await {
        Ok(version) => {
            if debug_messages {
                println!("Minecraft server is reachable. Server: {version}")
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

async fn is_minecraft_server_reachable(
    host: &str,
    port: u16,
    attempts: u32,
    attempt_delay: Duration,
    debug_messages: bool,
) -> Result<String, String> {
    if attempts <= 0 {
        return Err("Attempts must be at least 1".to_string());
    }

    let address = format!("{host}:{port}");
    let mut last_error = None;

    for attempt in 1..=attempts {
        if debug_messages {
            print!("\r{attempt}/{attempts} {address}");
            io::stdout().flush().ok();
        }

        match ping_java(&address).await {
            Ok(status) => {
                return Ok(status.version().to_string());
            }
            Err(error) => {
                last_error = Some(error.to_string());

                if attempt < attempts {
                    sleep(attempt_delay).await;
                }
            }
        }
    }
    if debug_messages {
        print!("\r\x1b[2K");
        io::stdout().flush().ok();
    }

    let error_details = last_error
        .map(|error| format!("Last Error: {error}"))
        .unwrap_or_default();

    Err(format!(
        "Unable to reach Minecraft server at '{address}' after {attempts} attempt(s) \
         with {}s delay between attempts. \
         Please verify that the server is running and the port is accessible. {error_details}",
        attempt_delay.as_secs_f64()
    ))
}