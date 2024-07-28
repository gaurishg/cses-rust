use std::io::Read;

fn f(vec: Vec<usize>, queries: Vec<(usize, usize)>) -> Vec<usize> {
    let cumsum = std::iter::once(0)
        .chain(vec.into_iter().scan(0, |state, ele| {
            *state += ele;
            Some(*state)
        }))
        .collect::<Vec<_>>();

    queries
        .into_iter()
        .map(|(a, b)| cumsum[b] - cumsum[a - 1])
        .collect()
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<(usize, usize)>) {
    (
        s.lines()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        s.lines()
            .skip(2)
            .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    )
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    let (v, q) = parse_input(&s);
    for x in f(v, q) {
        println!("{x}");
    }
}

#[cfg(test)]
mod test_static_range_sum_queries {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "8 4
3 2 4 5 1 1 5 3
2 4
5 6
1 8
3 3";
        let (v, q) = parse_input(s);
        assert_eq!(f(v, q), vec![11, 2, 24, 4]);
    }
}
