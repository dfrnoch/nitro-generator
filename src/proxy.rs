use std::fs::File;
use std::io::Write;

pub fn scrape() -> Result<Vec<String>, reqwest::Error> {
    let mut scraped = 0;
    let mut f = File::create("proxies.txt").expect("Unable to create file");
    f.write_all(b"").expect("Unable to write data");
    let r = reqwest::blocking::get(
        "https://api.proxyscrape.com/?request=displayproxies&proxytype=http&timeout=1500&ssl=yes",
    )?
    .text()?;
    let mut proxies = vec![];

    for mut proxy in r.split('\n') {
        proxy = proxy.trim();
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

    println!("[?] Scraped {} proxies.", scraped);
    return Ok(proxies);
}
