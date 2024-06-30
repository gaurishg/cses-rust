fn f(
    n_applicants: usize,
    n_apartments: usize,
    max_diff: isize,
    mut applicants: Vec<isize>,
    mut apartments: Vec<isize>,
) -> i32 {
    applicants.sort();
    apartments.sort();

    let mut i_applicant = 0;
    let mut i_apartment = 0;
    let mut total = 0;

    while i_applicant < n_applicants && i_apartment < n_apartments {
        while i_applicant < n_applicants
            && apartments[i_apartment] - applicants[i_applicant] > max_diff
        {
            i_applicant += 1;
        }

        if i_applicant < n_applicants
            && applicants[i_applicant] - apartments[i_apartment] <= max_diff
        {
            total += 1;
            i_applicant += 1;
        }

        i_apartment += 1;
    }
    total
}

pub fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let mut nums_split = buffer.split_ascii_whitespace();
    let n_applicants = nums_split.next().unwrap().parse().unwrap();
    let n_apartments = nums_split.next().unwrap().parse().unwrap();
    let max_diff = nums_split.next().unwrap().parse().unwrap();

    buffer.clear();
    let _ = std::io::stdin().read_line(&mut buffer);
    let applicants: Vec<isize> = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buffer.clear();
    let _ = std::io::stdin().read_line(&mut buffer);
    let apartments = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<isize>>();

    println!(
        "{}",
        f(n_applicants, n_apartments, max_diff, applicants, apartments)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, f(4, 3, 5, vec![60, 45, 80, 60], vec![30, 60, 75]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, f(5, 2, 2, vec![2, 2, 2, 40, 50], vec![40, 50]));
    }
}
