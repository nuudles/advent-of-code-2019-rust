use std::collections::HashMap;

use crate::{intcode::Intcode, selfprint::SelfPrint};

pub fn part1(input: String) {
    let memory = Intcode::new(input).run(HashMap::from([(1, 12), (2, 2)]));
    memory.get(&0).unwrap_or(&0).print();
}

pub fn part2(input: String) {
    let intcode = Intcode::new(input);

    let noun = 76i64; // Ran with increasing nouns until I got over the target
    let mut verb = 0i64;
    loop {
        let memory = intcode.run(HashMap::from([(1, noun), (2, verb)]));

        let output = memory.get(&0).unwrap_or(&0);
        if output == &19690720 {
            break;
        }
        verb += 1;
    }
    println!("{}", 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day02::Intcode;

    #[test]
    fn test_intcode_1() {
        let memory = Intcode::new("1,9,10,3,2,3,11,0,99,30,40,50".to_string()).run(HashMap::new());
        assert_eq!(memory.get(&0).unwrap_or(&0), &3500);
    }

    #[test]
    fn test_intcode_2() {
        let memory = Intcode::new("1,0,0,0,99".to_string()).run(HashMap::new());
        assert_eq!(memory.get(&0).unwrap_or(&0), &2);
    }

    #[test]
    fn test_intcode_3() {
        let memory = Intcode::new("2,3,0,3,99".to_string()).run(HashMap::new());
        assert_eq!(memory.get(&3).unwrap_or(&0), &6);
    }

    #[test]
    fn test_intcode_4() {
        let memory = Intcode::new("2,4,4,5,99,0".to_string()).run(HashMap::new());
        assert_eq!(memory.get(&5).unwrap_or(&0), &9801);
    }

    #[test]
    fn test_intcode_5() {
        let memory = Intcode::new("1,1,1,4,99,5,6,0,99".to_string()).run(HashMap::new());
        assert_eq!(memory.get(&0).unwrap_or(&0), &30);
    }
}
