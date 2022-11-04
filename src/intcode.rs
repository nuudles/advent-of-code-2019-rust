use std::collections::HashMap;

#[derive(Debug)]
enum Parameter {
    Position(u64),
    Immediate(i64),
}

impl Parameter {
    fn new(indicator: i64, value: &i64) -> Self {
        match indicator {
            0 => Self::Position(*value as u64),
            _ => Self::Immediate(*value),
        }
    }

    fn value(&self, memory: &HashMap<u64, i64>) -> i64 {
        match self {
            Self::Position(position) => *memory.get(position).unwrap_or(&0),
            Self::Immediate(value) => *value,
        }
    }

    fn position(&self) -> u64 {
        match self {
            Self::Position(position) => *position,
            Self::Immediate(_) => 0,
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
            _ => Some(Self::Terminate),
        }
    }
}

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

    pub fn run(&self, initial_values: HashMap<u64, i64>) -> HashMap<u64, i64> {
        self.run_with_input(initial_values, 0)
    }

    pub fn run_with_input(
        &self,
        initial_values: HashMap<u64, i64>,
        input: i64,
    ) -> HashMap<u64, i64> {
        let mut memory = self.memory.clone();
        let mut position = 0u64;

        for (k, v) in initial_values {
            memory.insert(k, v);
        }

        while let Some(opcode) = Opcode::new(&memory, position) {
            match opcode {
                Opcode::Add(a, b, o) => {
                    memory.insert(o.position(), a.value(&memory) + b.value(&memory));
                    position += 4;
                }
                Opcode::Multiply(a, b, o) => {
                    memory.insert(o.position(), a.value(&memory) * b.value(&memory));
                    position += 4;
                }
                Opcode::Input(o) => {
                    memory.insert(o.position(), input);
                    position += 2;
                }
                Opcode::Output(a) => {
                    println!("{}", a.value(&memory));
                    position += 2;
                }
                Opcode::JumpIfTrue(a, p) => {
                    if a.value(&memory) != 0 {
                        position = p.value(&memory) as u64;
                    } else {
                        position += 3;
                    }
                }
                Opcode::JumpIfFalse(a, p) => {
                    if a.value(&memory) == 0 {
                        position = p.value(&memory) as u64;
                    } else {
                        position += 3;
                    }
                }
                Opcode::LessThan(a, b, o) => {
                    memory.insert(
                        o.position(),
                        if a.value(&memory) < b.value(&memory) {
                            1
                        } else {
                            0
                        },
                    );
                    position += 4;
                }
                Opcode::Equals(a, b, o) => {
                    memory.insert(
                        o.position(),
                        if a.value(&memory) == b.value(&memory) {
                            1
                        } else {
                            0
                        },
                    );
                    position += 4;
                }
                _ => {
                    break;
                }
            }
        }

        memory
    }
}
