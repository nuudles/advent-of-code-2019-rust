use std::{collections::HashMap, sync::mpsc::channel};

use itertools::iproduct;

use crate::intcode::Intcode;

fn is_pulled(x: i64, y: i64, intcode: &Intcode) -> bool {
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
    _ = input_sender.send(x);
    _ = input_sender.send(y);
    handle.join().expect("Input failed");
    output_receiver.recv().unwrap_or_default() == 1
}

pub fn part1(input: String) {
    let intcode = Intcode::new(&input);

    let mut count = 0usize;
    for (y, x) in iproduct!(0..50, 0..50) {
        if is_pulled(x, y, &intcode) {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn part2(input: String) {
    let intcode = Intcode::new(&input);

    let mut lines = HashMap::<i64, (i64, i64)>::new();
    let mut last_min_x = 0;
    for y in 500..10000 {
        let (mut min_x, mut max_x) = (i64::MAX, i64::MIN);
        let mut x = last_min_x;
        loop {
            if is_pulled(x, y, &intcode) {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
            } else if min_x < i64::MAX {
                break;
            }
            x += 1;
        }
        lines.insert(y, (min_x, max_x));
        last_min_x = min_x;

        if max_x - min_x >= 99 {
            if let Some(last) = lines.get(&(y - 99)) {
                if last.1 - min_x >= 99 {
                    println!("{}", min_x * 10000 + (y - 99));
                    break;
                }
            }
        }
    }
}
