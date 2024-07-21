use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

fn f(n: usize, connections: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut edges = vec![HashSet::new(); n + 1];
    for (i, j) in connections {
        edges[i].insert(j);
        edges[j].insert(i);
    }

    let mut q = VecDeque::from([1]);
    let mut parents = (0..=n).collect::<Vec<_>>();
    let mut visited = vec![false; n + 1];
    visited[1] = true;

    'outer: while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for &v in edges[u].iter() {
            if !visited[v] {
                visited[v] = true;
                q.push_back(v);
                parents[v] = u;
                if v == n {
                    break 'outer;
                }
            }
        }
    }

    if !visited[n] {
        return None;
    }

    let mut result = vec![n];
    let mut i = n;
    while i != 1 {
        result.push(parents[i]);
        i = parents[i];
    }
    Some(result.into_iter().rev().collect())
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let result = f(
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    );
    if let Some(v) = result {
        println!("{}", v.len());
        for x in v {
            print!("{x} ");
        }
        println!();
    } else {
        println!("IMPOSSIBLE");
    }
}

#[cfg(test)]
mod test_message_route {
    struct UnionFind {
        data: Vec<usize>,
    }

    impl UnionFind {
        fn new(n: usize) -> Self {
            Self {
                data: (0..=n).collect(),
            }
        }

        fn root(&mut self, i: usize) -> usize {
            if i == self.data[i] {
                return i;
            }

            self.data[i] = self.root(self.data[i]);
            return self.data[i];
        }

        fn connected(&mut self, i: usize, j: usize) -> bool {
            self.root(i) == self.root(j)
        }

        fn join(&mut self, i: usize, j: usize) {
            let root_i = self.root(i);
            let root_j = self.root(j);
            self.data[root_i] = root_j;
        }
    }

    use super::f;

    fn check_answer(
        n: usize,
        connections: Vec<(usize, usize)>,
        result: Option<Vec<usize>>,
    ) -> bool {
        let mut uf = UnionFind::new(n);
        for (i, j) in connections {
            uf.join(i, j);
        }

        if result.is_none() {
            return !uf.connected(1, n);
        }

        let v = result.unwrap();
        if v[0] != 1 || v.last().unwrap() != &n {
            return false;
        }

        for w in v.windows(2) {
            let i = w[0];
            let j = w[1];
            if !uf.connected(i, j) {
                return false;
            }
        }

        true
    }

    #[test]
    fn example() {
        let n = 5;
        let edges = vec![(1, 2), (1, 3), (1, 4), (2, 3), (5, 4)];
        let result = f(n, edges.clone());
        eprintln!("{result:?}");
        assert!(check_answer(n, edges, result));
    }
}
