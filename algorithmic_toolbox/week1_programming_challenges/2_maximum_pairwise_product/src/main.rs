use std::iter::Iterator;

struct BigOperands {
    op_1: i64,
    op_2: i64,
}

impl BigOperands {
    fn new() -> Self {
        BigOperands { op_1: 0, op_2: 0 }
    }

    fn insert(&self, operand: i64) -> Self {
        if operand > self.op_1 || operand > self.op_2 {
            if self.op_1 < self.op_2 {
                BigOperands {
                    op_1: operand,
                    ..*self
                }
            } else {
                BigOperands {
                    op_2: operand,
                    ..*self
                }
            }
        } else {
            BigOperands { ..*self }
        }
    }

    fn compute(&self) -> i64 {
        self.op_1 * self.op_2
    }
}

pub fn max_pairwise_product(numbers: &mut Iterator<Item = i64>) -> i64 {
    let operands = numbers.fold(BigOperands::new(), |acc, elem| acc.insert(elem));
    return operands.compute();
}

fn main() {
    let mut to_read = String::new();
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut to_read).unwrap();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let words = buff.split_whitespace();

    let numbers: &mut Iterator<Item = i64> = &mut words.map(|ch| ch.parse().unwrap());
    println!("{}", max_pairwise_product(numbers));
}

#[cfg(test)]
mod tests {
    use super::max_pairwise_product;

    #[test]
    fn single_element_returns_0() {
        let mut v = [1_i64].iter().map(|n| *n);
        assert_eq!(0, max_pairwise_product(&mut v));
    }

    #[test]
    fn multiply_elements() {
        assert!(true);
    }

    #[test]
    fn exercise_example() {
        let v = &mut [7, 5, 14, 2, 8, 8, 10, 1, 2, 3].iter().map(|e| *e);
        assert_eq!(140, max_pairwise_product(v));
    }

    #[test]
    fn exercise_failing_test3() {
        let v = &mut [100000, 90000].iter().map(|e| *e);
        assert_eq!(9000000000, max_pairwise_product(v));
    }
}
