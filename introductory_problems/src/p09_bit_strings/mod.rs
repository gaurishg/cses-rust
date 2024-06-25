use std::io::Read;

fn f(n: u64) -> u64 {
    (1..=n).fold(1, |acc, _| (acc << 1) % 1_000_000_007)
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    println!("{}", f(buffer.trim().parse().unwrap()));
}

#[cfg(test)]
mod test {
    use std::io::BufRead;

    use super::f;

    #[test]
    fn test_from_file() {
        let filename = std::path::Path::new("..")
            .join(file!())
            .parent()
            .unwrap()
            .join("input.txt");
        let file = std::fs::File::open(filename).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();
        let n_tests = lines.next().unwrap().unwrap().parse().unwrap();
        for _ in 1..=n_tests {
            let n = lines.next().unwrap().unwrap().parse().unwrap();
            let my_answer = f(n);
            let expected_answer = lines.next().unwrap().unwrap().parse().unwrap();
            assert_eq!(my_answer, expected_answer);
        }
    }
}
