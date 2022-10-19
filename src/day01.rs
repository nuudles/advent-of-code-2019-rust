use pathfinding::num_traits::ToPrimitive;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .flat_map(|l| l.parse::<f64>().ok())
        .map(|m| (m / 3.0f64).floor().to_u64().unwrap_or_default() - 2)
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .flat_map(|l| l.parse::<f64>().ok())
        .map(fuel)
        .sum::<u64>()
        .print();
}

fn fuel(mass: f64) -> u64 {
    if mass == 0.0f64 {
        return 0;
    }
    let f = (mass / 3.0f64)
        .floor()
        .to_u64()
        .unwrap_or_default()
        .saturating_sub(2);
    f + fuel(f.to_f64().unwrap_or_default())
}
