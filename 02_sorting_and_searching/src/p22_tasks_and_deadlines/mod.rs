use std::io::Read;

fn f(mut tasks: Vec<(i64, i64)>) -> i64 {
    let sum_of_deadlines = tasks.iter().fold(0, |acc, (_, deadline)| acc + deadline);
    tasks.sort_unstable();
    sum_of_deadlines
        - tasks
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, (duration, _))| {
                acc + (idx as i64 + 1) * duration
            })
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let mut lines = buf.lines();
    lines.next();

    println!(
        "{}",
        f(lines
            .map(|s| s.split_whitespace())
            .map(|mut s| {
                let duration = s.next().unwrap().parse().unwrap();
                let deadline = s.next().unwrap().parse().unwrap();
                (duration, deadline)
            })
            .collect())
    );
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_1() {
        assert_eq!(2, f(vec![(6, 10), (8, 15), (5, 12)]));
    }
}
