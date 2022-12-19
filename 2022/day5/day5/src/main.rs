use std::io;

fn main() {
    let mut stacks = populate_stacks();
    play_rearrangement(&mut stacks);
    print_last_of_each_stack(&stacks);
}

fn populate_stacks() -> Vec<Vec<char>> {
    let mut input_as_strings = Vec::new();
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().len() == 0 { break; } // Reached end of input
        input_as_strings.push(
            buffer.replace("    ", " [0]") // Replace empty space on stacks with empty_crates
        );
        buffer.clear();
    }
    input_as_strings.pop();
    input_as_strings.reverse();
    let mut stacks = Vec::new();
    for line in input_as_strings.iter_mut() {
        let crates: Vec<char> = line // turn input lines to collection of chars
            .split(' ')
            .map(|x| x.chars().nth(1).unwrap())
            .collect();
        for (col, _crate) in crates.iter().enumerate() { // parse lines to respective stacks
            if stacks.len() <= col { stacks.push(Vec::new()); }
            if _crate != &'0' { stacks[col].push(_crate.to_owned()); } // ignore empty_crates
        }
    }
    stacks
}

fn play_rearrangement(stacks: &mut Vec<Vec<char>>) {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().len() == 0 { break; } // Reached end of input
        let line: Vec<&str> = buffer.trim().split(' ').collect();
        let mut rearrangement_move = Vec::new();
        rearrangement_move.push(line[1].parse::<usize>().unwrap());
        rearrangement_move.push(line[3].parse::<usize>().unwrap()-1);
        rearrangement_move.push(line[5].parse::<usize>().unwrap()-1);
        buffer.clear();
        for _ in 0..rearrangement_move[0] {
            let from = rearrangement_move[1];
            let to = rearrangement_move[2];
            let _crate = stacks[from].pop().unwrap();
            stacks[to].push(_crate);
        }
    }
}

fn print_last_of_each_stack(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", i+1, stack.last());
    }
}