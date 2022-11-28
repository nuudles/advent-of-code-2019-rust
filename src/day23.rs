use std::{collections::HashMap, sync::mpsc::channel, thread};

use itertools::Itertools;

use crate::intcode::Intcode;

pub fn part1(input: String) {
    let intcode = Intcode::new(&input);

    let (packet_senders, mut packet_receivers): (Vec<_>, Vec<_>) =
        (0..50).map(|_| channel()).unzip();

    let (nat_sender, nat_receiver) = channel();

    let handles = (0..50)
        .map(|address| {
            let (input_sender, input_receiver) = channel();
            let (output_sender, output_receiver) = channel();

            let packet_receiver = packet_receivers.remove(0);

            let cloned_senders = packet_senders.iter().cloned().collect_vec();
            let nat_sender = nat_sender.clone();

            let input_handle = thread::spawn(move || {
                _ = input_sender.send(address as i64);
                loop {
                    match packet_receiver.try_recv() {
                        Ok(packet) => {
                            _ = input_sender.send(packet);
                            _ = input_sender.send(packet_receiver.recv().unwrap_or_default());
                        }
                        _ => _ = input_sender.send(-1),
                    }

                    if let Ok(recipient) = output_receiver.try_recv() {
                        if recipient == 255 {
                            let (x, y) = (
                                output_receiver.recv().unwrap_or_default(),
                                output_receiver.recv().unwrap_or_default(),
                            );
                            println!("Part 1: {}", y);
                            _ = nat_sender.send(x);
                            _ = nat_sender.send(y);
                        } else {
                            _ = cloned_senders[recipient as usize]
                                .send(output_receiver.recv().unwrap_or_default());
                            _ = cloned_senders[recipient as usize]
                                .send(output_receiver.recv().unwrap_or_default());
                        }
                    }
                    thread::sleep(std::time::Duration::from_millis(10));
                }
            });
            let run_handle = intcode.run_with_input(HashMap::new(), input_receiver, output_sender);
            (run_handle, input_handle)
        })
        .collect_vec();

    let zero_sender = packet_senders[0].clone();
    let nat_handle = thread::spawn(move || {
        let mut last_y = 0;
        loop {
            thread::sleep(std::time::Duration::from_millis(500)); // Just sleep for 500ms. Should be idle by then.
            let (mut x, mut y) = (0, 0);
            for (i, p) in nat_receiver.try_iter().enumerate() {
                if i % 2 == 0 {
                    x = p;
                } else {
                    y = p;
                }
            }
            _ = zero_sender.send(x);
            _ = zero_sender.send(y);
            if last_y == y {
                println!("Part 2: {}", y);
                break;
            }
            last_y = y;
        }
    });

    for (run_handle, input_handle) in handles {
        run_handle.join().expect("Run failed");
        input_handle.join().expect("Input failed");
    }
    nat_handle.join().expect("Nat failed");
}
