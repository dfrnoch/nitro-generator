use cli::output::{display_message, MessageType};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

mod cli;
mod proxy;

#[tokio::main]
async fn main() {
    let codes = generate_codes(cli::input::input("How many codes do you want to generate?"));
    let threads: usize = cli::input::input("How many threads do you want to use?");

    let proxy_swap = Arc::new(Mutex::new(0));
    let proxies = Arc::new(proxy::scrape().await.unwrap());
    let codes = Arc::new(parse_codes(codes, threads));
    let mut handles = vec![];

    for i in 0..threads {
        let split_codes = codes[i].clone();
        let proxy_swap = Arc::clone(&proxy_swap);
        let proxies = Arc::clone(&proxies);

        let join = task::spawn(async move {
            let mut proxy_swap_lock = *proxy_swap.lock().await;
            for code in split_codes {
                if proxy_swap_lock > proxies.len() - 1 {
                    proxy_swap_lock = 0;
                }

                let proxy = &proxies[proxy_swap_lock];
                proxy_swap_lock += 1;

                let blacklist_check = proxy::check(proxy).await;
                if blacklist_check.is_err() {
                    continue;
                }

                let r = proxy::request(proxy, &code).await;

                let status = match &r {
                    Ok(r) => r.status(),
                    Err(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                };

                match status {
                    reqwest::StatusCode::OK => {
                        display_message(MessageType::Success, &format!("{} is valid", code));
                    }
                    reqwest::StatusCode::NOT_FOUND => {
                        display_message(MessageType::Error, &format!("{} is invalid", code));
                    }
                    reqwest::StatusCode::TOO_MANY_REQUESTS => {
                        display_message(MessageType::Warning, &format!("{} is ratelimited", code));
                    }
                    _ => {
                        display_message(
                            MessageType::Warning,
                            &format!("Proxy {} is invalid, removing from list", proxy),
                        );
                        proxy::blacklist(proxy).await;
                    }
                }
            }
        });
        handles.push(join);
    }
    for join in handles {
        join.await.unwrap();
    }

    // println!("{:?}", proxy_swap.lock().unwrap());
}

fn parse_codes(codes: Vec<String>, threads: usize) -> Vec<Vec<String>> {
    let mut parsed_codes: Vec<Vec<String>> = vec![];
    let parsed_codes_size: usize = codes.len() / threads;
    for i in 0..threads {
        let mut chunk: Vec<String> = vec![];
        for j in i * parsed_codes_size..(i + 1) * parsed_codes_size {
            chunk.push(codes[j].clone());
        }
        parsed_codes.push(chunk);
    }
    parsed_codes
}

fn generate_codes(count: i32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut codes = Vec::new();

    for _ in 0..count {
        let code: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();

        codes.push(code);
    }
    codes
}
