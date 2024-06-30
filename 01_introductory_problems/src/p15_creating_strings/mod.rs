fn make_strings(
    remaining: usize,
    s: &str,
    current_string: &mut String,
    taken: &mut Vec<bool>,
    result: &mut Vec<String>,
) {
    if remaining == 0 {
        result.push(current_string.clone());
        return;
    }

    let mut i = 0;
    while i < s.len() {
        if !taken[i] {
            current_string.push(s.chars().nth(i).unwrap());
            taken[i] = true;
            make_strings(remaining - 1, s, current_string, taken, result);
            current_string.pop();
            taken[i] = false;

            let mut j = i;
            while j < s.len() && s.chars().nth(j).unwrap() == s.chars().nth(i).unwrap() {
                j += 1;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    //for i in 0..taken.len() {
    //    if !taken[i] {
    //        current_string.push(s.chars().nth(i).unwrap());
    //        taken[i] = true;
    //        make_strings(remaining - 1, s, current_string, taken, result);
    //        current_string.pop();
    //        taken[i] = false;
    //    }
    //}
}

fn f(s: &str) -> Vec<String> {
    let mut current_string = String::new();
    let mut result = vec![];
    let mut taken = vec![false; s.len()];

    make_strings(s.len(), s, &mut current_string, &mut taken, &mut result);

    return result;
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);

    let buffer = buffer.trim();
    let mut buffer: Vec<char> = buffer.chars().collect();
    buffer.sort();
    let buffer: String = buffer.iter().collect();

    let ret = f(&buffer);
    println!("{}", ret.len());
    for s in ret {
        println!("{s}");
    }
}

#[cfg(test)]
mod tests {
    use super::f;

    #[test]
    fn test_abc() {
        let input = "abc";
        let expected_output: Vec<String> = vec!["abc", "acb", "bac", "bca", "cab", "cba"]
            .iter()
            .map(|&s| s.into())
            .collect();
        let output = f(input);
        assert_eq!(
            output, expected_output,
            "expected={:?}, got={:?}",
            expected_output, output
        );
    }
}
