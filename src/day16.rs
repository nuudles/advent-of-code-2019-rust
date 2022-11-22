use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn step(signal: Vec<i64>) -> Vec<i64> {
    signal
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut pattern: Vec<i64> = vec![];
            let base_pattern = [0, 1, 0, -1];
            let mut pattern_index = 0;
            while pattern.len() < signal.len() + 1 {
                for _ in 0..=i {
                    pattern.push(base_pattern[pattern_index]);
                }
                pattern_index = (pattern_index + 1) % 4;
            }
            _ = pattern.remove(0);
            signal
                .iter()
                .zip(pattern)
                .map(|(a, b)| a * b)
                .sum::<i64>()
                .abs()
                % 10
        })
        .collect_vec()
}

pub fn part1(input: String) {
    let mut signal = input
        .as_bytes()
        .iter()
        .map(|b| (*b - b'0') as i64)
        .collect_vec();
    for _ in 0..100 {
        signal = step(signal);
    }
    signal.iter().take(8).join("").print();
}

pub fn part2(input: String) {
    let offset = input
        .bytes()
        .take(7)
        .fold(0usize, |total, b| total * 10 + (b - b'0') as usize);
    let mut signal = input
        .as_bytes()
        .iter()
        .cycle()
        .take(input.len() * 10000)
        .skip(offset)
        .map(|b| (*b - b'0') as i64)
        .collect_vec();

    for _ in 0..100 {
        let mut sum = signal.iter().sum::<i64>();
        let mut last = 0i64;
        signal = signal
            .iter()
            .map(|n| {
                sum -= last;
                let digit = sum % 10;
                last = *n;
                digit
            })
            .collect();
    }

    signal.iter().take(8).join("").print();
}
