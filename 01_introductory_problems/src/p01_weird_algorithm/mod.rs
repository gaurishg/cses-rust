fn f(mut n: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    loop {
        ret.push(n);
        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }

    ret
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim().parse().unwrap();
    for x in f(n).iter() {
        print!("{} ", *x);
    }
}

#[cfg(test)]
mod test {
    use std::{io::BufRead, str::FromStr};

    use super::*;

    #[test]
    fn test_from_file() {
        let filepath = std::path::PathBuf::from_str("..")
            .unwrap()
            .join(file!())
            .parent()
            .unwrap()
            .join("input.txt");
        println!("{filepath:?}");
        let file = std::fs::File::options().read(true).open(filepath).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();
        let n_input = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
        println!("{n_input:?}");
        for input_idx in 1..=n_input {
            let n = lines.next().unwrap().unwrap().parse().unwrap();
            let result: Vec<i64> = lines
                .next()
                .unwrap()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let my_result = f(n);
            assert_eq!(
                result,
                my_result,
                "Input {} failed, expected={result:?}, got={my_result:?}",
                input_idx + 1
            );
        }
        //assert!(false);
    }
}
