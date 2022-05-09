use std::{
    fmt::Debug,
    io::{self, Write},
    str::FromStr,
};

use super::output::{display_message, MessageType};

pub fn input<T: FromStr>(message: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    display_message(MessageType::Info, message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().expect("Invalid input")
}
