use std::collections::{HashMap, HashSet};

use crate::{parse_nums::parse_nums, point::Point};

pub fn part1(input: String) {
    let mut set = HashSet::<Point<i64>>::new();
    set.insert(Point { x: 0, y: 0 });

    let mut closest = i64::MAX;
    for (i, line) in input.lines().enumerate() {
        let mut pos = Point { x: 0i64, y: 0i64 };

        for dir in line.split(',') {
            let amount: i64 = parse_nums(dir).next().unwrap_or_default();
            let (dx, dy) = match dir.chars().next().unwrap_or_default() {
                'R' => (1, 0),
                'L' => (-1, 0),
                'D' => (0, 1),
                'U' => (0, -1),
                _ => (0i64, 0i64),
            };
            for _ in 0..amount {
                pos.x += dx;
                pos.y += dy;
                if i == 0 {
                    set.insert(pos);
                } else if set.contains(&pos) && pos.x.abs() + pos.y.abs() < closest {
                    closest = pos.x.abs() + pos.y.abs();
                }
            }
        }
    }
    println!("{}", closest);
}

pub fn part2(input: String) {
    let mut set = HashMap::<Point<i64>, usize>::new();
    set.insert(Point { x: 0, y: 0 }, 0);

    let mut closest = usize::MAX;
    for (i, line) in input.lines().enumerate() {
        let mut pos = Point { x: 0i64, y: 0i64 };
        let mut size = 0usize;

        for dir in line.split(',') {
            let amount: i64 = parse_nums(dir).next().unwrap_or_default();
            let (dx, dy) = match dir.chars().next().unwrap_or_default() {
                'R' => (1, 0),
                'L' => (-1, 0),
                'D' => (0, 1),
                'U' => (0, -1),
                _ => (0i64, 0i64),
            };
            for _ in 0..amount {
                size += 1;

                pos.x += dx;
                pos.y += dy;
                if i == 0 {
                    set.insert(pos, size);
                } else if let Some(s) = set.get(&pos) {
                    if s + size < closest {
                        closest = s + size;
                    }
                }
            }
        }
    }
    println!("{}", closest);
}
