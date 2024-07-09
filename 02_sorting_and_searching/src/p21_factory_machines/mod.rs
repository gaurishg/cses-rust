fn is_possible(items_to_make: u64, machine_times: &Vec<u64>, allowed_time: u64) -> bool {
    let mut items_made = 0;
    for time_taken_by_machine in machine_times {
        items_made += allowed_time / time_taken_by_machine;
        if items_made >= items_to_make {
            return true;
        }
    }
    return false;
}

fn f(items_to_make: u64, machine_times: Vec<u64>) -> u64 {
    let max_time_taken = machine_times.iter().max().unwrap();
    let mut l = 1;
    let mut r = 1 + max_time_taken * items_to_make;

    while r - l >= 3 {
        let time = l + (r - l) / 2;
        if is_possible(items_to_make, &machine_times, time) {
            //eprintln!("{time} is possible, l={l}, r={r}");
            r = time;
        } else {
            //eprintln!("{time} is NOT possible, l={l}, r={r}");
            l = time + 1;
        }
    }

    for time in l..=r {
        if is_possible(items_to_make, &machine_times, time) {
            return time;
        }
    }
    return u64::MAX;
}

pub fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);

    let t;
    {
        let mut it = buf.trim().split_whitespace();
        it.next();
        t = it.next().unwrap().parse().unwrap();
    }

    buf.clear();
    let _ = std::io::stdin().read_line(&mut buf);
    let machines = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    println!("{}", f(t, machines));
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use super::f;

    #[test]
    fn test_from_file() {
        let folder_path = std::path::Path::new("..").join(file!());
        let folder_path = folder_path.parent().unwrap();

        let input_path = folder_path.join("input.txt");
        let output_path = folder_path.join("output.txt");

        let mut input_buf = String::new();
        let mut output_buf = String::new();

        let _ = std::fs::File::open(input_path)
            .unwrap()
            .read_to_string(&mut input_buf);
        let _ = std::fs::File::open(output_path)
            .unwrap()
            .read_to_string(&mut output_buf);

        let mut input_lines = input_buf.lines();
        let mut output_lines = output_buf.lines();

        let n_tests = input_lines.next().unwrap().parse().unwrap();
        for _ in 0..n_tests {
            let mut line_1 = input_lines.next().unwrap().split_whitespace();
            line_1.next();
            let items_to_produce = line_1.next().unwrap().parse().unwrap();
            let machines = input_lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let correct_output: u64 = output_lines.next().unwrap().parse().unwrap();

            assert_eq!(correct_output, f(items_to_produce, machines));
        }
    }
}
