use std::{
    collections::{HashMap, HashSet},
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use itertools::Itertools;
use pathfinding::prelude::bfs;

use crate::{intcode::Intcode, point::Point};

struct Robot {
    input_sender: Sender<i64>,
    output_receiver: Receiver<i64>,
    map: HashMap<Point<i64>, i64>,
    oxygen_pos: Point<i64>,
}

impl Robot {
    fn new(input_sender: Sender<i64>, output_receiver: Receiver<i64>) -> Robot {
        Robot {
            input_sender,
            output_receiver,
            map: HashMap::new(),
            oxygen_pos: Point { x: 0, y: 0 },
        }
    }

    fn explore(&mut self, position: Point<i64>) {
        if self.map.contains_key(&position) {
            return;
        }

        let reading = self.output_receiver.recv().unwrap_or(1);
        self.map.insert(position, reading);

        if reading == 0 {
            return;
        } else if reading == 2 {
            self.oxygen_pos = position;
        }

        for (next_pos, input, rev_input) in [
            (position.up(), 1, 2),
            (position.right(), 4, 3),
            (position.down(), 2, 1),
            (position.left(), 3, 4),
        ] {
            if self.map.contains_key(&next_pos) {
                continue;
            }

            _ = self.input_sender.send(input);
            self.explore(next_pos);
            if self.map.get(&next_pos).unwrap_or(&0) != &0 {
                _ = self.input_sender.send(rev_input);
                _ = self.output_receiver.recv(); // Backtracking so we already know the result here
            }
        }
    }

    fn fill(
        &self,
        position: Point<i64>,
        depth: usize,
        filled: &mut HashSet<Point<i64>>,
        max_depth: &mut usize,
    ) {
        if depth > *max_depth {
            *max_depth = depth;
        }
        for neighbor in position
            .neighbors()
            .iter()
            .filter(|n| self.map.get(*n).unwrap_or(&0) != &0)
        {
            if filled.contains(neighbor) {
                continue;
            }
            filled.insert(*neighbor);
            self.fill(*neighbor, depth + 1, filled, max_depth);
        }
    }
}

pub fn part1(input: String) {
    let intcode = Intcode::new(&input);
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    let input_handle = thread::spawn(move || {
        // Prime the receiver by going east, then west
        _ = input_sender.send(4);
        _ = output_receiver.recv();
        _ = input_sender.send(3);

        let mut robot = Robot::new(input_sender, output_receiver);
        robot.explore(Point { x: 0, y: 0 });

        let path = bfs(
            &Point { x: 0i64, y: 0i64 },
            |p| {
                p.neighbors()
                    .iter()
                    .filter(|n| robot.map.get(*n).unwrap_or(&1) != &0)
                    .copied()
                    .collect_vec()
            },
            |p| robot.map.get(p).unwrap_or(&1) == &2,
        )
        .expect("No path was found");
        println!("Part 1: {}", path.len());

        let mut filled = HashSet::<Point<i64>>::new();
        let mut max_depth = usize::MIN;
        robot.fill(robot.oxygen_pos, 0, &mut filled, &mut max_depth);
        println!("Part 2: {}", max_depth);
    });
    handle.join().expect("Run failed");
    input_handle.join().expect("Input run failed");
}
