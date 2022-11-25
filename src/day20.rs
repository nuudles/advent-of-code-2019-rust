use std::collections::{HashMap, HashSet};

use pathfinding::prelude::dijkstra;

use crate::point::Point;

#[derive(Debug)]
enum Node {
    Wall,
    Space,
    Tile(String),
}

pub fn part1(input: String) {
    let mut tiles = HashSet::<Point<i64>>::new();
    let mut map = HashMap::<Point<i64>, Node>::new();
    let (mut max_x, mut max_y) = (i64::MIN, i64::MIN);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point {
                x: x as i64,
                y: y as i64,
            };
            if c == '#' {
                map.insert(point, Node::Wall);
            } else if c == '.' {
                map.insert(point, Node::Space);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            } else if c.is_ascii_uppercase() {
                map.insert(point, Node::Tile(c.to_string()));
                tiles.insert(point);
            }
        }
    }

    let mut entrance: Point<i64> = Point { x: 0, y: 0 };
    let mut exit: Point<i64> = Point { x: 0, y: 0 };
    let mut portals = HashMap::<String, HashSet<Point<i64>>>::new();
    for point in tiles {
        if !matches!(map.get(&point), Some(Node::Tile(_))) {
            continue;
        }

        let neighbors = point.neighbors();
        if let Some(sibling) = neighbors
            .iter()
            .find(|n| matches!(map.get(*n), Some(Node::Tile(_))))
        {
            let name = match if point.x < sibling.x || point.y < sibling.y {
                (map.get(&point), map.get(sibling))
            } else {
                (map.get(sibling), map.get(&point))
            } {
                (Some(Node::Tile(a)), Some(Node::Tile(b))) => format!("{}{}", a, b),
                _ => String::new(),
            };
            let sibling_neighbors = sibling.neighbors();
            let position = neighbors
                .iter()
                .find(|n| matches!(map.get(*n), Some(Node::Space)))
                .unwrap_or_else(|| {
                    sibling_neighbors
                        .iter()
                        .find(|n| matches!(map.get(*n), Some(Node::Space)))
                        .unwrap_or(&Point { x: 0, y: 0 })
                });
            map.remove(&point);
            map.remove(sibling);
            if name == "AA" {
                entrance = *position;
            } else if name == "ZZ" {
                exit = *position;
            } else {
                map.insert(*position, Node::Tile(name.clone()));
                portals.entry(name).or_default().insert(*position);
            }
        }
    }
    let path = dijkstra(
        &entrance,
        |p| {
            let mut successors = vec![];
            for neighbor in p.neighbors() {
                match map.get(&neighbor) {
                    Some(Node::Space) => successors.push((neighbor, 1)),
                    Some(Node::Tile(name)) => {
                        if let Some(other_end) = portals
                            .get(name)
                            .and_then(|s| s.iter().find(|e| e != &&neighbor))
                        {
                            successors.push((*other_end, 2));
                        }
                    }
                    _ => (),
                }
            }
            successors
        },
        |p| p == &exit,
    )
    .expect("Could not find path");
    println!("Part 1: {}", path.1);

    let path_2 = dijkstra(
        &(entrance, 0),
        |(p, d)| {
            let mut successors = vec![];
            for neighbor in p.neighbors() {
                match map.get(&neighbor) {
                    Some(Node::Space) => {
                        if neighbor == entrance || neighbor == exit {
                            if d == &0 {
                                successors.push(((neighbor, *d), 1));
                            }
                        } else {
                            successors.push(((neighbor, *d), 1));
                        }
                    }
                    Some(Node::Tile(name)) => {
                        if let Some(other_end) = portals
                            .get(name)
                            .and_then(|s| s.iter().find(|e| e != &&neighbor))
                        {
                            if neighbor.x == 2
                                || neighbor.y == 2
                                || neighbor.x == max_x
                                || neighbor.y == max_y
                            {
                                // Outer
                                if d != &0 {
                                    successors.push(((*other_end, d - 1), 2));
                                }
                            } else {
                                // Inner
                                successors.push(((*other_end, d + 1), 2));
                            }
                        }
                    }
                    _ => (),
                }
            }
            successors
        },
        |(p, d)| p == &exit && d == &0,
    )
    .expect("Could not find path");
    println!("Part 2: {}", path_2.1);
}
