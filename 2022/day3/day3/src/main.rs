use std::{io, collections::HashSet};

fn main() {
    let mut sum = 0;
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).expect("input should be readable");
        if buffer.trim().len() == 0 { break; } // Reached end of input
        let rucksack: Vec<char> = buffer.trim().chars().collect();
        let rucksack: Vec<&[char]> = rucksack.chunks(rucksack.len()/2).collect();
        if rucksack.len() != 2 { panic!("rucksack should be even"); }
        let set1: HashSet<&char> = rucksack[0].iter().collect();
        let set2: HashSet<&char> = rucksack[1].iter().collect();
        let mistakes: Vec<&&char> = set1.intersection(&set2).collect();
        if mistakes.len() != 1 { panic!("rucksack should have only one mistake, not {}", mistakes.len()); }
        let mistake = **mistakes[0];
        match mistake.is_uppercase() {
            true => sum = sum + mistake as u32 - 38,
            false => sum = sum + mistake as u32 - 96
        }
        buffer.clear();
    }
    println!("Sum: {}", sum);
}