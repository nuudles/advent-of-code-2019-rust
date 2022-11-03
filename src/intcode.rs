use std::collections::HashMap;

#[derive(Debug)]
pub struct Intcode {
    memory: HashMap<u64, i64>,
}

impl Intcode {
    pub fn new(input: String) -> Self {
        Self {
            memory: input
                .split(',')
                .enumerate()
                .map(|(i, v)| (i as u64, v.parse().unwrap_or(0i64)))
                .collect(),
        }
    }

    pub fn run(&self, inputs: HashMap<u64, i64>) -> HashMap<u64, i64> {
        let mut memory = self.memory.clone();
        let mut position = 0u64;

        for (k, v) in inputs {
            memory.insert(k, v);
        }

        while let Some(opcode) = memory.get(&position) {
            match opcode {
                1 => {
                    let (a, b, o) = (
                        *memory.get(&(position + 1)).unwrap_or(&0) as u64,
                        *memory.get(&(position + 2)).unwrap_or(&0) as u64,
                        *memory.get(&(position + 3)).unwrap_or(&0) as u64,
                    );
                    memory.insert(
                        o,
                        *memory.get(&a).unwrap_or(&0) + *memory.get(&b).unwrap_or(&0),
                    );
                    position += 4;
                }
                2 => {
                    let (a, b, o) = (
                        *memory.get(&(position + 1)).unwrap_or(&0) as u64,
                        *memory.get(&(position + 2)).unwrap_or(&0) as u64,
                        *memory.get(&(position + 3)).unwrap_or(&0) as u64,
                    );
                    memory.insert(
                        o,
                        *memory.get(&a).unwrap_or(&0) * *memory.get(&b).unwrap_or(&0),
                    );
                    position += 4;
                }
                99 => {
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        memory
    }
}
