use std::io::Read;

#[derive(PartialEq, Eq, Clone, Copy)]
enum VisitedState {
    NotVisited,
    InProgress,
    Done,
}

fn topological_sort(
    u: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<VisitedState>,
    result: &mut Vec<usize>,
) -> bool {
    if visited[u] == VisitedState::Done {
        return true;
    } else if visited[u] == VisitedState::InProgress {
        return false;
    }

    visited[u] = VisitedState::InProgress;
    for &child in &adj[u] {
        if !topological_sort(child, adj, visited, result) {
            return false;
        }
    }
    visited[u] = VisitedState::Done;
    result.push(u);
    return true;
}

fn f(n_vertices: usize, edges: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertices + 1];
    for (u, v) in edges {
        adj[v].push(u);
    }
    let adj = adj;
    let mut visited = vec![VisitedState::NotVisited; n_vertices + 1];
    let mut result = vec![];

    for u in 1..=n_vertices {
        if !topological_sort(u, &adj, &mut visited, &mut result) {
            return None;
        }
    }

    Some(result)
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
    let (n_vertices, edges) = parse_input(&s);
    if let Some(v) = f(n_vertices, edges) {
        for x in v {
            print!("{x} ");
        }
    } else {
        println!("IMPOSSIBLE");
    }
}

#[cfg(test)]
mod test_course_schedule {
    use super::{f, parse_input};

    #[test]
    #[ignore]
    fn example() {
        let s = "5 3
1 2
3 1
4 5";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), Some(vec![3, 4, 1, 5, 2]));
    }
}
