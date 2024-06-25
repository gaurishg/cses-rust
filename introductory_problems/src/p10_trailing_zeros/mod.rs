use std::io::Read;

fn f(n: u64) -> u64 {
    (1..)
        .take_while(|&x| n >= 5u64.pow(x))
        .map(|x| n / 5u64.pow(x))
        .sum()
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    println!("{}", f(buffer.trim().parse().unwrap()));
}
