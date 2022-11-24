use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::{build_path, dijkstra, dijkstra_all};

use crate::point::Point;

pub fn part1(input: String) {
    let mut keys = HashMap::<char, Point<usize>>::new();
    let mut map = HashMap::<Point<usize>, char>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point { x, y };
            if c.is_ascii_lowercase() || c == '@' {
                keys.insert(c, point);
            }
            map.insert(point, c);
        }
    }

    let mut paths = HashMap::<BTreeSet<char>, (usize, HashSet<char>)>::new();
    for (key, pos) in &keys {
        let reachables = dijkstra_all(pos, |p| {
            p.neighbors()
                .iter()
                .filter(|n| map.get(*n).unwrap_or(&'#') != &'#')
                .map(|n| (*n, 1usize))
                .collect_vec()
        });
        for (other_key, other_pos) in &keys {
            if key == other_key {
                continue;
            }
            if let Some(&(_, cost)) = reachables.get(other_pos) {
                let path = build_path(other_pos, &reachables);
                paths.insert(
                    BTreeSet::from([*key, *other_key]),
                    (
                        cost,
                        path.iter()
                            .flat_map(|p| {
                                let c = map.get(p).unwrap_or(&'#');
                                if c.is_ascii_uppercase() {
                                    Some(c.to_ascii_lowercase())
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    ),
                );
            }
        }
    }

    let all_keys: BTreeSet<char> = keys.keys().copied().collect();
    let path = dijkstra(
        &(BTreeSet::from(['@']), '@'),
        |(visited, last)| {
            all_keys
                .difference(visited)
                .flat_map(|k| {
                    if let Some(path) = paths.get(&BTreeSet::from([*last, *k])) {
                        if path.1.iter().all(|p| visited.contains(p)) {
                            let mut next = visited.clone();
                            next.insert(*k);
                            Some(((next, *k), path.0))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
        |(v, _)| v.len() == all_keys.len(),
    )
    .expect("No path found");
    println!("{}", path.1);
}

pub fn part2(input: String) {
    let mut keys = HashMap::<char, Point<usize>>::new();
    let mut map = HashMap::<Point<usize>, char>::new();
    let mut entrance = Point {
        x: 0usize,
        y: 0usize,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point { x, y };
            map.insert(point, c);
            if c == '@' {
                entrance = point;
            } else if c.is_ascii_lowercase() {
                keys.insert(c, point);
            }
        }
    }
    map.insert(entrance, '#');
    map.insert(entrance.down(), '#');
    map.insert(entrance.up(), '#');
    map.insert(entrance.left(), '#');
    map.insert(entrance.right(), '#');

    let all_keys: BTreeSet<char> = keys.keys().copied().collect();

    keys.insert(
        '1',
        Point {
            x: entrance.x - 1,
            y: entrance.y - 1,
        },
    );
    keys.insert(
        '2',
        Point {
            x: entrance.x + 1,
            y: entrance.y - 1,
        },
    );
    keys.insert(
        '3',
        Point {
            x: entrance.x - 1,
            y: entrance.y + 1,
        },
    );
    keys.insert(
        '4',
        Point {
            x: entrance.x + 1,
            y: entrance.y + 1,
        },
    );

    let mut paths = HashMap::<BTreeSet<Point<usize>>, (usize, HashSet<char>)>::new();
    for (key, pos) in &keys {
        let reachables = dijkstra_all(pos, |p| {
            p.neighbors()
                .iter()
                .filter(|n| map.get(*n).unwrap_or(&'#') != &'#')
                .map(|n| (*n, 1usize))
                .collect_vec()
        });
        for (other_key, other_pos) in &keys {
            if key == other_key {
                continue;
            }
            if let Some(&(_, cost)) = reachables.get(other_pos) {
                let path = build_path(other_pos, &reachables);
                paths.insert(
                    BTreeSet::from([*pos, *other_pos]),
                    (
                        cost,
                        path.iter()
                            .flat_map(|p| {
                                let c = map.get(p).unwrap_or(&'#');
                                if c.is_ascii_uppercase() {
                                    Some(c.to_ascii_lowercase())
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    ),
                );
            }
        }
    }
    let path = dijkstra(
        &(
            BTreeSet::<char>::new(),
            [
                Point {
                    x: entrance.x - 1,
                    y: entrance.y - 1,
                },
                Point {
                    x: entrance.x + 1,
                    y: entrance.y - 1,
                },
                Point {
                    x: entrance.x - 1,
                    y: entrance.y + 1,
                },
                Point {
                    x: entrance.x + 1,
                    y: entrance.y + 1,
                },
            ],
        ),
        |(visited, robots)| {
            let mut successors = vec![];
            for (i, robot) in robots.iter().enumerate() {
                for key in all_keys.difference(visited) {
                    let pos = keys.get(key).unwrap_or(&Point { x: 0, y: 0 });
                    if let Some(path) = paths.get(&BTreeSet::from([*pos, *robot])) {
                        if path.1.iter().all(|p| visited.contains(p)) {
                            let mut next_visited = visited.clone();
                            next_visited.insert(*key);
                            let mut next_robots = *robots;
                            next_robots[i] = *pos;
                            successors.push(((next_visited, next_robots), path.0));
                        }
                    }
                }
            }
            successors
        },
        |(visited, _)| visited.len() == all_keys.len(),
    )
    .expect("No path found");
    println!("{}", path.1);
}
