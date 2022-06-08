use std::io::Write;
use std::{fs::File, time::Duration};

use async_recursion::async_recursion;

use crate::cli::output::{display_message, MessageType};

static mut BLACKLIST: Vec<String> = Vec::new();

#[async_recursion]
pub async fn scrape() -> Result<Vec<String>, reqwest::Error> {
    let mut scraped = 0;
    let mut f = File::create("proxies.txt").expect("Unable to create file");
    let r = reqwest::get(
        "https://api.proxyscrape.com/?request=displayproxies&proxytype=http&timeout=1500&ssl=yes",
    )
    .await?;
    if r.status().is_server_error() {
        display_message(MessageType::Warning, "Failed to fetch proxies, retrying...");
        return scrape().await;
    }

    let mut proxies = vec![];
    let r = r.text().await?;

    for proxy in r.trim().lines() {
        if !proxy.is_empty() {
            let proxy = format!("http://{}", proxy);
            proxies.push(proxy);
        }
    }

    for p in &proxies {
        scraped = scraped + 1;
        f.write(format!("{}\n", p).as_bytes())
            .expect("Unable to write data");
    }

    display_message(
        MessageType::Success,
        &format!("Scraped {} proxies.", scraped),
    );
    return Ok(proxies);
}

pub async fn check(proxy: &str) -> Result<(), &'static str> {
    // TODO: Make this safe
    if unsafe { BLACKLIST.contains(&proxy.to_string()) } {
        return Err("Error: Proxy is blacklisted");
    }
    Ok(())
}

pub async fn blacklist(proxy: &str) {
    unsafe {
        BLACKLIST.push(proxy.to_string());
    }
}

pub async fn request(proxy: &str, code: &str) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::https(proxy)?)
        .timeout(Duration::from_secs(5))
        .build()?;

    println!("requesting");

    let r = client.get(format!("https://discordapp.com/reedem/{}", code).as_str());
    let r = r.send().await?;

    Ok(r)
}
