use std::io;

struct Elf { lower: u32, upper: u32 }

fn main() {
    let elves = populate_elves();
    let sum = calculate_overlap(elves);
    println!("sum: {sum}");
}

fn populate_elves() -> Vec<(Elf, Elf)> {
    let mut elf_pairs = Vec::new();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().len() == 0 { break; } // Reached end of input
        let pair: Vec<&str> = buffer.trim().split(',').collect();
        if pair.len() != 2 { panic!("input line should conatain two elves."); }
        let elf1: Vec<&str> = pair[0].split('-').collect();
        if elf1.len() != 2 { panic!("elf should have upper and lower bound."); }
        let elf2: Vec<&str> = pair[1].split('-').collect();
        if elf1.len() != 2 { panic!("elf should have upper and lower bound."); }
        let elf1 = Elf{lower: elf1[0].parse().unwrap(), upper: elf1[1].parse().unwrap()};
        let elf2 = Elf{lower: elf2[0].parse().unwrap(), upper: elf2[1].parse().unwrap()};
        elf_pairs.push((elf1, elf2));
    }
    elf_pairs
}

fn calculate_overlap(elves: Vec<(Elf, Elf)>) -> u32 {
    let mut sum = 0;
    for pair in elves.iter() {

        if pair.0.upper >= pair.1.lower && !(pair.0.lower > pair.1.upper) {
            sum = sum + 1;
        }
    }
    sum
}