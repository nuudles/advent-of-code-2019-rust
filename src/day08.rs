use counter::Counter;
use itertools::Itertools;

pub fn part1(input: String) {
    let (width, height) = (25usize, 6usize);

    let mut min = (usize::MAX, 0usize, 0usize);
    for chunk in input.chars().chunks(width * height).into_iter() {
        let counter: Counter<_> = chunk.collect();
        if counter[&'0'] < min.0 {
            min = (counter[&'0'], counter[&'1'], counter[&'2']);
        }
    }
    println!("{}", min.1 * min.2);
}

pub fn part2(input: String) {
    let (width, height) = (25usize, 6usize);
    let mut pixels = [['0'; 25]; 6];
    for chunk in input.chars().chunks(width * height).into_iter() {
        for (y, row) in chunk.chunks(width).into_iter().enumerate() {
            for (x, c) in row.enumerate() {
                if pixels[y][x] != '0' || c == '2' {
                    continue;
                }
                pixels[y][x] = if c == '1' { '█' } else { ' ' };
            }
        }
    }
    (0..height).for_each(|y| {
        for x in 0..width {
            print!("{}", pixels[y][x]);
        }
        println!();
    });
}
