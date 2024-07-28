use std::io::Read;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Query {
    IncreaseRange {
        a: usize,
        b: usize,
        increase_by: usize,
    },
    Get(usize),
}

struct SegmentTree {
    n: usize,
    deltas: Vec<usize>,
    v: Vec<usize>,
}

impl SegmentTree {
    fn from(v: Vec<usize>) -> Self {
        let n = v.len().next_power_of_two();
        Self {
            n,
            deltas: vec![0; 2 * n],
            v,
        }
    }

    fn update(&mut self, mut a: usize, mut b: usize, increase_by: usize) {
        a += self.n - 1;
        b += self.n - 1;

        while a <= b {
            if a % 2 == 1
            // right child
            {
                self.deltas[a] += increase_by;
                a += 1;
            }
            if b % 2 == 0
            // left child
            {
                self.deltas[b] += increase_by;
                b -= 1;
            }

            a /= 2;
            b /= 2;
        }
    }

    fn get(&self, mut pos: usize) -> usize {
        let mut total = self.v[pos - 1];
        pos += self.n - 1;
        while pos != 0 {
            total += self.deltas[pos];
            pos /= 2;
        }

        total
    }
}

fn f(v: Vec<usize>, queries: Vec<Query>) -> Vec<usize> {
    let mut segtree = SegmentTree::from(v);
    queries
        .into_iter()
        .filter_map(|q| match q {
            Query::Get(pos) => Some(segtree.get(pos)),
            Query::IncreaseRange { a, b, increase_by } => {
                segtree.update(a, b, increase_by);
                None
            }
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
            .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| {
                let qtype = it.next().unwrap();
                if qtype == 1 {
                    Query::IncreaseRange {
                        a: it.next().unwrap(),
                        b: it.next().unwrap(),
                        increase_by: it.next().unwrap(),
                    }
                } else {
                    Query::Get(it.next().unwrap())
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
