pub fn f(seq: &str) -> i32 {
    let mut maxv = 1;
    let mut current_count = 0;
    let mut prev_char = '\0';
    for c in seq.chars() {
        if prev_char == c {
            current_count += 1;
        } else {
            current_count = 1;
        }
        prev_char = c;
        maxv = std::cmp::max(maxv, current_count);
    }
    maxv
}

pub fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    println!("{}", f(&input));
}

#[cfg(test)]
mod test {
    use std::{io::BufRead, str::FromStr};

    use super::f;

    #[test]
    fn test_from_file() {
        let filepath = std::path::PathBuf::from_str("..")
            .unwrap()
            .join(file!())
            .parent()
            .unwrap()
            .join("input.txt");
        dbg!(&filepath);
        let file = std::fs::File::open(filepath).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();
        let n_input = lines.next().unwrap().unwrap().parse().unwrap();
        for _ in 1..=n_input {
            let dna_seq = lines.next().unwrap().unwrap();
            let correct_answer: i32 = lines.next().unwrap().unwrap().parse().unwrap();
            let my_answer = f(&dna_seq);
            assert_eq!(correct_answer, my_answer);
        }
    }
}
