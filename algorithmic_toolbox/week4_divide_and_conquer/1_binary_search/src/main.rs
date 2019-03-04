use std::cmp::Ordering;
use std::fmt::Debug;
use std::io;
use std::io::prelude::*;
use std::io::StdinLock;
use std::iter::Iterator;

trait BinarySearchable<T>
where
    T: Eq,
{
    fn binary_search(&self, item: &T) -> Result<usize, usize>;
}

fn binary_search_impl<T>(items: &[T], item: &T) -> Result<usize, usize>
where
    T: Eq + Ord + Debug,
{
    let len = items.len();
    if len == 0 {
        return Err(0);
    }

    let mut left = 0;
    let mut right = len;

    loop {
        if right == left {
            return Err(left);
        }
        let i = (right - left) / 2 + left;
        return match item.cmp(&items[i]) {
            Ordering::Equal => Ok(i),
            Ordering::Less => {
                if i == left {
                    return Err(left);
                }
                right = i;
                continue;
            }
            Ordering::Greater => {
                if i == right {
                    return Err(right);
                }
                if right - left == 1 {
                    left = right
                } else {
                    left = i;
                }
                continue;
            }
        };
    }
}

impl<T> BinarySearchable<T> for Vec<T>
where
    T: Eq + Ord + Debug,
{
    fn binary_search(&self, item: &T) -> Result<usize, usize> {
        binary_search_impl(self, item)
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

fn main() {
    let mut input: Vec<i32> = {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut iter = iterable_input(&mut stdin);
        let iter = SplitIterator::new(&mut iter);
        iter.map(|v| String::from_utf8(v).unwrap().parse().unwrap())
            .collect()
    };
    input.remove(0);

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut iter = iterable_input(&mut stdin);
    let mut iter = SplitIterator { iter: &mut iter };
    let _ = iter.next();
    iter.map(|v| {
        match binary_search_impl(&mut input, &String::from_utf8(v).unwrap().parse().unwrap()) {
            Ok(v) => format!("{} ", v),
            Err(_) => format!("{} ", -1),
        }
    })
    .for_each(|v| {
        let bytes: Vec<u8> = v.bytes().collect();
        io::stdout().write(&bytes).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_empty() {
        let items = vec![];
        let result = items.binary_search(&1);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), 0);
    }

    #[test]
    fn not_found_first() {
        let items = vec![2];
        let result = items.binary_search(&1);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), 0);
    }

    #[test]
    fn find_first() {
        let items = vec![1];
        let result = items.binary_search(&1);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), 0);
    }

    #[test]
    fn not_found_second() {
        let items = vec![1];
        let result = items.binary_search(&2);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), 1);
    }

    #[test]
    fn not_found_f() {
        let items = vec![1, 2];
        let result = items.binary_search(&0);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), 0);
    }

    #[test]
    fn find_second() {
        let items = vec![1, 2];
        let result = items.binary_search(&2);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), 1);
    }

    #[test]
    fn last_not_found() {
        let items = vec![1, 2];
        let result = items.binary_search(&4);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), 2);
    }

    #[test]
    fn exercise_example() {
        let items = vec![1, 5, 8, 12, 13];
        let matches = vec![8, 1, 23, 1, 11];
        let result: Vec<i32> = matches
            .iter()
            .map(|i| match items.binary_search(i) {
                Ok(i) => i as i32,
                Err(_) => -1,
            })
            .collect();
        let expected = vec![2, 0, -1, 0, -1];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_case_2() {
        let items = vec![1, 2, 3, 4, 5];
        let matches = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = matches
            .iter()
            .map(|i| match items.binary_search(i) {
                Ok(i) => i as i32,
                Err(_) => -1,
            })
            .collect();
        let expected = vec![0, 1, 2, 3, 4];
        assert_eq!(expected, result);
    }
}
