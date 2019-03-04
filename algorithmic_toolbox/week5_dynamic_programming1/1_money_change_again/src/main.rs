use std::collections::HashMap;
use std::i32::MAX;

fn min_change(money: i32) -> i32 {
    let denominators = vec![4, 3, 1];

    fn helper<'a>(
        money: i32,
        denominators: &Vec<i32>,
        mem: &'a mut HashMap<i32, Vec<i32>>,
    ) -> Option<Vec<i32>> {
        let result = mem.get(&money).map(|n| n.clone());

        match result {
            Some(v) => Some(v.clone()),
            None if money == 0 => Some(vec![]),
            None if money > 0 => {
                let result = denominators
                    .iter()
                    .map(|denominator| {
                        let d_money = money - denominator;
                        helper(d_money, denominators, mem).map(|v| {
                            let mut mv = v.clone();
                            mv.push(*denominator);
                            mv
                        })
                    })
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
                    .fold((MAX, Vec::new()), |(vl, v), n| {
                        let nl = n.len() as i32;
                        if nl < vl {
                            (nl, n)
                        } else {
                            (vl, v)
                        }
                    })
                    .1;
                mem.insert(money, result.clone());
                Some(result)
            }
            None => None,
        }
    }

    let mut mem = HashMap::new();
    let result = helper(money, &denominators, &mut mem).unwrap();
    result.len() as i32
}

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff).unwrap();
    let money: i32 = buff.split_whitespace().next().unwrap().parse().unwrap();

    println!("{}", min_change(money));
}
