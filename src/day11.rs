use std::{
    collections::{HashMap, HashSet},
    sync::mpsc::channel,
    thread,
};

use crate::{intcode::Intcode, point::Point};

pub fn part1(input: String) {
    let intcode = Intcode::new(&input);
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    let input_handle = thread::spawn(move || {
        let mut white = HashSet::<Point<i64>>::new();
        let directions = [
            Point { x: 0i64, y: -1i64 },
            Point { x: 1i64, y: 0 },
            Point { x: 0i64, y: 1i64 },
            Point { x: -1i64, y: 0i64 },
        ];
        let mut direction = 0usize;
        let mut position = Point { x: 0i64, y: 0i64 };
        let mut painted = HashSet::new();

        _ = input_sender.send(0);
        while let Ok(color) = output_receiver.recv() {
            if let Ok(should_turn_right) = output_receiver.recv() {
                painted.insert(position);
                if color == 1 {
                    white.insert(position);
                } else {
                    white.remove(&position);
                }
                if should_turn_right == 1 {
                    direction = (direction + 1) % 4;
                } else {
                    direction = if direction > 0 { direction - 1 } else { 3 };
                }
                position = position + directions[direction];
            } else {
                break;
            }

            if white.contains(&position) {
                _ = input_sender.send(1);
            } else {
                _ = input_sender.send(0);
            }
        }
        println!("{}", painted.len());
    });
    handle.join().expect("Run failed");
    input_handle.join().expect("Input failed");
}

pub fn part2(input: String) {
    let intcode = Intcode::new(&input);
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    let input_handle = thread::spawn(move || {
        let mut white = HashSet::<Point<i64>>::new();
        let directions = [
            Point { x: 0i64, y: -1i64 },
            Point { x: 1i64, y: 0 },
            Point { x: 0i64, y: 1i64 },
            Point { x: -1i64, y: 0i64 },
        ];
        let mut direction = 0usize;
        let mut position = Point { x: 0i64, y: 0i64 };

        _ = input_sender.send(1);
        while let Ok(color) = output_receiver.recv() {
            if let Ok(should_turn_right) = output_receiver.recv() {
                if color == 1 {
                    white.insert(position);
                } else {
                    white.remove(&position);
                }
                if should_turn_right == 1 {
                    direction = (direction + 1) % 4;
                } else {
                    direction = if direction > 0 { direction - 1 } else { 3 };
                }
                position = position + directions[direction];
            } else {
                break;
            }

            if white.contains(&position) {
                _ = input_sender.send(1);
            } else {
                _ = input_sender.send(0);
            }
        }

        let (mut min_x, mut max_x, mut min_y, mut max_y) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
        for point in &white {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if white.contains(&Point { x, y }) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    });
    handle.join().expect("Run failed");
    input_handle.join().expect("Input failed");
}
