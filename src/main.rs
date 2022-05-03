use rand::distributions::Alphanumeric;
use rand::Rng;
use std::thread;

mod proxy_scrape;

fn main() {
    let codes = generate_codes(10);
    let proxies = proxy_scrape::scrape().unwrap();

    println!("{:?}", proxies);
    println!("{:?}", codes);

    for i in 0..10 {
        let code = codes[i].clone();

        thread::spawn(move || {
            println!("{:?}", code);
        });
    }
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
