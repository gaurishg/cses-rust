use std::io::Read;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Query {
    Update { pos: usize, value: usize },
    Get { from: usize, to: usize },
}

struct SegmentTree {
    n: usize,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn from(v: Vec<usize>) -> Self {
        let n = v.len().next_power_of_two();
        let mut tree = vec![usize::MAX; 2 * n];
        tree[n..n + v.len()].copy_from_slice(&v[..]);

        for i in (1..n).rev() {
            tree[i] = tree[2 * i].min(tree[2 * i + 1]);
        }

        Self { n, tree }
    }

    fn update(&mut self, mut pos: usize, value: usize) {
        pos += self.n - 1;
        self.tree[pos] = value;
        pos /= 2;
        while pos != 0 {
            self.tree[pos] = self.tree[2 * pos].min(self.tree[2 * pos + 1]);
            pos /= 2;
        }
    }

    fn get(&self, mut a: usize, mut b: usize) -> usize {
        a += self.n - 1;
        b += self.n - 1;

        let mut answer = usize::MAX;
        while a <= b {
            if a % 2 == 1
            // right child
            {
                answer = answer.min(self.tree[a]);
                a += 1;
            }
            if b % 2 == 0
            // left child
            {
                answer = answer.min(self.tree[b]);
                b -= 1;
            }

            a /= 2;
            b /= 2;
        }

        answer
    }
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
            .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| {
                let x = it.next().unwrap();
                let y = it.next().unwrap();
                let z = it.next().unwrap();
                if x == 1 {
                    Query::Update { pos: y, value: z }
                } else {
                    Query::Get { from: y, to: z }
                }
            })
            .collect(),
    )
}

fn f(v: Vec<usize>, queries: Vec<Query>) -> Vec<usize> {
    let mut segtree = SegmentTree::from(v);
    queries
        .into_iter()
        .filter_map(|q| match q {
            Query::Update { pos, value } => {
                segtree.update(pos, value);
                None
            }
            Query::Get { from, to } => Some(segtree.get(from, to)),
        })
        .collect()
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
mod test_dynamic_range_minimum_queries {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "8 4
3 2 4 5 1 1 5 3
2 1 4
2 5 6
1 2 3
2 1 4";
        let (v, q) = parse_input(s);
        assert_eq!(f(v, q), vec![2, 1, 3]);
    }
}
