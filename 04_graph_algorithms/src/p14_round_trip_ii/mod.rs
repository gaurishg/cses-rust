use std::{collections::HashMap, io::Read};

fn f(n_vertices: usize, flights: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertices + 1];
    for (u, v) in flights {
        adj[u].push(v);
    }
    let adj = adj;
    let mut visited = vec![0; n_vertices + 1];
    let mut stack = Vec::new();
    let mut parents = vec![usize::MAX; n_vertices + 1];

    let mut start_idx = 1;
    for start_node in 1..=n_vertices {
        if visited[start_node] != 0 {
            continue;
        }
        stack.push(start_node);
        let mut counter = 0;
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            if visited[node] != 0 {
                if visited[node] < start_idx {
                    continue;
                }
                let mut res = vec![node];
                let mut prev_idx = HashMap::new();
                prev_idx.insert(node, 0);
                let mut current_node = parents[node];
                for idx in 1..n_vertices {
                    res.push(current_node);
                    if let Some(last_idx) = prev_idx.insert(current_node, idx) {
                        return Some(res.into_iter().skip(last_idx).rev().collect());
                    }
                    current_node = parents[current_node];
                }
            }
            visited[node] = start_idx + counter;
            counter += 1;

            for &child in &adj[node] {
                if visited[child] != 0 && visited[child] < start_idx {
                    continue;
                }
                stack.push(child);
                parents[child] = node;
            }
        }
        start_idx += counter;
    }

    None
}

fn parse_input(s: &str) -> (usize, Vec<(usize, usize)>) {
    (
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    )
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let (n_vertices, flights) = parse_input(&s);
    if let Some(v) = f(n_vertices, flights) {
        println!("{}", v.len());
        for x in v {
            print!("{x} ");
        }
    } else {
        println!("IMPOSSIBLE");
    }
}

#[cfg(test)]
mod test_round_trip_ii {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "4 5
1 3
2 1
2 4
3 2
3 4";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), Some(vec![2, 1, 3, 2]));
    }

    #[test]
    fn test_16() {
        let s = "5 6
1 5
1 2
1 3
3 4
4 2
2 5";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), None);
    }
}
