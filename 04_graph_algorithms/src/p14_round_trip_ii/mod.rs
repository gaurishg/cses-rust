use std::io::Read;

#[derive(PartialEq, Eq, Clone, Copy)]
enum VisitedState {
    NotVisited,
    InProgress,
    Done,
}

fn dfs(
    i: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<VisitedState>,
    v: &mut Vec<usize>,
) -> bool {
    if visited[i] == VisitedState::Done {
        return false;
    } else if visited[i] == VisitedState::InProgress {
        v.push(i);
        return true;
    }

    visited[i] = VisitedState::InProgress;
    v.push(i);

    for &child in &adj[i] {
        if dfs(child, adj, visited, v) {
            return true;
        }
    }

    visited[i] = VisitedState::Done;
    v.pop();
    false
}

fn f(n_vertices: usize, flights: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut visited = vec![VisitedState::NotVisited; n_vertices + 1];
    let mut v = vec![];
    let mut adj = vec![vec![]; n_vertices + 1];
    for (u, v) in flights {
        adj[u].push(v);
    }
    let adj = adj;

    for i in 1..=n_vertices {
        if dfs(i, &adj, &mut visited, &mut v) {
            let &first_element = v.last().unwrap();
            return Some(v.into_iter().skip_while(|&x| x != first_element).collect());
        }
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
    fn test_1() {
        let s = "10 20
1 3
7 4
5 4
9 6
4 7
6 9
10 8
4 8
8 3
3 10
2 8
9 5
10 2
9 8
10 9
1 10
3 6
4 5
8 10
3 2";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), Some(vec![3, 10, 8, 3]));
    }

    #[test]
    fn test_2() {
        let s = "10 20
1 7
2 8
10 5
4 8
7 10
6 4
9 10
7 2
6 3
4 7
9 3
2 5
4 3
8 9
7 1
5 10
7 6
8 1
8 2
6 7";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), Some(vec![10, 5, 10]));
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
