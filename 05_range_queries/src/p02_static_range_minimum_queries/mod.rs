use std::{io::Read, usize};

struct RangeQuery {
    n: usize,
    tree: Vec<usize>,
}

impl RangeQuery {
    fn from(v: Vec<usize>) -> Self {
        let n = v.len().next_power_of_two();
        let mut tree = vec![usize::MAX; 2 * n];
        tree[n..n + v.len()].copy_from_slice(&v[..]);

        for i in (1..n).rev() {
            tree[i] = tree[2 * i].min(tree[2 * i + 1]);
        }

        Self { n, tree }
    }

    fn get_internal(
        &self,
        i: usize,
        range_l: usize,
        range_r: usize,
        ql: usize,
        qr: usize,
    ) -> usize {
        if range_r < ql || range_l > qr {
            return usize::MAX;
        }
        if range_l >= ql && range_r <= qr {
            return self.tree[i];
        }
        let last_of_first = (range_l + range_r) / 2;
        self.get_internal(2 * i, range_l, last_of_first, ql, qr)
            .min(self.get_internal(2 * i + 1, last_of_first + 1, range_r, ql, qr))
    }

    fn get(&self, a: usize, b: usize) -> usize {
        self.get_internal(1, 0, self.n - 1, a - 1, b - 1)
    }
}

fn f(v: Vec<usize>, queries: Vec<(usize, usize)>) -> Vec<usize> {
    let rq = RangeQuery::from(v);
    queries.into_iter().map(|(a, b)| rq.get(a, b)).collect()
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
mod test_static_range_minimum_queries {
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
        assert_eq!(f(v, q), vec![2, 1, 1, 4]);
    }
}
