use std::{io::Read, usize};

struct Edge {
    source: usize,
    target: usize,
    weight: usize,
}

fn f(n_vertices: usize, edges: Vec<Edge>) -> Vec<usize> {
    let mut costs = vec![usize::MAX; n_vertices];
    costs[0] = 0;

    for _ in 0..n_vertices - 1 {
        let mut anything_changed = false;
        for edge in edges.iter() {
            let u = edge.source;
            let v = edge.target;
            let edge_weight = edge.weight;
            if costs[u] != usize::MAX && costs[u] + edge_weight < costs[v] {
                costs[v] = costs[u] + edge_weight;
                anything_changed = true;
            }
        }

        if !anything_changed {
            break;
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
}
