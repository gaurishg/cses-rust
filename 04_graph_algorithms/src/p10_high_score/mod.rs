use std::io::Read;

struct Edge {
    source: usize,
    destination: usize,
    weight: i64,
}

fn dfs(source: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
    visited[source] = true;
    for &dest in adj[source].iter() {
        if !visited[dest] {
            dfs(dest, visited, adj);
        }
    }
}

fn f(n_vertices: usize, mut edges: Vec<Edge>) -> i64 {
    let mut dp = vec![i64::MIN; n_vertices + 1];
    let mut can_be_reached_from_first = vec![false; n_vertices + 1];
    let mut can_reach_last = vec![false; n_vertices + 1];
    let mut adj = vec![vec![]; n_vertices + 1];
    let mut reverse_adj = vec![vec![]; n_vertices + 1];

    for &Edge {
        source,
        destination,
        ..
    } in edges.iter()
    {
        adj[source].push(destination);
        reverse_adj[destination].push(source);
    }

    dp[1] = 0;
    dfs(1, &mut can_be_reached_from_first, &adj);
    dfs(n_vertices, &mut can_reach_last, &reverse_adj);

    edges.retain(
        |&Edge {
             source,
             destination,
             ..
         }| can_be_reached_from_first[source] && can_reach_last[destination],
    );

    for _ in 1..n_vertices {
        for &Edge {
            source,
            destination,
            weight,
        } in edges.iter()
        {
            dp[destination] = dp[destination].max(dp[source].saturating_add(weight));
        }
    }

    for Edge {
        source,
        destination,
        weight,
    } in edges
    {
        if dp[destination] < dp[source].saturating_add(weight) {
            return -1;
        }
    }

    dp[n_vertices]
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let (n_vertices, edges) = parse_input(&s);
    println!("{}", f(n_vertices, edges));
}

fn parse_input(s: &str) -> (usize, Vec<Edge>) {
    (
        s.split_whitespace().next().unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|l| l.split_whitespace())
            .map(|mut it| Edge {
                source: it.next().unwrap().parse().unwrap(),
                destination: it.next().unwrap().parse().unwrap(),
                weight: it.next().unwrap().parse().unwrap(),
            })
            .collect(),
    )
}

#[cfg(test)]
mod test_high_score {
    use crate::p10_high_score::parse_input;

    use super::f;

    #[test]
    fn example() {
        let s = "4 5
1 2 3
2 4 -1
1 3 -2
3 4 7
1 4 4";
        let (n_vertices, edges) = parse_input(s);
        assert_eq!(f(n_vertices, edges), 5);
    }

    #[test]
    fn test_32() {
        let s = "10 10
1 9 -100000000
9 10 1
8 9 1
7 8 1
6 7 1
5 6 1
4 5 1
3 4 1
2 3 1
10 2 1";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), -1);
    }

    #[test]
    fn test_33() {
        let s = "6 8
1 2 3
2 3 -1
3 2 5
1 4 3
4 5 -1
5 4 5
2 6 1
1 6 1000000000";
        let (n_vertices, edges) = parse_input(s);
        assert_eq!(f(n_vertices, edges), -1);
    }

    #[test]
    fn test_5() {
        let s = "4 4
1 2 1
2 3 1
3 2 1
1 4 1";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), 1);
    }

    #[test]
    fn test_9() {
        let s = "4 4
1 4 1
2 4 1
2 3 1
3 2 1
";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), 1);
    }

    #[test]
    fn test_27() {
        let s = "6 8
1 2 -100
1 3 1
2 4 -100
3 4 1
4 5 -100
4 6 1
5 1 -100
6 1 1";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), -1);
    }
}
