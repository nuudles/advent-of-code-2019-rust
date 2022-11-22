use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Chemical<'a> {
    name: &'a str,
    quantity: i64,
}

impl Chemical<'_> {
    fn from(line: &str) -> Chemical {
        let mut split = line.split(' ');
        let (quantity, name) = (
            split.next().unwrap_or_default(),
            split.next().unwrap_or_default(),
        );
        Chemical {
            name,
            quantity: quantity.parse().unwrap_or_default(),
        }
    }
}

fn calculate_depth<'a>(
    chemical: &'a str,
    requirements: &'a HashMap<&'a str, (i64, Vec<Chemical>)>,
    depths: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(depth) = depths.get(chemical) {
        return *depth;
    }

    if let Some((_, chemicals)) = requirements.get(chemical) {
        let depth = if chemicals.iter().any(|c| c.name == "ORE") {
            0
        } else {
            chemicals
                .iter()
                .map(|c| 1 + calculate_depth(c.name, requirements, depths))
                .max()
                .unwrap_or_default()
        };
        depths.insert(chemical, depth);
        depth
    } else {
        usize::MAX
    }
}

fn ore_required(fuel_count: i64, requirements: &HashMap<&str, (i64, Vec<Chemical>)>) -> i64 {
    let mut need = HashMap::from([("FUEL", fuel_count)]);
    let mut have: HashMap<&str, i64> = HashMap::new();
    let mut ore = 0i64;

    let mut depths = HashMap::<&str, usize>::new();
    for (_, chemicals) in requirements.values() {
        for chemical in chemicals {
            if depths.contains_key(chemical.name) {
                continue;
            }
            calculate_depth(chemical.name, requirements, &mut depths);
        }
    }

    while let Some(&name) = &need
        .keys()
        .sorted_by(|a, b| {
            depths
                .get(**b)
                .unwrap_or(&0)
                .cmp(depths.get(**a).unwrap_or(&0))
        })
        .next()
    {
        let mut quantity = need.remove(name).unwrap_or_default();
        if let Some(count) = have.get(name) {
            if quantity < *count {
                *have.entry(name).or_default() -= quantity;
                continue;
            } else {
                quantity -= *count;
                have.remove(name);
                if quantity <= 0 {
                    continue;
                }
            }
        }
        if let Some((count, chemicals)) = requirements.get(name) {
            let needed = (quantity as f64 / *count as f64).ceil() as i64;
            for chemical in chemicals {
                if chemical.name == "ORE" {
                    ore += chemical.quantity * needed;
                } else {
                    *need.entry(chemical.name).or_default() += chemical.quantity * needed;
                }
                let extra = needed * count - quantity;
                if extra > 0 {
                    *have.entry(name).or_default() += extra;
                }
            }
        }
    }
    ore
}

pub fn part1(input: String) {
    let requirements = input.lines().fold(
        HashMap::<&str, (i64, Vec<Chemical>)>::new(),
        |mut map, line| {
            let mut split = line.split(" => ");
            let (left, right) = (
                split.next().unwrap_or_default(),
                split.next().unwrap_or_default(),
            );
            let output = Chemical::from(right);
            map.insert(
                output.name,
                (
                    output.quantity,
                    left.split(", ").map(Chemical::from).collect_vec(),
                ),
            );
            map
        },
    );

    println!("{}", ore_required(1, &requirements));

    // I used this to binary search around for the solution, then eyeballed it until I got the right answer
    let mut fuel = 10000000i64;
    let mut delta = 10000000i64 / 2;
    while delta > 0 {
        let required = ore_required(fuel, &requirements);
        println!("Part 2: {} {} {}", fuel, required, delta);
        if required > 1000000000000 {
            fuel -= delta;
        } else {
            fuel += delta;
        }
        delta /= 2;
    }
}
