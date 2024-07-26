use std::{
    collections::{BinaryHeap, VecDeque},
    io::Read,
};

fn f(n_vertices: usize, k: usize, edges: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let (adj, reverse_adj) = {
        let mut adj = vec![vec![]; n_vertices + 1];
        let mut reverse_adj = vec![vec![]; n_vertices + 1];
        for &(u, v, cost) in &edges {
            adj[u].push((v, cost));
            reverse_adj[v].push(u);
        }
        (adj, reverse_adj)
    };

    let can_reach_destination = {
        let mut can_reach_dest = vec![false; n_vertices + 1];
        let mut q = VecDeque::new();
        q.push_back(n_vertices);

        let mut visited = vec![false; n_vertices + 1];
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            if visited[node] {
                continue;
            }

            visited[node] = true;
            can_reach_dest[node] = true;

            for &child in &reverse_adj[node] {
                q.push_back(child);
            }
        }
        can_reach_dest
    };

    let mut visited = vec![0; n_vertices + 1];
    let mut q = BinaryHeap::new();
    let mut res = vec![];

    q.push((std::cmp::Reverse(0), 1));
    while !q.is_empty() {
        let (std::cmp::Reverse(cost), node) = q.pop().unwrap();
        visited[node] += 1;
        if visited[node] > k {
            if node == n_vertices {
                break;
            } else {
                continue;
            }
        }
        if node == n_vertices {
            res.push(cost);
        }

        for &(child, edge_wt) in &adj[node] {
            if visited[child] < k && can_reach_destination[child] {
                q.push((std::cmp::Reverse(cost + edge_wt), child));
            }
        }
    }

    res
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let (n_vertices, k, edges) = parse_input(&s);
    for x in f(n_vertices, k, edges) {
        print!("{x} ");
    }
}

fn parse_input(s: &str) -> (usize, usize, Vec<(usize, usize, usize)>) {
    (
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.split_whitespace().nth(2).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap(), it.next().unwrap()))
            .collect(),
    )
}

#[cfg(test)]
mod test_flight_routes {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "4 6 3
1 2 1
1 3 3
2 3 2
2 4 6
3 2 8
3 4 1";
        let (n_vertices, k, edges) = parse_input(s);
        assert_eq!(f(n_vertices, k, edges), vec![4, 4, 7]);
    }

    #[test]
    fn test_1() {
        let s = "10 12 3
6 7 5
3 4 5
1 3 2
4 6 3
7 8 3
2 4 4
5 7 1
8 10 2
4 5 4
1 2 3
7 9 2
9 10 4";
        let (n, k, e) = parse_input(s);
        assert_eq!(f(n, k, e), vec![17, 17, 18]);
    }
}
