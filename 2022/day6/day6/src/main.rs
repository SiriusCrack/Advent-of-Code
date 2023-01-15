use std::{io, collections::HashSet};

fn main() {
    let datastream = populate_datasastream();
    let marker = find_marker(datastream);
    println!("{}", marker);
}

fn populate_datasastream() -> Vec<char> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let datastream: Vec<char> = buffer.chars().collect();
    datastream
}

fn find_marker(datastream: Vec<char>) -> usize {
    let sequence_size = 14;
    let mut marker = 0;
    for (i, window) in datastream.windows(sequence_size).enumerate() {
        let set: HashSet<&char> = window.iter().collect();
        if set.len() == sequence_size {
            marker = i+sequence_size;
            break;
        }
    }
    marker
}