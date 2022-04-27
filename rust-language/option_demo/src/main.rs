// Rust Option 例子

fn main() {
    println!(
        "{}",
        match find_store("android") {
            Some(s) => s,
            None => "Not a valid mobile OS",
        }
    );
}

fn find_store(mobile_os: &str) -> Option<&str> {
    match mobile_os {
        "iOS" => Some("App Store"),
        "android" => Some("Play Store"),
        _ => None,
    }
}
