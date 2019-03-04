#[derive(Debug, Clone)]
enum Operation {
    Minus,
    Plus,
    Multiplication,
}

enum ParseResult {
    Op(Operation),
    Value(i64),
}
fn parse_byte(b: u8) -> ParseResult {
    match b as char {
        b if b >= '0' && b <= '9' => ParseResult::Value(b as i64 - 48),
        '+' => ParseResult::Op(Operation::Plus),
        '-' => ParseResult::Op(Operation::Minus),
        '*' => ParseResult::Op(Operation::Multiplication),
        b => panic!("Unrecognized symbol {}", b),
    }
}

fn apply(v1: i64, v2: i64, op: &Operation) -> i64 {
    match *op {
        Operation::Plus => v1 + v2,
        Operation::Minus => v1 - v2,
        Operation::Multiplication => v1 * v2,
    }
}

fn min_and_max(
    min_mem: &Vec<Vec<i64>>,
    max_mem: &Vec<Vec<i64>>,
    operations: &Vec<Operation>,
    i: usize,
    j: usize,
) -> (i64, i64) {
    let mut min = i64::max_value();
    let mut max = i64::min_value();
    for k in i..j {
        let opk = &operations[k];
        let a = apply(max_mem[i][k], max_mem[k + 1][j], opk);
        let b = apply(max_mem[i][k], min_mem[k + 1][j], opk);
        let c = apply(min_mem[i][k], max_mem[k + 1][j], opk);
        let d = apply(min_mem[i][k], min_mem[k + 1][j], opk);
        min = min.min(a).min(b).min(c).min(d);
        max = max.max(a).max(b).max(c).max(d);
    }
    (min, max)
}

fn print_matrix(matrix: Vec<Vec<i64>>) {
    for i in matrix {
        for j in i {
            print!("{} ", j);
        }
        print!("\n");
    }
}

fn parentheses(val: Vec<i64>, op: Vec<Operation>) -> i64 {
    let mut min_mem: Vec<Vec<i64>> = Vec::with_capacity(val.len());
    let mut max_mem: Vec<Vec<i64>> = Vec::with_capacity(val.len());

    for i in 0..val.len() {
        min_mem.push(vec![0; val.len()]);
        min_mem[i][i] = val[i];
        max_mem.push(vec![0; val.len()]);
        max_mem[i][i] = val[i];
    }

    for s in 1..val.len() {
        for i in 0..val.len() - s {
            let j = i + s;
            let (min, max) = min_and_max(&min_mem, &max_mem, &op, i, j);
            min_mem[i][j] = min;
            max_mem[i][j] = max;
        }
    }
    max_mem[0][val.len() - 1]
}

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let input: (Vec<i64>, Vec<Operation>) = buff
        .bytes()
        .filter(|&c| !(c as char).is_whitespace())
        .fold((Vec::new(), Vec::new()), |mut acc, c| {
            match parse_byte(c) {
                ParseResult::Value(v) => acc.0.push(v),
                ParseResult::Op(o) => acc.1.push(o),
            };
            acc
        });

    //dbg!(input.clone());
    println!("{}", parentheses(input.0, input.1));
}
