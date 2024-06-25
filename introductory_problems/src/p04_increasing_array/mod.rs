pub fn f(nums: Vec<i64>) -> i64 {
    let mut sum = 0;
    let mut maxv = *nums.first().unwrap();
    for x in nums {
        sum += std::cmp::max(0, maxv - x);
        maxv = std::cmp::max(maxv, x);
    }
    sum
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    //let n = buffer.parse().unwrap();
    buffer.clear();
    let _ = std::io::stdin().read_line(&mut buffer);
    let nums = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", f(nums));
}

#[cfg(test)]
mod test {
    use std::{io::BufRead, str::FromStr};

    use super::f;

    #[test]
    fn test_from_file() {
        let filename = std::path::PathBuf::from_str("..")
            .unwrap()
            .join(file!())
            .parent()
            .unwrap()
            .join("input.txt");
        let file = std::fs::File::open(filename).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();
        let n_input = lines.next().unwrap().unwrap().parse().unwrap();
        for _ in 1..=n_input {
            lines.next();
            let nums = lines
                .next()
                .unwrap()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let correct_answer: i64 = lines.next().unwrap().unwrap().parse().unwrap();
            let my_answer = f(nums);
            assert_eq!(correct_answer, my_answer);
        }
    }
}
