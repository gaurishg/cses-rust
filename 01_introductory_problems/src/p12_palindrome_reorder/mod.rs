fn f(s: &str) -> String {
    let mut freqs = [0; 256];
    for c in s.chars() {
        freqs[c as usize] += 1;
    }
    if freqs.iter().filter(|x| **x % 2 == 1).count() > 1 {
        return "NO SOLUTION".to_owned();
    }

    let mut result = String::new();
    let mut odd_char = '\0';
    for (i, &freq) in freqs.iter().enumerate() {
        result.push_str(
            vec![char::from_u32(i as u32).unwrap(); freq / 2]
                .iter()
                .collect::<String>()
                .as_str(),
        );
        if freq % 2 == 1 {
            odd_char = char::from_u32(i as u32).unwrap();
        }
    }
    if odd_char != '\0' {
        result.push(odd_char);
    }
    for (i, &freq) in freqs.iter().enumerate().rev() {
        result.push_str(
            vec![char::from_u32(i as u32).unwrap(); freq / 2]
                .iter()
                .collect::<String>()
                .as_str(),
        );
    }
    result
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    println!("{}", f(buffer.trim()));
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    #[test]
    fn test_from_file() {
        let filename = std::path::Path::new("..")
            .join(file!())
            .parent()
            .unwrap()
            .join("input.txt");
        let file = std::fs::File::open(filename).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();
        let n = lines.next().unwrap().unwrap().parse().unwrap();
        for _ in 1..=n {
            let s = lines.next().unwrap().unwrap();
            let s = super::f(&s);
            assert!(s.chars().eq(s.chars().rev()), "Failed for {}", s);
        }
    }
}
