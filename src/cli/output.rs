pub enum MessageType {
    Info,
    Warning,
    Error,
    Success,
}

pub fn display_message(message_type: MessageType, message: &str) {
    match message_type {
        MessageType::Success => println!("[+] {}", message),
        MessageType::Error => println!("[-] {}", message),
        MessageType::Warning => println!("[!] {}", message),
        MessageType::Info => println!("[?] {}", message),
    }
}
