use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

#[derive(Debug)]
enum Parameter {
    Position(u64),
    Immediate(i64),
    Relative(i64),
}

impl Parameter {
    fn new(indicator: i64, value: &i64) -> Self {
        match indicator {
            0 => Self::Position(*value as u64),
            1 => Self::Immediate(*value),
            _ => Self::Relative(*value),
        }
    }

    fn value(&self, memory: &HashMap<u64, i64>, base: i64) -> i64 {
        match self {
            Self::Immediate(value) => *value,
            _ => *memory.get(&self.position(base)).unwrap_or(&0),
        }
    }

    fn position(&self, base: i64) -> u64 {
        match self {
            Self::Position(position) => *position,
            Self::Immediate(_) => 0,
            Self::Relative(offset) => (base + *offset) as u64,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    AdjustRelativeBase(Parameter),
    Terminate,
}

impl Opcode {
    fn new(memory: &HashMap<u64, i64>, position: u64) -> Option<Self> {
        let opcode = memory.get(&position)?;
        match opcode % 100 {
            1 => Some(Self::Add(
                Parameter::new(
                    (opcode / 100) % 10,
                    memory.get(&(position + 1)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 1000) % 10,
                    memory.get(&(position + 2)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 10000) % 10,
                    memory.get(&(position + 3)).unwrap_or(&0),
                ),
            )),
            2 => Some(Self::Multiply(
                Parameter::new(
                    (opcode / 100) % 10,
                    memory.get(&(position + 1)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 1000) % 10,
                    memory.get(&(position + 2)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 10000) % 10,
                    memory.get(&(position + 3)).unwrap_or(&0),
                ),
            )),
            3 => Some(Self::Input(Parameter::new(
                (opcode / 100) % 10,
                memory.get(&(position + 1)).unwrap_or(&0),
            ))),
            4 => Some(Self::Output(Parameter::new(
                (opcode / 100) % 10,
                memory.get(&(position + 1)).unwrap_or(&0),
            ))),
            5 => Some(Self::JumpIfTrue(
                Parameter::new(
                    (opcode / 100) % 10,
                    memory.get(&(position + 1)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 1000) % 10,
                    memory.get(&(position + 2)).unwrap_or(&0),
                ),
            )),
            6 => Some(Self::JumpIfFalse(
                Parameter::new(
                    (opcode / 100) % 10,
                    memory.get(&(position + 1)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 1000) % 10,
                    memory.get(&(position + 2)).unwrap_or(&0),
                ),
            )),
            7 => Some(Self::LessThan(
                Parameter::new(
                    (opcode / 100) % 10,
                    memory.get(&(position + 1)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 1000) % 10,
                    memory.get(&(position + 2)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 10000) % 10,
                    memory.get(&(position + 3)).unwrap_or(&0),
                ),
            )),
            8 => Some(Self::Equals(
                Parameter::new(
                    (opcode / 100) % 10,
                    memory.get(&(position + 1)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 1000) % 10,
                    memory.get(&(position + 2)).unwrap_or(&0),
                ),
                Parameter::new(
                    (opcode / 10000) % 10,
                    memory.get(&(position + 3)).unwrap_or(&0),
                ),
            )),
            9 => Some(Self::AdjustRelativeBase(Parameter::new(
                (opcode / 100) % 10,
                memory.get(&(position + 1)).unwrap_or(&0),
            ))),
            _ => Some(Self::Terminate),
        }
    }
}

#[derive(Debug)]
pub struct Intcode {
    memory: HashMap<u64, i64>,
}

impl Intcode {
    pub fn new(input: &str) -> Self {
        Self {
            memory: input
                .split(',')
                .enumerate()
                .map(|(i, v)| (i as u64, v.parse().unwrap_or(0i64)))
                .collect(),
        }
    }

    pub fn run(&self, initial_values: HashMap<u64, i64>) -> HashMap<u64, i64> {
        let (sender, recv) = channel();
        let handle = self.run_with_input(initial_values, recv, sender);
        handle.join().expect("Failed to run")
    }

    pub fn run_with_input(
        &self,
        initial_values: HashMap<u64, i64>,
        input_receiver: Receiver<i64>,
        output_sender: Sender<i64>,
    ) -> JoinHandle<HashMap<u64, i64>> {
        let mut memory = self.memory.clone();
        let mut position = 0u64;

        for (k, v) in initial_values {
            memory.insert(k, v);
        }

        thread::spawn(move || {
            let mut base = 0i64;

            while let Some(opcode) = Opcode::new(&memory, position) {
                match opcode {
                    Opcode::Add(a, b, o) => {
                        memory.insert(
                            o.position(base),
                            a.value(&memory, base) + b.value(&memory, base),
                        );
                        position += 4;
                    }
                    Opcode::Multiply(a, b, o) => {
                        memory.insert(
                            o.position(base),
                            a.value(&memory, base) * b.value(&memory, base),
                        );
                        position += 4;
                    }
                    Opcode::Input(o) => {
                        let value = input_receiver.recv();
                        memory.insert(o.position(base), value.unwrap_or_default());
                        position += 2;
                    }
                    Opcode::Output(a) => {
                        _ = output_sender.send(a.value(&memory, base));
                        position += 2;
                    }
                    Opcode::JumpIfTrue(a, p) => {
                        if a.value(&memory, base) != 0 {
                            position = p.value(&memory, base) as u64;
                        } else {
                            position += 3;
                        }
                    }
                    Opcode::JumpIfFalse(a, p) => {
                        if a.value(&memory, base) == 0 {
                            position = p.value(&memory, base) as u64;
                        } else {
                            position += 3;
                        }
                    }
                    Opcode::LessThan(a, b, o) => {
                        memory.insert(
                            o.position(base),
                            if a.value(&memory, base) < b.value(&memory, base) {
                                1
                            } else {
                                0
                            },
                        );
                        position += 4;
                    }
                    Opcode::Equals(a, b, o) => {
                        memory.insert(
                            o.position(base),
                            if a.value(&memory, base) == b.value(&memory, base) {
                                1
                            } else {
                                0
                            },
                        );
                        position += 4;
                    }
                    Opcode::AdjustRelativeBase(o) => {
                        base += o.value(&memory, base);
                        position += 2;
                    }
                    _ => {
                        break;
                    }
                }
            }

            memory
        })
    }
}
