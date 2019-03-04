use std::fmt::Debug;
use std::io;
use std::io::prelude::*;
use std::io::StdinLock;
use std::iter::Iterator;
use std::time::{SystemTime, UNIX_EPOCH};

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

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rand {
    pub fn new(seed: Option<u32>) -> Rand {
        let seed = seed.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        });
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }

    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (<u32>::max_value() as f64)
    }
}

fn partition3<T>(input: &mut Vec<T>, pivot: usize, left: usize, right: usize) -> (usize, usize)
where
    T: Ord + Copy + Debug,
{
    // println!("{:?} - pi:{} l:{} r:{} (Init)", input, pivot, left, right);

    let p_e = input[pivot];
    input[pivot] = input[left];
    input[left] = p_e;
    let mut x = left;
    let mut y = left;
    //  println!("{:?} -- {} -> {} (Pivot)", input, pivot, left);
    for i in left..right {
        let i = i + 1;
        if input[i] == p_e {
            x = x + 1;
            y = y + 1;
            let h = input[i];
            input[i] = input[y];
            input[y] = input[x];
            input[x] = h;
        //    println!("{:?} -- {} -> {} -> {} -> {} (Eq)", input, x, y, i, x);
        } else if input[i] < p_e {
            y = y + 1;
            let h = input[i];
            input[i] = input[y];
            input[y] = h;

            //   println!("{:?} -- {} -> {} (Less)", input, i, y);
        }
    }

    for i in left..(x + 1) {
        let n = (y - i) + left;
        let h = input[i];
        input[i] = input[n];
        input[n] = h;
        //  println!("{:?} -- {} -> {} (Positioning)", input, i, n);
    }

    let from = (y - x) + left;
    let to = y;
    // println!("{:?} x:{} y:{} (Result)\n", input, from, to);
    (from, to)
}

fn partition2<T>(input: &mut Vec<T>, pivot: usize, left: usize, right: usize) -> usize
where
    T: Ord + Copy + Debug,
{
    let p_e = input[pivot];
    input[pivot] = input[left];
    input[left] = p_e;

    let mut j = left;
    for i in left..right {
        let i = i + 1;
        if input[i] <= p_e {
            j = j + 1;
            let h = input[i];
            input[i] = input[j];
            input[j] = h;
        }
    }

    let h = input[left];
    input[left] = input[j];
    input[j] = h;
    j
}

fn get_pivot(left: usize, right: usize) -> usize {
    let mut rng = Rand::new(None);
    left + rng.rand() as usize % (right - left + 1)
}

fn quick_sort<T>(input: &mut Vec<T>)
where
    T: Ord + Copy + Debug,
{
    fn helper<I>(input: &mut Vec<I>, left: usize, right: usize)
    where
        I: Ord + Copy + Debug,
    {
        if left >= right {
            return;
        }

        let pivot = get_pivot(left, right);

        // println!("pivot {}", pivot);
        let (x, y) = partition3(input, pivot, left, right);
        // println!("{:?} - x:{} y:{}", input, x, y);

        if x > 0 {
            helper(input, left, x - 1);
        }
        if y < right {
            helper(input, y + 1, right);
        }
    }

    let right = input.len() - 1;
    helper(input, 0, right);
    // println!("\n\n------\n\n")
}

fn main() {
    let mut _size: u32 = {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut iter = iterable_input(&mut stdin);
        SplitIterator::new(&mut iter)
            .map(|v| String::from_utf8(v).unwrap().parse().unwrap())
            .next()
            .unwrap()
    };

    let mut input: Vec<i32> = {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut iter = iterable_input(&mut stdin);
        let iter = SplitIterator::new(&mut iter);
        iter.map(|v| String::from_utf8(v).unwrap().parse().unwrap())
            .collect()
    };

    quick_sort(&mut input);
    input.iter().for_each(|v| {
        let bytes: Vec<u8> = format!("{} ", v).bytes().collect();
        io::stdout().write(&bytes).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition2_two_ordered() {
        let mut provided = vec![1, 2];
        let result = partition2(&mut provided, 0, 0, 1);
        assert_eq!(result, 0);
        let expected = vec![1, 2];
        assert_eq!(provided, expected);

        let result = partition2(&mut provided, 1, 0, 1);
        assert_eq!(result, 1);
        let expected = vec![1, 2];
        assert_eq!(provided, expected);
    }

    #[test]
    fn partition3_non_repeated() {
        let mut provided = vec![1, 2, 3, 4, 5];
        let expected = vec![1, 2, 3, 4, 5];

        for i in 0..5 {
            let (x, y) = partition3(&mut provided, i, 0, 4);
            assert_eq!(x, y);
            assert_eq!(x, i);
            assert_eq!(provided, expected);
        }
    }

    #[test]
    fn partition3_repeated() {
        let provided = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];

        let mut next = provided.clone();
        let (x, y) = partition3(&mut next, 0, 0, 9);

        assert_eq!(next, vec![1, 1, 3, 4, 5, 2, 2, 3, 4, 5]);
        assert_eq!(x, 0);
        assert_eq!(y, 1);

        let mut next = provided.clone();
        let (x, y) = partition3(&mut next, 1, 0, 9);

        assert_eq!(next, vec![1, 1, 2, 2, 5, 3, 4, 3, 4, 5]);
        assert_eq!(x, 2);
        assert_eq!(y, 3);

        let (x, y) = partition3(&mut provided.clone(), 2, 0, 9);

        assert_eq!(x, 4);
        assert_eq!(y, 5);

        let (x, y) = partition3(&mut provided.clone(), 3, 0, 9);

        assert_eq!(x, 6);
        assert_eq!(y, 7);

        let (x, y) = partition3(&mut provided.clone(), 4, 0, 9);

        assert_eq!(x, 8);
        assert_eq!(y, 9);

        let (x, y) = partition3(&mut provided.clone(), 5, 0, 9);

        assert_eq!(x, 0);
        assert_eq!(y, 1);

        let (x, y) = partition3(&mut provided.clone(), 9, 0, 9);

        assert_eq!(x, 8);
        assert_eq!(y, 9);

        let mut next = vec![3, 3, 3, 1];
        let (x, y) = partition3(&mut next, 0, 0, 3);
        assert_eq!(x, 1);
        assert_eq!(y, 3);
        assert_eq!(next, vec![1, 3, 3, 3]);
    }

    #[test]
    fn quicksort_t() {
        let mut provided = vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1, 5, 4, 3, 2, 1];
        let expected = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5];

        loop {
            quick_sort(&mut provided);
            assert_eq!(provided, expected);
        }
    }

    #[test]
    fn quicksort_first_repeated() {
        let mut provided = vec![6, 6, 6, 6, 6, 6, 1];
        let expected = vec![1, 6, 6, 6, 6, 6, 6];

        quick_sort(&mut provided);
        assert_eq!(provided, expected);
    }
}
