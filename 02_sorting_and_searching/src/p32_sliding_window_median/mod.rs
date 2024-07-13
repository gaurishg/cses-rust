use std::{
    collections::{BTreeSet, VecDeque},
    io::Read,
};

#[derive(Debug)]
struct Median {
    k: usize,
    lower: BTreeSet<(usize, usize)>,
    higher: BTreeSet<(usize, usize)>,
    q: VecDeque<(usize, usize)>,
    i: usize,
}

impl Median {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            lower: Default::default(),
            higher: Default::default(),
            q: Default::default(),
            i: 0,
        }
    }

    pub fn push(&mut self, val: usize) {
        self.lower.insert((val, self.i));
        self.q.push_back((val, self.i));
        if self.q.len() > self.k {
            self.remove();
        }
        self.balance();
        self.i += 1;
    }

    pub fn get_median(&mut self) -> usize {
        self.lower.last().unwrap().0
    }

    fn balance(&mut self) {
        self.higher.insert(self.lower.pop_last().unwrap());
        if self.lower.len() < self.higher.len() {
            self.lower.insert(self.higher.pop_first().unwrap());
        }
    }

    fn remove(&mut self) {
        let ele = self.q.pop_front().unwrap();
        if !self.lower.remove(&ele) {
            self.higher.remove(&ele);
        }
    }
}

fn f(k: usize, v: Vec<usize>) -> Vec<usize> {
    let mut median = Median::new(k);
    for x in v.iter().take(k - 1) {
        median.push(*x);
    }

    v.into_iter()
        .skip(k - 1)
        .map(|x| {
            median.push(x);
            median.get_median()
        })
        .collect()
}

pub fn main() {
    let mut s = String::new();
    let _ = std::io::stdin().read_to_string(&mut s);
    let k = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let v = s
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for x in f(k, v) {
        print!("{x} ");
    }
}

#[cfg(test)]
mod test {
    use super::f;

    #[test]
    fn test_example() {
        assert_eq!(vec![3, 4, 5, 5, 2, 1], f(3, vec![2, 4, 3, 5, 8, 1, 2, 1]));
    }

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1,],
            f(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1,])
        );
    }
}
