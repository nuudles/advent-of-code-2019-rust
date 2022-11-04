use std::collections::HashMap;

use crate::intcode::Intcode;

pub fn part1(input: String) {
    Intcode::new(input).run_with_input(HashMap::new(), 1);
}

pub fn part2(input: String) {
    Intcode::new(input).run_with_input(HashMap::new(), 5);
}
