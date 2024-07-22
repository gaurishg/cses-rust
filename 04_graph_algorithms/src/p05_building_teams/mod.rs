use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

#[derive(Default, PartialEq, Clone, Copy)]
enum Color {
    #[default]
    NoColor,
    Red,
    Black,
}

fn f(n: usize, friendships: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut colors = vec![Color::default(); n + 1];
    let mut edges = vec![HashSet::new(); n + 1];
    for (i, j) in friendships {
        edges[i].insert(j);
        edges[j].insert(i);
    }

    let edges = edges;

    for i in 1..=n {
        if colors[i] == Color::NoColor {
            let mut q = VecDeque::from([i]);
            colors[i] = Color::Red;
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                let color = colors[u];
                let opp_color = if color == Color::Red {
                    Color::Black
                } else {
                    Color::Red
                };
                for &v in edges[u].iter() {
                    if colors[v] == color {
                        return None;
                    } else if colors[v] == Color::NoColor {
                        colors[v] = opp_color;
                        q.push_back(v);
                    }
                }
            }
        }
    }

    Some(colors.into_iter().skip(1).map(|c| c as usize).collect())
}

pub fn main() {
    let mut s = String::new();
    _ = std::io::stdin().read_to_string(&mut s);

    if let Some(v) = f(
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
    ) {
        for x in v {
            print!("{x} ");
        }
        println!();
    } else {
        println!("IMPOSSIBLE");
    }
}

#[cfg(test)]
mod test_building_teams {
    use super::f;

    #[test]
    fn example() {
        let s = "10 20
3 4
8 10
3 7
1 8
2 8
9 10
2 4
6 9
1 4
3 5
1 9
6 7
2 9
2 7
1 7
4 10
5 6
4 6
3 8
7 10";
        assert_eq!(f(
        s.split_whitespace().nth(0).unwrap().parse().unwrap(),
        s.lines()
            .skip(1)
            .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect(),
        ), Some(vec![1, 1, 1, 2, 2, 1, 2, 2, 2, 1]));
        
    }
}
