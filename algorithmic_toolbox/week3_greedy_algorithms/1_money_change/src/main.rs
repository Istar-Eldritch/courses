fn money_change(n: u64) -> i32 {
    let change: &[f64] = &[10.0, 5.0, 1.0];
    let result = change.iter().fold((n as f64, 0.0), |acc, n| {
        let rest = (acc.0 / n).floor();
        if rest > 0.0 {
            (acc.0 - (rest * n), acc.1 + rest)
        } else {
            acc
        }
    });

    result.1 as i32
}

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let input: Vec<u64> = buff
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    println!("{}", money_change(input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(money_change(2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(money_change(28), 6);
    }
}
