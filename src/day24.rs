use std::collections::BTreeSet;

use itertools::iproduct;

use crate::point::Point;

pub fn part1(input: String) {
    let mut bugs = BTreeSet::<Point<i64>>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                bugs.insert(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    let mut seen = BTreeSet::<BTreeSet<Point<i64>>>::new();
    while !seen.contains(&bugs) {
        seen.insert(bugs.clone());
        let mut bugs_next = BTreeSet::<Point<i64>>::new();
        for (y, x) in iproduct!(0..5, 0..5) {
            let point = Point { x, y };
            let bug_neighbors = point
                .neighbors()
                .iter()
                .filter(|n| bugs.contains(*n))
                .count();
            if bugs.contains(&point) {
                if bug_neighbors == 1 {
                    bugs_next.insert(point);
                }
            } else if bug_neighbors == 1 || bug_neighbors == 2 {
                bugs_next.insert(point);
            }
        }
        bugs = bugs_next;
    }

    let mut total = 0i64;
    let mut points = 1;
    for (y, x) in iproduct!(0..5, 0..5) {
        if bugs.contains(&Point { x, y }) {
            total += points;
        }
        points *= 2;
    }
    println!("{}", total);
}

fn recursive_neighbors(bug: (i64, Point<i64>)) -> BTreeSet<(i64, Point<i64>)> {
    let (depth, position) = bug;
    let mut neighbors = BTreeSet::new();
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let n = Point {
            x: position.x + dx,
            y: position.y + dy,
        };
        match (n.x, n.y) {
            (-1, _) => _ = neighbors.insert((depth - 1, Point { x: 1, y: 2 })),
            (5, _) => _ = neighbors.insert((depth - 1, Point { x: 3, y: 2 })),
            (_, -1) => _ = neighbors.insert((depth - 1, Point { x: 2, y: 1 })),
            (_, 5) => _ = neighbors.insert((depth - 1, Point { x: 2, y: 3 })),
            (2, 2) => match (dx, dy) {
                (-1, 0) => {
                    for y in 0..5 {
                        neighbors.insert((depth + 1, Point { x: 4, y }));
                    }
                }
                (1, 0) => {
                    for y in 0..5 {
                        neighbors.insert((depth + 1, Point { x: 0, y }));
                    }
                }
                (0, -1) => {
                    for x in 0..5 {
                        neighbors.insert((depth + 1, Point { x, y: 4 }));
                    }
                }
                (0, 1) => {
                    for x in 0..5 {
                        neighbors.insert((depth + 1, Point { x, y: 0 }));
                    }
                }
                _ => (),
            },
            _ => _ = neighbors.insert((depth, n)),
        };
    }
    neighbors
}

pub fn part2(input: String) {
    let mut bugs = BTreeSet::<(i64, Point<i64>)>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                bugs.insert((
                    0,
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                ));
            }
        }
    }

    for _ in 0..200 {
        let mut bugs_next = BTreeSet::<(i64, Point<i64>)>::new();
        for bug in &bugs {
            for neighbor in recursive_neighbors(*bug) {
                let bug_neighbors = recursive_neighbors(neighbor)
                    .iter()
                    .filter(|n| bugs.contains(*n))
                    .count();
                if bugs.contains(&neighbor) {
                    if bug_neighbors == 1 {
                        bugs_next.insert(neighbor);
                    }
                } else if bug_neighbors == 1 || bug_neighbors == 2 {
                    bugs_next.insert(neighbor);
                }
            }
        }
        bugs = bugs_next;
    }
    println!("{}", bugs.len());
}
