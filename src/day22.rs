use itertools::Itertools;

use crate::parse_nums::parse_nums;
use mod_exp::mod_exp;

#[derive(Debug)]
enum Instruction {
    DealWithNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        if line.starts_with("cut") {
            let mut numbers = parse_nums::<i128>(line);
            Instruction::Cut(numbers.next().unwrap_or_default())
        } else if line.starts_with("deal with increment") {
            let mut numbers = parse_nums::<i128>(line);
            Instruction::DealWithIncrement(numbers.next().unwrap_or_default())
        } else {
            Instruction::DealWithNewStack
        }
    }
}

pub fn part1(input: String) {
    let instructions = input.lines().map(Instruction::from).collect_vec();

    let mut deck = (0..10007).collect_vec();
    for instruction in instructions {
        match instruction {
            Instruction::DealWithNewStack => deck = deck.iter().rev().copied().collect(),
            Instruction::Cut(amount) => {
                if amount > 0 {
                    deck.rotate_left(amount as usize);
                } else {
                    deck.rotate_right(amount.unsigned_abs() as usize);
                }
            }
            Instruction::DealWithIncrement(amount) => {
                let mut new_deck = deck.clone();
                let mut index = 0;

                for card in &deck {
                    new_deck[index] = *card;
                    index = (index + amount as usize) % deck.len();
                }
                deck = new_deck;
            }
        }
    }
    for (i, card) in deck.iter().enumerate() {
        if card == &2019 {
            println!("{}", i);
            break;
        }
    }
}

/*
fn mod_inv(a: i128, module: i128) -> i128 {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}
 */

pub fn part2(input: String) {
    let instructions = input.lines().map(Instruction::from).collect_vec();

    /*
    // First attempt that took way too long
    let mut index = 2020;
    let length = 119315717514047;

    for i in 0..101741582076661i128 {
        if i % 100000 == 0 {
            println!("{}", 101741582076661i128 - i);
        }
        for instruction in instructions.iter().rev() {
            match instruction {
                Instruction::DealWithNewStack => index = length - 1 - index,
                Instruction::Cut(amount) => index = (index + amount) % length,
                Instruction::DealWithIncrement(amount) => {
                    index = mod_inv(*amount, length) * index % length
                }
            }
        }
    }
    println!("{}", index);
     */

    let length = 119315717514047i128;

    // I didn't have the math background, so I looked up the solution thread and
    // read up on it. Don't think I would've been able to get this on my own, although
    // I started down the path of squishing the recipe into as few instructions as possible.

    // Convert the whole process to a linear equation: ax + b
    let (a, b) = instructions
        .iter()
        .rev()
        .fold((1, 0), |(a, b), instruction| {
            let (a_next, b_next) = match instruction {
                Instruction::DealWithNewStack => (-a, length - (1 + b)),
                Instruction::Cut(amount) => (a, b + amount),
                Instruction::DealWithIncrement(amount) => {
                    let n = mod_exp(*amount, length - 2, length);
                    (a * n, b * n)
                }
            };
            (a_next % length, b_next % length)
        });

    let count = 101741582076661i128;

    // Applying the function count times simplifies to:
    // x * a^n + b * (a^n - 1) / (a-1)
    let term1 = 2020 * mod_exp(a, count, length) % length;
    let tmp = (mod_exp(a, count, length) - 1) * mod_exp(a - 1, length - 2, length) % length;
    let term2 = b * tmp % length;
    println!("{}", (term1 + term2) % length);
}
