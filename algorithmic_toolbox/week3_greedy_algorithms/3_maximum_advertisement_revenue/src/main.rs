fn parse(input: &String) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect()
}

fn max_permutation(len: i64, input1: &mut Vec<i64>, input2: &mut Vec<i64>) -> i64 {
    let len = len as usize;
    input1.sort();
    input2.sort();
    (0..len).fold(0, |acc, index| acc + input1[index] * input2[index])
}

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let len = parse(&buff)[0];
    buff.truncate(0);

    ::std::io::stdin().read_line(&mut buff).unwrap();
    let mut input1 = parse(&buff);
    buff.truncate(0);

    ::std::io::stdin().read_line(&mut buff).unwrap();
    let mut input2 = parse(&buff);
    buff.truncate(0);

    println!("{}", max_permutation(len, &mut input1, &mut input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_examples() {
        assert_eq!(max_permutation(1, &mut vec![23], &mut vec![39]), 897);
        assert_eq!(
            max_permutation(3, &mut vec![1, 3, -5], &mut vec![-2, 4, 1]),
            23
        )
    }
}
