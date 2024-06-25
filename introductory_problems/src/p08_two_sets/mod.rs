use std::io::Read;

fn f(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let sum = (n * (n + 1)) / 2;
    if sum % 2 == 0 {
        let mut remaining = sum / 2;
        let mut a = Vec::new();
        let mut b = Vec::new();
        for x in (1..=n).rev() {
            if x <= remaining {
                a.push(x);
                remaining -= x;
            } else {
                b.push(x);
            }
        }
        return Some((a, b));
    }
    None
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let n = buffer.trim().parse().unwrap();
    if let Some((vec_a, vec_b)) = f(n) {
        println!("YES");
        println!("{}", vec_a.len());
        for x in vec_a {
            print!("{x} ");
        }
        println!("\n{}", vec_b.len());
        for x in vec_b {
            print!("{x} ");
        }
    } else {
        println!("NO");
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, io::BufRead};

    use super::f;

    #[test]
    fn test_from_file() {
        let filepath = std::path::Path::new("..")
            .join(file!())
            .parent()
            .unwrap()
            .join("input.txt");
        let file = std::fs::File::open(filepath).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();
        let n_tests = lines.next().unwrap().unwrap().parse().unwrap();
        for _ in 1..=n_tests {
            let n = lines.next().unwrap().unwrap().parse().unwrap();
            let my_answer = f(n);
            let expected_yes_or_no = lines.next().unwrap().unwrap();
            if let Some((vec_a, vec_b)) = my_answer {
                assert_eq!(expected_yes_or_no.trim(), "YES");
                assert_eq!(vec_a.iter().sum::<u64>(), vec_b.iter().sum::<u64>());
                assert_eq!(vec_a.len() + vec_b.len(), n as usize);
                assert_eq!(
                    vec_a
                        .iter()
                        .chain(vec_b.iter())
                        .copied()
                        .collect::<HashSet<u64>>(),
                    (1..=n).collect()
                );
            }
        }
    }
}
