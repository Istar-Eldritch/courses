fn fibonacci_last_digit(n: u64) -> u64 {
    let reduced = n % 60;
    fibonacci_last_digit_impl(reduced)
}

fn fibonacci_last_digit_impl(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 0..(n - 1) {
        let tmp_previous = previous;
        previous = current;
        current = tmp_previous + current;
    }

    current % 10
}

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let input: Vec<u64> = buff
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    println!("{}", fibonacci_last_digit(input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_to_naive() {
        for i in 0..90 {
            let fib = fibonacci_last_digit_impl(i);
            assert_eq!(fib, fibonacci_last_digit(i));
        }
    }

    #[test]
    fn test_failing_3() {
        assert_eq!(1, fibonacci_last_digit(239));
    }

}
