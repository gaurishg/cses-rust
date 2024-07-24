use std::{io::Read, usize};

struct Edge {
    source: usize,
    target: usize,
    weight: isize,
}

fn f(n_vertices: usize, roads: Vec<Edge>, queries: Vec<(usize, usize)>) -> Vec<isize> {
    let mut dp = vec![vec![isize::MAX; n_vertices + 1]; n_vertices + 1];
    for Edge {
        source,
        target,
        weight,
    } in roads
    {
        dp[source][target] = dp[source][target].min(weight);
        dp[target][source] = dp[target][source].min(weight);
    }

    for i in 1..=n_vertices {
        dp[i][i] = 0;
    }

    for middle_vertex in 1..=n_vertices {
        for source in 1..=n_vertices {
            for target in 1..=n_vertices {
                if dp[source][target]
                    > dp[source][middle_vertex].saturating_add(dp[middle_vertex][target])
                {
                    dp[source][target] = dp[source][middle_vertex] + dp[middle_vertex][target];
                }
            }
        }
    }

    queries
        .into_iter()
        .map(|(source, dest)| {
            if dp[source][dest] == isize::MAX {
                -1
            } else {
                dp[source][dest]
            }
        })
        .collect()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let (n_vertices, edges, queries) = parse_input(&s);

    for x in f(n_vertices, edges, queries) {
        print!("{x} ");
    }
}

fn parse_input(s: &str) -> (usize, Vec<Edge>, Vec<(usize, usize)>) {
    let n_vertices = s.split_whitespace().next().unwrap().parse().unwrap();
    let n_edges = s.split_whitespace().nth(1).unwrap().parse().unwrap();
    let edges = s
        .lines()
        .skip(1)
        .take(n_edges)
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()))
        .map(|mut it| Edge {
            source: it.next().unwrap(),
            target: it.next().unwrap(),
            weight: it.next().unwrap() as isize,
        })
        .collect();
    let queries = s
        .lines()
        .skip(n_edges + 1)
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()))
        .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
        .collect();

    (n_vertices, edges, queries)
}

#[cfg(test)]
mod test_shortest_routes_ii {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "4 3 5
1 2 5
1 3 9
2 3 3
1 2
2 1
1 3
1 4
3 2";

        let (n_vertices, edges, queries) = parse_input(s);
        let correct_answer: Vec<isize> = "5
5
8
-1
3"
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

        assert_eq!(f(n_vertices, edges, queries), correct_answer);
    }
}
