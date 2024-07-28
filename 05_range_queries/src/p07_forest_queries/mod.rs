use std::io::Read;

fn f(
    n: usize,
    mut forest: Vec<Vec<usize>>,
    queries: Vec<(usize, usize, usize, usize)>,
) -> Vec<usize> {
    for i in 1..=n {
        for j in 1..=n {
            forest[i][j] += forest[i - 1][j] + forest[i][j - 1] - forest[i - 1][j - 1];
        }
    }

    queries
        .into_iter()
        .map(|(r1, c1, r2, c2)| {
            forest[r2][c2] - forest[r1 - 1][c2] - forest[r2][c1 - 1] + forest[r1 - 1][c1 - 1]
        })
        .collect()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    let (n, forest, q) = parse_input(&s);
    for x in f(n, forest, q) {
        println!("{x}");
    }
}

fn parse_input(s: &str) -> (usize, Vec<Vec<usize>>, Vec<(usize, usize, usize, usize)>) {
    let mut it = s.split_whitespace();
    let n = it.next().unwrap().parse().unwrap();
    let n_queries = it.next().unwrap().parse().unwrap();

    let mut forest = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        let line = it.next().unwrap().as_bytes();
        for j in 1..=n {
            if line[j - 1] == b'*' {
                forest[i][j] = 1;
            }
        }
    }

    let mut queries = vec![];
    for _ in 0..n_queries {
        queries.push((
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        ));
    }

    (n, forest, queries)
}

#[cfg(test)]
mod test_forest_queries {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "4 3
.*..
*.**
**..
****
2 2 3 4
3 1 3 1
1 1 2 2";
        let (n, forest, q) = parse_input(s);
        assert_eq!(f(n, forest, q), vec![3, 1, 2]);
    }
}
