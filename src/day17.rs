use std::{
    collections::{HashMap, HashSet},
    sync::mpsc::channel,
};

use crate::{intcode::Intcode, point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let intcode = Intcode::new(&input);
    let (_, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    handle.join().expect("Run failed");
    let mut pos = Point { x: 0i64, y: 0i64 };
    let mut scaffolds = HashSet::<Point<i64>>::new();
    for c in output_receiver.iter() {
        match c {
            35 => _ = scaffolds.insert(pos),
            10 => {
                pos.y += 1;
                pos.x = -1;
            }
            _ => (),
        }
        pos.x += 1;
    }
    scaffolds
        .iter()
        .filter(|s| s.neighbors().iter().all(|n| scaffolds.contains(n)))
        .map(|s| s.x * s.y)
        .sum::<i64>()
        .print();
}

pub fn part2(input: String) {
    let intcode = Intcode::new(&input);
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::from([(0, 2)]), input_receiver, output_sender);

    // Printed it out and manually solved the problem
    let program = "A,B,B,A,B,C,A,C,B,C";
    let a = "L,4,L,6,L,8,L,12";
    let b = "L,8,R,12,L,12";
    let c = "R,12,L,6,L,6,L,8";
    for c in program.chars() {
        _ = input_sender.send(c as i64);
    }
    _ = input_sender.send('\n' as i64);
    for c in a.chars() {
        _ = input_sender.send(c as i64);
    }
    _ = input_sender.send('\n' as i64);
    for c in b.chars() {
        _ = input_sender.send(c as i64);
    }
    _ = input_sender.send('\n' as i64);
    for c in c.chars() {
        _ = input_sender.send(c as i64);
    }
    _ = input_sender.send('\n' as i64);
    _ = input_sender.send('n' as i64);
    _ = input_sender.send('\n' as i64);
    handle.join().expect("Run failed");
    output_receiver.iter().last().unwrap_or_default().print();
}
