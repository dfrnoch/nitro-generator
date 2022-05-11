use std::fs::File;
use std::io::Write;

use crate::cli::output::{display_message, MessageType};

pub fn scrape() -> Result<Vec<String>, reqwest::Error> {
    let mut scraped = 0;
    let mut f = File::create("proxies.txt").expect("Unable to create file");
    let r = reqwest::blocking::get(
        "https://api.proxyscrape.com/?request=displayproxies&proxytype=http&timeout=1500&ssl=yes",
    );
    if Result::is_err(&r) {
        display_message(MessageType::Warning, "Failed to fetch proxies, retrying...");
        return scrape();
    }

    let mut proxies = vec![];
    let r = r.unwrap().text()?;

    for proxy in r.trim().lines() {
        if !proxy.is_empty() {
            let proxy = format!("https://{}", proxy);
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

pub fn check(proxy: &str, code: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::https(proxy)?)
        .build()?;

    let mut r = client.get(format!("https://discordapp.com/reedem/{}", code).as_str());

    return Ok(());
}
