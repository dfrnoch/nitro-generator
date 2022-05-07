use rand::distributions::Alphanumeric;
use rand::Rng;
use std::thread;

mod cli;
mod proxy;

fn main() {
    let codes = generate_codes(cli::input::input("How many codes do you want to generate?"));
    let threads: usize = cli::input::input("How many threads do you want to use?");

    let proxies = proxy::scrape().unwrap();
    let codes = parse_codes(codes, threads);

    println!("{:?}", codes);

    for i in 0..threads {
        let code = codes[i].clone();

        thread::spawn(move || {
            println!("{:?}", code);
        });
    }
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
