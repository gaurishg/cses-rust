use std::io::Read;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Query {
    Update { position: usize, value: usize },
    GetSum { from: usize, to: usize },
}

struct FenwickTree {
    n: usize,
    tree: Vec<usize>,
}

impl FenwickTree {
    fn from(v: Vec<usize>) -> Self {
        let n = v.len();
        let mut tree = vec![0; n + 1];
        for i in 1..=n {
            tree[i] += v[i - 1];
            let one_level_up = i + (1 << i.trailing_zeros());
            if one_level_up <= n {
                tree[one_level_up] += tree[i];
            }
        }

        Self { n, tree }
    }

    fn update(&mut self, mut pos: usize, value: usize) {
        let old_value = self.get_range(pos, pos);
        while pos <= self.n {
            self.tree[pos] -= old_value;
            self.tree[pos] += value;
            pos += 1 << pos.trailing_zeros();
        }
    }

    fn get_upto(&self, mut pos: usize) -> usize {
        let mut value = 0;
        while pos != 0 {
            value += self.tree[pos];
            pos -= 1 << pos.trailing_zeros();
        }

        value
    }

    fn get_range(&self, from: usize, to: usize) -> usize {
        self.get_upto(to) - self.get_upto(from - 1)
    }
}

fn f(v: Vec<usize>, queries: Vec<Query>) -> Vec<usize> {
    let mut fenwick = FenwickTree::from(v);
    queries
        .into_iter()
        .filter_map(|q| match q {
            Query::Update { position, value } => {
                fenwick.update(position, value);
                None
            }
            Query::GetSum { from, to } => Some(fenwick.get_range(from, to)),
        })
        .collect()
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<Query>) {
    (
        s.lines()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        s.lines()
            .skip(2)
            .map(|l| l.split_whitespace())
            .map(|mut it| {
                let x: usize = it.next().unwrap().parse().unwrap();
                let y = it.next().unwrap().parse().unwrap();
                let z = it.next().unwrap().parse().unwrap();
                if x == 1 {
                    Query::Update {
                        position: y,
                        value: z,
                    }
                } else {
                    Query::GetSum { from: y, to: z }
                }
            })
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
mod test_dynamic_range_sum_queries {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "8 4
3 2 4 5 1 1 5 3
2 1 4
2 5 6
1 3 1
2 1 4";
        let (v, q) = parse_input(s);
        assert_eq!(f(v, q), vec![14, 2, 11]);
    }
}
