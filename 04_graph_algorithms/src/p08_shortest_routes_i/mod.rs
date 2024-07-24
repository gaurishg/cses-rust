use std::{
    collections::{BinaryHeap, HashMap},
    io::Read,
};

struct Edge {
    source: usize,
    target: usize,
    weight: usize,
}

fn f(n_vertices: usize, edges: Vec<Edge>) -> Vec<usize> {
    let mut adj = vec![HashMap::new(); n_vertices];
    for edge in edges {
        adj[edge.source]
            .entry(edge.target)
            .and_modify(|weight| *weight = std::cmp::min(*weight, edge.weight))
            .or_insert(edge.weight);
    }

    let mut costs = vec![usize::MAX; n_vertices];
    let mut heap = BinaryHeap::new();
    heap.push((std::cmp::Reverse(0), 0));

    while !heap.is_empty() {
        let (std::cmp::Reverse(cost), node) = heap.pop().unwrap();
        if costs[node] < cost {
            continue;
        }
        costs[node] = cost;
        for (&child, &weight) in &adj[node] {
            if costs[child] > costs[node] + weight {
                costs[child] = costs[node] + weight;
                heap.push((std::cmp::Reverse(costs[child]), child));
            }
        }
    }

    costs
}

fn get_edges(s: &str) -> Vec<Edge> {
    s.lines()
        .skip(1)
        .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
        .map(|mut it| Edge {
            source: it.next().unwrap() - 1,
            target: it.next().unwrap() - 1,
            weight: it.next().unwrap(),
        })
        .collect()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    for x in f(
        s.split_whitespace().next().unwrap().parse().unwrap(),
        get_edges(&s),
    ) {
        print!("{x} ");
    }
}

#[cfg(test)]
mod test_shortest_routes_i {
    use super::{f, get_edges};

    #[test]
    fn example() {
        let s = "3 4
1 2 6
1 3 2
3 2 3
1 3 4";
        assert_eq!(f(3, get_edges(s)), vec![0, 5, 2]);
    }

    #[test]
    fn test_1() {
        let s = "10 20
8 5 1
9 10 2
7 9 8
9 8 8
10 9 9
7 8 10
8 9 2
7 10 10
4 5 8
5 6 1
4 2 1
5 3 6
10 7 3
3 5 2
5 4 7
1 2 9
2 3 2
6 7 5
3 4 10
3 2 10";
        assert_eq!(
            f(10, get_edges(s)),
            "0 9 11 20 13 14 19 29 27 29"
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_2() {
        let s = "10 20
5 6 4
5 1 7
7 4 4
7 8 1
8 9 3
5 7 6
2 3 2
1 2 7
7 9 5
2 5 8
8 5 7
4 5 6
6 7 6
8 7 6
6 2 2
3 6 8
6 4 7
9 10 5
5 2 1
3 4 8";
        assert_eq!(
            f(10, get_edges(s)),
            "0 7 9 17 15 17 21 22 25 30"
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        );
    }
}
