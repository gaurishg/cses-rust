use std::io::Read;

fn dfs(
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    cycle: &mut Vec<usize>,
    current_node: usize,
    parent_node: usize,
) -> bool {
    cycle.push(current_node);
    if visited[current_node] {
        return true;
    }

    visited[current_node] = true;
    for &next_node in edges[current_node].iter() {
        if next_node != parent_node && dfs(edges, visited, cycle, next_node, current_node) {
            return true;
        }
    }
    cycle.pop();
    false
}

fn f(n: usize, roads: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut edges = vec![vec![]; n + 1];
    for (i, j) in roads {
        edges[i].push(j);
        edges[j].push(i);
    }

    let mut cycle = Vec::new();
    let mut visited = vec![false; n + 1];

    for i in 1..=n {
        if !visited[i] {
            if dfs(&edges, &mut visited, &mut cycle, i, 0) {
                let last_item = cycle.last().copied().unwrap();
                return Some(cycle.into_iter().skip_while(|x| *x != last_item).collect());
            }
        }
    }

    None
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    if let Some(v) = f(
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    ) {
        println!("{}", v.len());
        for x in v {
            print!("{x} ");
        }
        println!();
    } else {
        println!("IMPOSSIBLE");
    }
}

#[cfg(test)]
mod test_round_trip {
    use super::f;

    #[test]
    #[ignore]
    fn example() {
        let s = "5 6
1 3
1 2
5 3
1 5
2 4
4 5";
        let res = f(
            s.split_whitespace().nth(0).unwrap().parse().unwrap(),
            s.lines()
                .skip(1)
                .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()))
                .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
                .collect(),
        );
        assert_eq!(res, Some(vec![3, 5, 1, 3]));
    }
}
