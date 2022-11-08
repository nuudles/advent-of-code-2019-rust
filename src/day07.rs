use std::{collections::HashMap, sync::mpsc::channel, thread};

use itertools::Itertools;

use crate::intcode::Intcode;

pub fn part1(input: String) {
    let mut largest = i64::MIN;
    for permutation in (0..5).permutations(5) {
        let mut channels = (0..6).map(|_| channel::<i64>()).collect_vec();

        let handles = permutation
            .iter()
            .enumerate()
            .map(|(i, p)| {
                _ = channels[0].0.send(*p);
                if i == 0 {
                    _ = channels[0].0.send(0);
                }
                let sender = channels[1].0.clone();
                let receiver = channels.remove(0).1;

                let machine = Intcode::new(&input);
                machine.run_with_input(HashMap::new(), receiver, sender)
            })
            .collect_vec();

        for handle in handles {
            handle.join().expect("Run failed");
        }

        largest = largest.max(channels[0].1.recv().unwrap_or_default());
    }
    println!("{}", largest);
}

pub fn part2(input: String) {
    let mut largest = i64::MIN;
    for permutation in (5..10).permutations(5) {
        let (mut senders, mut receivers): (Vec<_>, Vec<_>) = (0..6).map(|_| channel()).unzip();

        let handles = permutation
            .iter()
            .enumerate()
            .map(|(i, p)| {
                _ = senders[i].send(*p);
                let sender = senders[i + 1].clone();
                let receiver = receivers.remove(0);

                let machine = Intcode::new(&input);
                machine.run_with_input(HashMap::new(), receiver, sender)
            })
            .collect_vec();

        _ = senders[0].send(0);

        let last_receiver = receivers.remove(0);
        let first_sender = senders[0].clone();

        let (tx, rx) = channel::<i64>();

        let handler = thread::spawn(move || {
            while let Ok(value) = last_receiver.recv() {
                _ = tx.send(value);
                _ = first_sender.send(value);
            }
        });

        for handle in handles {
            handle.join().expect("Run failed");
        }
        senders.clear();
        handler.join().expect("Last run failed");

        largest = largest.max(rx.iter().last().unwrap_or_default());
    }
    println!("{}", largest);
}
