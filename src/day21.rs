use std::{collections::HashMap, sync::mpsc::channel};

use crate::{intcode::Intcode, selfprint::SelfPrint};

pub fn part1(input: String) {
    let intcode = Intcode::new(&input);
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    for c in "NOT A J\nNOT C T\nOR T J\nAND D J\nWALK\n".chars() {
        _ = input_sender.send(c as i64);
    }
    handle.join().expect("Run failed");
    output_receiver.iter().last().unwrap_or_default().print();
}

pub fn part2(input: String) {
    let intcode = Intcode::new(&input);
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    for command in [
        "NOT J J", "AND B J", "AND C J", "NOT J J", "NOT T T", "AND E T", "AND F T", "AND G T",
        "NOT T T", "AND T J", "AND D J", "AND H J", "NOT A T", "OR T J", "RUN",
    ] {
        for c in command.chars() {
            _ = input_sender.send(c as i64);
        }
        _ = input_sender.send('\n' as i64);
    }
    handle.join().expect("Run failed");
    output_receiver.iter().last().unwrap_or_default().print();
}
