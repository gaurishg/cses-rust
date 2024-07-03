use std::collections::HashSet;

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer.clear();

    let _ = std::io::stdin().read_line(&mut buffer);
    let nums: Vec<isize> = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map = HashSet::new();
    let mut rounds = 0;
    for x in nums {
        if map.get(&(x - 1)).is_none() {
            rounds += 1;
        }
        map.insert(x);
    }

    println!("{rounds}");
}
