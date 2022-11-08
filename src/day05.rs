use std::{collections::HashMap, sync::mpsc::channel};

use crate::intcode::Intcode;

pub fn part1(input: String) {
    let (sender, recv) = channel();
    let (output_sender, output_recv) = channel();
    let intcode = Intcode::new(&input);
    _ = sender.send(1);
    intcode
        .run_with_input(HashMap::new(), recv, output_sender)
        .join()
        .expect("Failed to run");

    println!("{}", output_recv.iter().last().unwrap_or_default());
}

pub fn part2(input: String) {
    let (sender, recv) = channel();
    let (output_sender, output_recv) = channel();
    let intcode = Intcode::new(&input);
    _ = sender.send(5);
    intcode
        .run_with_input(HashMap::new(), recv, output_sender)
        .join()
        .expect("Failed to run");

    println!("{}", output_recv.iter().last().unwrap_or_default());
}
