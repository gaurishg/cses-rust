use std::io::Read;

struct UnionFind {
    number_of_groups: usize,
    data: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            number_of_groups: n,
            data: (0..=n).collect(),
        }
    }

    fn root(&mut self, mut i: usize) -> usize {
        while i != self.data[i] {
            self.data[i] = self.data[self.data[i]];
            i = self.data[self.data[i]];
        }
        i
    }

    pub fn connected(&mut self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }

    pub fn join(&mut self, i: usize, j: usize) {
        let root_i = self.root(i);
        let root_j = self.root(j);
        if !self.connected(i, j) {
            self.data[root_i] = self.data[root_j];
            self.number_of_groups -= 1;
        }
    }

    pub fn groups(&self) -> usize {
        self.number_of_groups
    }
}

fn f(n_vertices: usize, edges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut data = UnionFind::new(n_vertices);

    for (i, j) in edges {
        data.join(i, j);
    }

    let mut result = Vec::new();

    'outer: for i in 1..=n_vertices {
        for j in i + 1..=n_vertices {
            if !data.connected(i, j) {
                result.push((i, j));
                data.join(i, j);
                if data.groups() == 1 {
                    break 'outer;
                }
            }
        }
    }

    result
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let n_vertices = s
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(0)
        .unwrap()
        .parse()
        .unwrap();
    let roads = s
        .lines()
        .skip(1)
        .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()))
        .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
        .collect();

    let result = f(n_vertices, roads);
    println!("{}", result.len());
    for (i, j) in result {
        println!("{i} {j}");
    }
}

#[cfg(test)]
mod test_building_roads {
    use super::{f, UnionFind};

    #[test]
    fn example() {
        let n = 4;
        let roads = vec![(1, 2), (3, 4)];
        let mut data = UnionFind::new(n);
        for &(i, j) in roads.iter() {
            data.join(i, j);
        }

        let roads = f(n, roads);
        for (i, j) in roads {
            assert!(!data.connected(i, j));
            data.join(i, j);
        }
    }
}
