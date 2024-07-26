use std::io::Read;

fn f(n_vertices: usize, edges: Vec<(usize, usize, isize)>) -> Option<Vec<usize>> {
    let mut dist = vec![isize::MAX; n_vertices + 1];
    let mut relaxant = (0..=n_vertices).collect::<Vec<_>>();

    let mut start_edge = None;
    for _ in 0..n_vertices {
        start_edge = None;
        for &(u, v, edge_length) in &edges {
            if dist[v] > dist[u].saturating_add(edge_length) {
                relaxant[v] = u;
                dist[v] = dist[u] + edge_length;
                start_edge = Some(v);
            }
        }
    }

    if let Some(mut start_edge) = start_edge {
        for _ in 0..n_vertices {
            start_edge = relaxant[start_edge];
        }
        let mut res = vec![start_edge];
        let mut node = relaxant[start_edge];
        while node != start_edge {
            res.push(node);
            node = relaxant[node];
        }
        res.push(start_edge);
        Some(res.into_iter().rev().collect())
    } else {
        None
    }
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    let (n_vertices, edges) = parse_input(&s);
    if let Some(v) = f(n_vertices, edges) {
        println!("YES");
        for x in v {
            print!("{x} ");
        }
    } else {
        println!("NO");
    }
}

fn parse_input(s: &str) -> (usize, Vec<(usize, usize, isize)>) {
    (
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|l| l.split_whitespace())
            .map(|mut it| {
                (
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                )
            })
            .collect(),
    )
}

#[cfg(test)]
mod test_cycle_finding {
    use super::{f, parse_input};

    #[test]
    #[ignore]
    fn example() {
        let s = "4 5
1 2 1
2 4 1
3 1 1
4 1 -3
4 3 -2";
        let (n, e) = parse_input(s);
        assert_eq!(f(n, e), Some(vec![1, 2, 4, 1]));
    }

    #[test]
    #[ignore]
    fn test_1() {
        let s = "25 50
17 22 68
12 3 93
20 23 92
13 9 -34
25 3 -13
9 8 54
21 2 -8
15 4 26
13 15 8
6 24 39
6 17 -9
21 4 -46
12 6 16
13 22 75
23 5 -49
2 21 33
18 20 -38
4 7 45
21 14 96
5 3 63
9 11 49
1 10 95
8 5 -34
1 23 4
24 7 51
9 18 -6
8 19 46
16 11 6
21 6 71
23 7 6
4 23 44
23 14 38
4 9 0
14 4 -13
13 4 -41
5 13 14
22 25 51
13 21 14
16 4 80
18 5 -38
4 2 22
18 7 75
2 18 -46
6 18 32
6 9 -12
12 5 55
23 24 72
22 3 85
3 2 -14
17 9 55";
        let (n_vertices, edges) = parse_input(s);
        let mut my_res = f(n_vertices, edges).unwrap();
        my_res.sort_unstable();
        let mut correct_res = vec![13, 4, 2, 18, 5, 13];
        correct_res.sort_unstable();
        assert_eq!(my_res, correct_res);
    }
}
