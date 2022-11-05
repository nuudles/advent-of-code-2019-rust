use std::collections::HashMap;

use pathfinding::prelude::bfs;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let mut orbits = HashMap::<&str, &str>::new();
    for line in input.lines() {
        let mut split = line.split(')');
        let (left, right) = (
            split.next().unwrap_or_default(),
            split.next().unwrap_or_default(),
        );
        orbits.insert(right, left);
    }
    orbits
        .keys()
        .map(|k| calculate_count(k, &orbits))
        .sum::<usize>()
        .print();
}

fn calculate_count(from: &str, orbits: &HashMap<&str, &str>) -> usize {
    if from == "COM" {
        return 0;
    }
    1 + calculate_count(orbits.get(from).unwrap_or(&"COM"), orbits)
}

pub fn part2(input: String) {
    let mut orbits = HashMap::<&str, Vec<&str>>::new();
    for line in input.lines() {
        let mut split = line.split(')');
        let (left, right) = (
            split.next().unwrap_or_default(),
            split.next().unwrap_or_default(),
        );
        orbits.entry(left).or_insert(vec![]).push(right);
        orbits.entry(right).or_insert(vec![]).push(left);
    }
    let path = bfs(
        &"YOU",
        |&c| orbits.get(c).unwrap_or(&vec![]).clone(),
        |&c| c == "SAN",
    );
    println!("{}", path.unwrap_or_default().len() - 3);
}
