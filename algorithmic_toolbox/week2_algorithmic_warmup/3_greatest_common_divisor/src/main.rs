fn gcd(n1: u64, n2: u64) -> u64 {
    if n1 == n2 {
        n1
    } else if n1 == 0 {
        n2
    } else if n2 == 0 {
        n1
    } else if n1 % 2 == 0 && n2 % 2 == 0 {
        // If u and v are both even, then gcd(u, v) = 2·gcd(u/2, v/2), because 2 is a common divisor.
        2 * gcd(n1 / 2, n2 / 2)
    // If u is even and v is odd, then gcd(u, v) = gcd(u/2, v), because 2 is not a common divisor. Similarly, if u is odd and v is even, then gcd(u, v) = gcd(u, v/2).
    } else if n1 % 2 == 0 {
        gcd(n1 / 2, n2)
    } else if n2 % 2 == 0 {
        gcd(n1, n2 / 2)
    } else if n1 > n2 {
        gcd((n1 - n2) / 2, n2)
    } else if n1 < n2 {
        gcd((n2 - n1) / 2, n1)
    } else {
        1
    }
}

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let input: Vec<u64> = buff
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    println!("{}", gcd(input[0], input[1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(gcd(18, 35), 1);
    }
}
