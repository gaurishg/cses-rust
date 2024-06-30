use std::collections::HashSet;

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer.clear();
    let _ = std::io::stdin().read_line(&mut buffer);
    println!(
        "{}",
        buffer
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>()
            .len()
    );
}
