use std::{collections::BTreeSet, io::Read};

fn f(customers: Vec<(u32, u32)>) -> (u32, Vec<u32>) {
    let mut customers: Vec<_> = customers
        .into_iter()
        .enumerate()
        .map(|(idx, (come, go))| (come, go, idx))
        .collect();
    customers.sort_unstable();
    let mut available_pool = BTreeSet::new();
    let mut allotments = vec![0; customers.len()];
    let mut occupied: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut largest_room_to_allot = 1;

    for (come, go, idx) in customers {
        while let Some(&(room_left_time, room_number)) = occupied.first() {
            if room_left_time < come {
                available_pool.insert(room_number);
                occupied.pop_first();
            } else {
                break;
            }
        }

        if !available_pool.is_empty() {
            let room_to_allot = available_pool.pop_first().unwrap();
            allotments[idx] = room_to_allot;
            occupied.insert((go, room_to_allot));
        } else {
            allotments[idx] = largest_room_to_allot;
            occupied.insert((go, largest_room_to_allot));
            largest_room_to_allot += 1;
        }
    }

    (*allotments.iter().max().unwrap(), allotments)
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    buf.clear();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let customers = buf
        .lines()
        .map(|s| s.split_whitespace())
        .map(|mut s| {
            let x = s.next().unwrap().parse().unwrap();
            let y = s.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let (max_room, allotments) = f(customers);
    println!("{max_room}");
    for x in allotments {
        print!("{x} ");
    }
    println!();
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use super::f;
    #[test]
    fn test_1() {
        assert_eq!((2, vec![1, 2, 1]), f(vec![(1, 2), (2, 4), (4, 4)]));
    }

    #[test]
    fn test_from_file() {
        let folder_path = std::path::Path::new("..").join(file!());
        let folder_path = folder_path.parent().unwrap();
        let input_file_path = folder_path.join("input.txt");
        let output_file_path = folder_path.join("output.txt");

        let mut input_buf = String::new();
        let _ = std::fs::File::open(input_file_path)
            .unwrap()
            .read_to_string(&mut input_buf);

        let mut output_buf = String::new();
        let _ = std::fs::File::open(output_file_path)
            .unwrap()
            .read_to_string(&mut output_buf);

        let mut input_lines = input_buf.lines();
        let mut output_lines = output_buf.lines();

        let n = input_lines.next().unwrap().parse().unwrap();

        for _ in 1..=n {
            let n = input_lines.next().unwrap().parse().unwrap();
            let mut input = vec![];
            for _ in 0..n {
                let mut line = input_lines.next().unwrap().split_whitespace();
                let come = line.next().unwrap().parse().unwrap();
                let go = line.next().unwrap().parse().unwrap();
                input.push((come, go));
            }
            let correct_max_room = output_lines.next().unwrap().parse().unwrap();
            let correct_alloted_rooms = output_lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!((correct_max_room, correct_alloted_rooms), f(input));
        }
    }
}
