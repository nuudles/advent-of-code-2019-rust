use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    f64::consts::PI,
};

use itertools::Itertools;

use crate::point::Point;

pub fn part1(input: String) {
    let mut asteroids = HashSet::<Point<usize>>::new();
    let (mut max_x, mut max_y) = (usize::MIN, usize::MIN);
    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                asteroids.insert(Point { x, y });
                max_x = max_x.max(x);
                max_y = max_y.max(x);
            }
        });
    });
    let mut max = usize::MIN;
    let mut target = &Point {
        x: 0usize,
        y: 0usize,
    };
    let mut best = HashMap::<_, _>::new();
    for asteroid in &asteroids {
        let angles = asteroids.iter().filter(|a| a != &asteroid).fold(
            HashMap::<String, Vec<&Point<usize>>>::new(),
            |mut map, a| {
                let mut angle =
                    (a.y as f64 - asteroid.y as f64).atan2(a.x as f64 - asteroid.x as f64);
                if angle < 0f64 {
                    angle += 2f64 * PI;
                }
                let key = format!("{}", angle);
                map.entry(key).or_default().push(a);
                map
            },
        );
        if angles.len() > max {
            max = angles.len();
            target = asteroid;
            best = angles;
        }
    }

    println!("Part 1: {}", max);

    let angles = best
        .keys()
        .flat_map(|k| k.parse::<f64>().ok())
        .sorted_by(|a, b| {
            if a < b {
                Ordering::Less
            } else if a == b {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        })
        .collect_vec();

    best.values_mut().for_each(|v| {
        v.sort_by(|a, b| {
            if a.manhattan_distance(target) < b.manhattan_distance(target) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
    });

    let mut index = angles
        .iter()
        .enumerate()
        .find(|(_, a)| **a >= 3f64 * PI / 2f64)
        .map(|(i, _)| i)
        .unwrap_or_default();
    let mut count = 0;
    while count < 200 {
        let key = format!("{}", angles[index]);
        let entry = best.entry(key).or_default();
        if let Some(destroyed) = entry.pop() {
            count += 1;
            if count == 200 {
                println!("Part 2: {}", destroyed.x * 100 + destroyed.y);
            }
        }
        index = (index + 1) % angles.len();
    }
}
