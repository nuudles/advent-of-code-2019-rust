use std::cmp::Ordering;

use gcd::Gcd;
use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut positions = input
        .lines()
        .map(|l| {
            let mut nums = parse_nums::<i64>(l);
            (
                nums.next().unwrap_or_default(),
                nums.next().unwrap_or_default(),
                nums.next().unwrap_or_default(),
            )
        })
        .collect_vec();

    let original_x = positions.iter().map(|p| p.0).collect_vec();
    let original_y = positions.iter().map(|p| p.1).collect_vec();
    let original_z = positions.iter().map(|p| p.2).collect_vec();
    let mut periods: [usize; 3] = [0, 0, 0];

    let mut velocities = positions.iter().map(|_| (0i64, 0i64, 0i64)).collect_vec();
    let mut step = 0usize;
    loop {
        // Apply gravity
        for ((ai, (ax, ay, az)), (bi, (bx, by, bz))) in
            positions.iter().enumerate().tuple_combinations()
        {
            let (avx, avy, avz) = velocities[ai];
            let (bvx, bvy, bvz) = velocities[bi];
            velocities[ai] = (
                avx + match ax.cmp(bx) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                },
                avy + match ay.cmp(by) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                },
                avz + match az.cmp(bz) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                },
            );
            velocities[bi] = (
                bvx + match bx.cmp(ax) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                },
                bvy + match by.cmp(ay) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                },
                bvz + match bz.cmp(az) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                },
            );
        }

        // Apply velocities
        for i in 0..positions.len() {
            let (x, y, z) = positions[i];
            let (vx, vy, vz) = velocities[i];
            positions[i] = (x + vx, y + vy, z + vz);
        }
        step += 1;

        if periods[0] == 0
            && positions.iter().map(|p| p.0).collect_vec() == original_x
            && velocities.iter().map(|v| v.0).all(|vx| vx == 0)
        {
            periods[0] = step;
        }
        if periods[1] == 0
            && positions.iter().map(|p| p.1).collect_vec() == original_y
            && velocities.iter().map(|v| v.1).all(|vy| vy == 0)
        {
            periods[1] = step;
        }
        if periods[2] == 0
            && positions.iter().map(|p| p.2).collect_vec() == original_z
            && velocities.iter().map(|v| v.2).all(|vz| vz == 0)
        {
            periods[2] = step;
        }

        if periods.iter().all(|p| p > &0) {
            break;
        }

        if step == 1000 {
            print!("Part 1: ");
            positions
                .iter()
                .zip(&velocities)
                .map(|((x, y, z), (vx, vy, vz))| {
                    (x.abs() + y.abs() + z.abs()) * (vx.abs() + vy.abs() + vz.abs())
                })
                .sum::<i64>()
                .print();
        }
    }
    println!("Part 2: {}", lcd(periods[0], lcd(periods[1], periods[2])));
}

fn lcd(a: usize, b: usize) -> usize {
    a * b / a.gcd(b)
}

pub fn part2(input: String) {}
