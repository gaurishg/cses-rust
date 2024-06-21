pub fn f(n: i32, numbers: Vec<i32>) -> i32 {
    std::iter::zip(numbers, 1..n).fold(0, |acc, (x, y)| acc ^ x ^ y) ^ n
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse().unwrap();
    buffer.clear();
    let _ = std::io::stdin().read_line(&mut buffer);
    let nums = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", f(n, nums));
}

#[cfg(test)]
mod test {
    use std::{io::BufRead, str::FromStr};

    use crate::p02_missing_number::f;

    #[test]
    fn test_from_file() {
        let filepath = std::path::PathBuf::from_str("..")
            .unwrap()
            .join(file!())
            .parent()
            .unwrap()
            .join("input.txt");
        let file = std::fs::File::options().read(true).open(filepath).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();
        let n_input = lines.next().unwrap().unwrap().parse().unwrap();
        for _ in 1..=n_input {
            let n = lines.next().unwrap().unwrap().parse().unwrap();
            let nums = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let correct_result = lines.next().unwrap().unwrap().parse().unwrap();
            let my_result = f(n, nums);
            assert_eq!(my_result, correct_result);
        }
    }
}
