use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::stdin;
use std::ops::Range;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Item {
    density: f64,
    value: f64,
    weight: f64,
}

#[derive(Debug)]
struct ItemCutError {}

impl Display for ItemCutError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "You are trying to cut a piece larger than the Item itself!"
        )
    }
}
impl Error for ItemCutError {
    fn description(&self) -> &str {
        "You are trying to cut a piece larger than the Item itself"
    }
}

impl Item {
    fn new(value: f64, weight: f64) -> Self {
        let density = value / weight;
        Item {
            density,
            value,
            weight,
        }
    }

    fn cut(&self, weight: f64) -> Result<(Self, Self), ItemCutError> {
        if self.weight >= weight {
            let cutted = Item::new(weight * self.density, weight);
            let r_weight = self.weight - weight;
            let r_value = r_weight * self.density;
            let rest = Item::new(r_value, r_weight);

            Ok((cutted, rest))
        } else {
            Err(ItemCutError {})
        }
    }
}

#[derive(PartialEq, Debug)]
enum KnapsackErrors {
    NotEnoughCapacity,
}

impl Display for KnapsackErrors {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for KnapsackErrors {
    fn description(&self) -> &str {
        match self {
            &KnapsackErrors::NotEnoughCapacity => "The item doesnt fit in the knapsack",
        }
    }
}

#[derive(Debug)]
struct Knapsack {
    capacity: f64,
    value: f64,
}

impl Knapsack {
    fn new(capacity: f64) -> Self {
        Knapsack {
            capacity,
            value: 0_f64,
        }
    }

    fn fill(&mut self, item: Item) -> Result<(), KnapsackErrors> {
        if item.weight <= self.capacity {
            self.capacity = self.capacity - item.weight;
            self.value = self.value + item.value;
            Ok(())
        } else {
            Err(KnapsackErrors::NotEnoughCapacity)
        }
    }
}

fn deserialize_item(buff: String) -> Item {
    let values: Vec<f64> = buff
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    Item::new(values[0], values[1])
}

fn density_sort_items(items: &mut Vec<Item>) {
    items.sort_by(|first, second| first.density.partial_cmp(&second.density).unwrap());
}

fn max_loot_impl(sack: &mut Knapsack, items: &mut Vec<Item>) -> f64 {
    if sack.capacity > 0_f64 {
        let item = items.pop();
        match item {
            Some(i) => {
                if i.weight > sack.capacity {
                    let (cutted, rest) = i.cut(sack.capacity).unwrap();
                    sack.fill(cutted).unwrap();
                    items.push(rest);
                    max_loot_impl(sack, items)
                } else {
                    sack.fill(i).unwrap();
                    max_loot_impl(sack, items)
                }
            }
            None => sack.value,
        }
    } else {
        sack.value
    }
}

fn max_loot(sack: &mut Knapsack, items: &mut Vec<Item>) -> f64 {
    density_sort_items(items);

    max_loot_impl(sack, items)
}

fn main() {
    let mut buff = String::new();
    stdin().read_line(&mut buff).unwrap();
    let input: Vec<f64> = buff
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    let capacity = input[1];

    let mut items: Vec<Item> = Range {
        start: 0,
        end: input[0] as u32,
    }
    .map(|_| {
        let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        deserialize_item(buff)
    })
    .collect();

    let mut sack = Knapsack::new(capacity);
    println!("{}", max_loot(&mut sack, &mut items));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
        let factor = 10.0f64.powi(decimal_places as i32);
        let a = (a * factor).trunc();
        let b = (b * factor).trunc();
        a == b
    }

    #[test]
    fn density() {
        let item = Item::new(120_f64, 30_f64);
        assert_eq!(4, item.density.floor() as i32);
        let item = Item::new(60_f64, 20_f64);
        assert_eq!(3, item.density.floor() as i32);
        let item = Item::new(100_f64, 50_f64);
        assert_eq!(2, item.density.floor() as i32);

        let item = Item::new(500_f64, 30_f64);
        assert!(approx_equal(item.density, 16.6666_f64, 4));
    }

    #[test]
    fn cut() {
        let item = Item::new(120_f64, 30_f64);
        let result = item.cut(10_f64);
        assert!(result.is_ok());
        assert_eq!(
            result.ok().unwrap(),
            (Item::new(40_f64, 10_f64), Item::new(80_f64, 20_f64))
        );
    }

    #[test]
    fn deserializer() {
        let expected = Item::new(1_f64, 2_f64);
        let input = String::from("1 2");
        let result = deserialize_item(input);

        assert_eq!(expected, result);

        let expected = Item::new(100_f64, 20000_f64);
        let input = String::from("100 20000");
        let result = deserialize_item(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn density_sort() {
        let expected = vec![
            Item::new(100_f64, 50_f64),
            Item::new(60_f64, 20_f64),
            Item::new(120_f64, 30_f64),
        ];
        let mut items = vec![
            Item::new(60_f64, 20_f64),
            Item::new(100_f64, 50_f64),
            Item::new(120_f64, 30_f64),
        ];
        density_sort_items(&mut items);
        assert_eq!(items, expected);
    }

    #[test]
    fn knapsack() {
        let mut sack = Knapsack::new(50_f64);
        assert_eq!(sack.capacity.floor() as i32, 50);
        assert_eq!(sack.value.floor() as i32, 0);

        let result = sack.fill(Item::new(120_f64, 30_f64));
        assert!(result.is_ok());
        assert_eq!(sack.capacity.floor() as i32, 20);
        assert_eq!(sack.value.floor() as i32, 120);

        let result = sack.fill(Item::new(100_f64, 20_f64));
        assert!(result.is_ok());
        assert_eq!(sack.capacity.floor() as i32, 0);
        assert_eq!(sack.value.floor() as i32, 220);

        let result = sack.fill(Item::new(1_f64, 1_f64));
        assert_eq!(sack.capacity.floor() as i32, 0);
        assert_eq!(sack.value.floor() as i32, 220);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), KnapsackErrors::NotEnoughCapacity);
    }

    #[test]
    fn loot() {
        let mut items = vec![
            Item::new(60_f64, 20_f64),
            Item::new(100_f64, 50_f64),
            Item::new(120_f64, 30_f64),
        ];
        let mut sack = Knapsack::new(50_f64);
        let expected = 180;
        let loot = max_loot(&mut sack, &mut items);

        assert_eq!(loot.floor() as i32, expected);

        let mut items = vec![Item::new(500_f64, 30_f64)];
        let mut sack = Knapsack::new(10_f64);
        let expected = 166.6666;
        let loot = max_loot(&mut sack, &mut items);
        assert!(approx_equal(expected, loot, 4));
    }
}
