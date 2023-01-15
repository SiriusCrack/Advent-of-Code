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
    let mut marker = 0;
    for (i, window) in datastream.windows(4).enumerate() {
        let set: HashSet<&char> = window.iter().collect();
        if set.len() == 4 {
            marker = i+4;
            break;
        }
    }
    marker
}