use std::io::Read;

// f_max_len(u) = 1 + for all child f_max_len(child)

fn dfs(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    if !visited[u] {
        visited[u] = true;
        for &v in &adj[u] {
            dfs(v, adj, visited);
        }
    }
}

fn max_route_len_from_node(
    u: usize,
    adj: &Vec<Vec<usize>>,
    max_lengths: &mut Vec<Option<usize>>,
    next_city: &mut Vec<usize>,
    can_reach_last_node: &Vec<bool>,
) -> Option<usize> {
    if max_lengths[u].is_some() {
        return max_lengths[u];
    }

    for &v in &adj[u] {
        if !can_reach_last_node[v] {
            continue;
        }
        if let Some(max_len_from_v) =
            max_route_len_from_node(v, adj, max_lengths, next_city, can_reach_last_node)
        {
            if max_lengths[u].is_none() || 1 + max_len_from_v > max_lengths[u].unwrap() {
                max_lengths[u] = Some(1 + max_len_from_v);
                next_city[u] = v;
            }
        }
    }

    max_lengths[u]
}
fn f(n_vertices: usize, edges: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertices + 1];
    let mut reverse_adj = vec![vec![]; n_vertices + 1];
    for (u, v) in edges {
        adj[u].push(v);
        reverse_adj[v].push(u);
    }
    let adj = adj;
    let reverse_adj = reverse_adj;

    let mut can_reach_the_last_node = vec![false; n_vertices + 1];
    dfs(n_vertices, &reverse_adj, &mut can_reach_the_last_node);

    let mut max_lengths = vec![None; n_vertices + 1];
    max_lengths[n_vertices] = Some(1);
    let mut next_city = vec![usize::MAX; n_vertices + 1];

    if let Some(_) = max_route_len_from_node(
        1,
        &adj,
        &mut max_lengths,
        &mut next_city,
        &can_reach_the_last_node,
    ) {
        let mut v = vec![];
        let mut city = 1;
        while city != n_vertices {
            v.push(city);
            city = next_city[city];
        }
        v.push(n_vertices);
        Some(v)
    } else {
        None
    }
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);
    let (n_vertices, edges) = parse_input(&s);

    if let Some(v) = f(n_vertices, edges) {
        println!("{}", v.len());
        for x in v {
            print!("{x} ");
        }
    } else {
        println!("IMPOSSIBLE");
    }
}

fn parse_input(s: &str) -> (usize, Vec<(usize, usize)>) {
    (
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|l| l.split_whitespace())
            .map(|sp| sp.map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    )
}

#[cfg(test)]
mod test_longest_flight_route {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "5 5
1 2
2 5
1 3
3 4
4 5";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), Some(vec![1, 3, 4, 5]));
    }
}
