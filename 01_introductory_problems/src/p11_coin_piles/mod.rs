use std::io::Read;

fn f(a: u64, b: u64) -> bool {
    let (a, b) = (std::cmp::min(a, b), std::cmp::max(a, b));
    if a == 0 && b == 0 {
        return true;
    } else if a == 0 || b == 0 {
        return false;
    } else if (a + b) % 3 != 0 {
        return false;
    } else if a == b {
        return a % 3 == 0;
    } else if 2 * a < b {
        return false;
    }

    return f(a - (b - a), b - 2 * (b - a));
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let mut lines = buffer.lines();
    let n = lines.next().unwrap().parse().unwrap();
    for _ in 1..=n {
        let inputs: Vec<_> = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let a = inputs[0];
        let b = inputs[1];
        println!("{}", if f(a, b) { "YES" } else { "NO" });
    }
}

#[cfg(test)]
mod test {
    use std::io::BufRead;

    use super::f;

    #[test]
    fn test_from_file() {
        let basepath = std::path::Path::new("..").join(file!());
        let basepath = basepath.parent().unwrap();

        let inputfile = std::fs::File::open(basepath.join("input.txt")).unwrap();
        let outputfile = std::fs::File::open(basepath.join("output.txt")).unwrap();
        let mut input_lines = std::io::BufReader::new(inputfile).lines();
        let mut output_lines = std::io::BufReader::new(outputfile).lines();
        let n = input_lines.next().unwrap().unwrap().parse().unwrap();

        for _ in 1..=n {
            let inputs: Vec<u64> = input_lines
                .next()
                .unwrap()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let output = output_lines.next().unwrap().unwrap() == "YES";
            assert_eq!(f(inputs[0], inputs[1]), output);
        }
    }
}
