use std::{collections::HashSet, io::Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    NotVisited,
    InProgress,
    Done,
}

fn dfs(
    edges: &Vec<HashSet<usize>>,
    states: &mut Vec<State>,
    origins: &mut Option<Vec<usize>>,
    i: usize,
    v: &mut Vec<usize>,
) {
    if states[i] == State::InProgress {
        let mut res = vec![i];
        res.extend(v.iter().rev().take_while(|&&x| x != i));
        res.push(i);
        if res.len() >= 4 {
            *origins = Some(res);
        }
        return;
    }

    states[i] = State::InProgress;
    v.push(i);
    for &j in edges[i].iter() {
        dfs(edges, states, origins, j, v);
        if origins.is_some() {
            break;
        }
    }
    v.pop();
    states[i] = State::Done;
}

fn f(n: usize, roads: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut edges = vec![HashSet::new(); n + 1];
    for (i, j) in roads {
        edges[i].insert(j);
        edges[j].insert(i);
    }

    let mut states = vec![State::NotVisited; n + 1];
    let mut origins = None;
    let mut v = vec![];

    for i in 1..=n {
        if states[i] == State::NotVisited {
            dfs(&edges, &mut states, &mut origins, i, &mut v);
            if origins.is_some() {
                return origins;
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
