use std::collections::HashMap;

fn fibonacci_impl(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    let i = cache.get(&n).map(|n| *n);
    match i {
        Some(v) => v,
        None if n <= 1 => n,
        None => {
            let v1 = fibonacci_impl(n - 1, cache);
            let v2 = fibonacci_impl(n - 2, cache);
            let result = v1 + v2;
            cache.insert(n, result);
            result
        }
    }
    // if n <= 1 {
    //     return n;
    // }
    // if let Some(v) = cache.get(&n) {
    //     return *v;
    // }
    // let v1 = fibonacci_impl(n - 1, cache);
    // let v2 = fibonacci_impl(n - 2, cache);
    // let result = v1 + v2;
    // cache.insert(n, result);
    // result
}

fn fibonacci(n: u64) -> u64 {
    fibonacci_impl(n, &mut HashMap::new())
}

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let input: Vec<u64> = buff
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    println!("{}", fibonacci(input[0]));
}

#[cfg(tests)]
mod tests {
    use super::*;

}
