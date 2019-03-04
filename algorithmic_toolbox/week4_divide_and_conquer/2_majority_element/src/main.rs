use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::StdinLock;
use std::iter::Iterator;

struct SplitIterator<'a> {
    iter: &'a mut Iterator<Item = u8>,
}

impl<'a> SplitIterator<'a> {
    fn new(iter: &'a mut Iterator<Item = u8>) -> Self {
        SplitIterator { iter }
    }
}

impl<'a> Iterator for SplitIterator<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let buffer: Vec<u8> = self.iter.take_while(|c| *c != b' ').collect();

        if buffer.len() > 0 {
            Some(buffer)
        } else {
            None
        }
    }
}

fn iterable_input<'a>(stdin: &'a mut StdinLock) -> Box<Iterator<Item = u8> + 'a> {
    Box::new(
        stdin
            .bytes()
            .map(|c| c.unwrap())
            .take_while(|c| *c != b'\n'),
    )
}

#[derive(Debug)]
struct Entity {
    position: usize,
    value: String,
    repetitions: i32,
}

fn main() {
    let size: f64 = {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut iter: Vec<u8> = iterable_input(&mut stdin).collect();
        let buffer = String::from_utf8(iter).unwrap();
        buffer.parse().unwrap()
    };

    if size == 0_f64 {
        println!("{}", 0);
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut iter = iterable_input(&mut stdin);
        let iter = SplitIterator::new(&mut iter);
        let mut memory: HashMap<String, Entity> = HashMap::new();
        let mut position: usize = 1;
        iter.fold(&mut memory, |acc, v| {
            let value = String::from_utf8(v).unwrap();
            acc.get_mut(&value)
                .map(|e| {
                    e.repetitions = e.repetitions + 1;
                })
                .or_else(|| {
                    acc.insert(
                        value.clone(),
                        Entity {
                            position,
                            value,
                            repetitions: 1,
                        },
                    );
                    None
                });
            position = position + 1;
            acc
        });

        let mut count_vec: Vec<(&String, &Entity)> = memory.iter().collect();

        count_vec.sort_by(|a, b| b.1.repetitions.cmp(&a.1.repetitions));

        let result = {
            if count_vec[0].1.repetitions as f64 > (size / 2_f64) {
                1
            } else {
                0
            }
        };
        println!("{:?}", result);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn example_1() {
//         assert_eq!(majority_search(vec![2, 3, 9, 2, 2]), 1);
//     }
//
//     #[test]
//     fn example_2() {
//         assert_eq!(majority_search(vec![1, 2, 3, 4]), 0);
//     }
// }
