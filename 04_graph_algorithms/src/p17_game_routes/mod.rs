use std::io::Read;

fn ways(node: usize, adj: &Vec<Vec<usize>>, dp: &mut Vec<isize>) -> isize {
    if dp[node] >= 0 {
        return dp[node];
    }

    dp[node] = 0;
    for &child in &adj[node] {
        dp[node] = (dp[node] + ways(child, adj, dp)) % 1_000_000_007;
    }
    return dp[node];
}

fn f(n_vertices: usize, edges: Vec<(usize, usize)>) -> isize {
    let mut adj = vec![vec![]; n_vertices + 1];
    for (u, v) in edges {
        adj[u].push(v);
    }
    let adj = adj;

    let mut dp = vec![-1; n_vertices + 1];
    dp[n_vertices] = 1;
    ways(1, &adj, &mut dp)
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    let (n_vertices, edges) = parse_input(&s);
    println!("{}", f(n_vertices, edges));
}

fn parse_input(s: &str) -> (usize, Vec<(usize, usize)>) {
    (
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    )
}

#[cfg(test)]
mod test_game_routes {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "4 5
1 2
2 4
1 3
3 4
1 4";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), 3);
    }
}
