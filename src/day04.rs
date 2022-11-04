use std::collections::HashMap;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let mut values = input.split('-').flat_map(|v| v.parse::<u64>());
    let min = values.next().unwrap_or_default();
    let max = values.next().unwrap_or_default();
    (min..max)
        .filter(|x| {
            let mut value = *x;
            let mut last_digit = value % 10;
            value /= 10;

            let mut duplicate_found = false;
            while value > 0 {
                let digit = value % 10;
                if !duplicate_found && digit == last_digit {
                    duplicate_found = true;
                }
                if digit > last_digit {
                    return false;
                }
                last_digit = digit;
                value /= 10;
            }
            duplicate_found
        })
        .count()
        .print();
}

pub fn part2(input: String) {
    let mut values = input.split('-').flat_map(|v| v.parse::<u64>());
    let min = values.next().unwrap_or_default();
    let max = values.next().unwrap_or_default();
    (min..max)
        .filter(|x| {
            let mut value = *x;
            let mut last_digit = value % 10;
            value /= 10;

            let mut counts = HashMap::<u64, usize>::new();
            counts.insert(last_digit, 1);
            while value > 0 {
                let digit = value % 10;
                counts.insert(digit, counts.get(&digit).unwrap_or(&0) + 1);
                if digit > last_digit {
                    return false;
                }
                last_digit = digit;
                value /= 10;
            }
            counts.values().contains(&2)
        })
        .count()
        .print();
}
