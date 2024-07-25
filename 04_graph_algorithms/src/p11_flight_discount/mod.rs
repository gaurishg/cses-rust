use std::{collections::BinaryHeap, io::Read};

fn f(n_vertices: usize, flights: Vec<(usize, usize, usize)>) -> usize {
    let adj = {
        let mut adj = vec![vec![]; n_vertices + 1];
        for (from, to, cost) in flights {
            adj[from].push((to, cost));
        }

        adj
    };

    let mut heap = BinaryHeap::new();
    heap.push((std::cmp::Reverse(0), 1, 1));
    let mut visited = vec![vec![false; n_vertices + 1]; 2];

    while !heap.is_empty() {
        let (std::cmp::Reverse(cost), node, coupons_available) = heap.pop().unwrap();
        if node == n_vertices {
            return cost;
        }
        if visited[coupons_available][node] {
            continue;
        }

        visited[coupons_available][node] = true;

        for &(child, weight) in adj[node].iter() {
            if visited[coupons_available][child] {
                continue;
            }
            if coupons_available == 1 {
                heap.push((std::cmp::Reverse(cost + weight), child, 1));
                heap.push((std::cmp::Reverse(cost + weight / 2), child, 0));
            } else {
                heap.push((std::cmp::Reverse(cost + weight), child, 0));
            }
        }
    }

    unreachable!()
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let (n_vertices, flights) = parse_input(&s);
    println!("{}", f(n_vertices, flights));
}

fn parse_input(s: &str) -> (usize, Vec<(usize, usize, usize)>) {
    (
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap(), it.next().unwrap()))
            .collect(),
    )
}

#[cfg(test)]
mod test_flight_discount {
    use super::{f, parse_input};

    #[test]
    fn example() {
        let s = "3 4
1 2 3
2 3 1
1 3 7
2 1 5";
        let (n, fl) = parse_input(s);
        assert_eq!(f(n, fl), 2);
    }
}
