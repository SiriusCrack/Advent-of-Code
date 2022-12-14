use std::{collections::HashSet, io};

fn main() {
    let mut sum = 0;
    let mut buffer = String::new();
    'prime_loop: loop {
        let mut group = [HashSet::new(), HashSet::new(), HashSet::new()];
        for i in 0..3 {
            io::stdin().read_line(&mut buffer).expect("input should be readable");
            if buffer.trim().len() == 0 { break 'prime_loop; } // Reached end of input
            let rucksack: HashSet<char> = buffer.trim().chars().collect();
            group[i] = rucksack;
            buffer.clear();
        }
        let mistakes = group[0]
            .intersection(&group[1]).copied().collect::<HashSet<char>>()
            .intersection(&group[2]).copied().collect::<Vec<char>>();
        if mistakes.len() != 1 { panic!("rucksack should have only one mistake, not {}", mistakes.len()); }
        let mistake = mistakes[0];
        match mistake.is_uppercase() {
            true => sum = sum + mistake as u32 - 38,
            false => sum = sum + mistake as u32 - 96
        }
    }
    println!("Sum: {}", sum);
}