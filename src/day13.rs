use std::{cmp::Ordering, collections::HashMap, sync::mpsc::channel, thread};

use itertools::Itertools;

use crate::{intcode::Intcode, selfprint::SelfPrint};

pub fn part1(input: String) {
    let intcode = Intcode::new(&input);
    let (_, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    handle.join().expect("Run failed");
    output_receiver
        .iter()
        .enumerate()
        .filter(|(i, o)| i % 3 == 2 && o == &2)
        .count()
        .print();
}

pub fn part2(input: String) {
    let intcode = Intcode::new(&input);
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let (score_sender, score_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::from([(0, 2)]), input_receiver, output_sender);
    let input_handle = thread::spawn(move || {
        let mut ball_x = -1;
        let mut paddle_x = -1;
        for (x, y, id) in output_receiver.iter().tuples() {
            if id == 4 {
                ball_x = x;
            } else if id == 3 {
                paddle_x = x;
            }
            if ball_x > -1 && paddle_x > -1 {
                _ = input_sender.send(match ball_x.cmp(&paddle_x) {
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                });
                ball_x = -1;
            }
            if x == -1 && y == 0 {
                _ = score_sender.send(id);
            }
        }
    });
    handle.join().expect("Run failed");
    input_handle.join().expect("Input run failed");
    score_receiver.iter().last().unwrap_or_default().print();
}
