use std::io::Read;

fn f(n: usize) -> Vec<usize> {
    (0..(1 << n)).map(|x| x ^ (x >> 1)).collect()
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let n = buffer.trim().parse().unwrap();
    for x in f(n) {
        let mut binary = format!("{:b}", x);
        while binary.len() < n {
            binary.insert(0, '0');
        }
        println!("{binary}");
    }
}
