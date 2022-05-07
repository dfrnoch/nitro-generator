use std::{
    fmt::Debug,
    io::{self, Write},
    str::FromStr,
};

pub fn input<T: std::str::FromStr>(message: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    print!("[?] {}: ", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().expect("Invalid input")
}
