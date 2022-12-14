use crate::util;
use itertools;
use std::collections::{HashSet, VecDeque};

const BUFFER_SIZE: usize = 14;

fn all_unique(col: &VecDeque<char>) -> bool {
    let unique_chars: HashSet<&char> = HashSet::from_iter(col.iter());
    return unique_chars.len() == col.len();
}

pub fn solve() {
    let input = util::load_input_file("src/y2022/day6/input.txt");
    println!("Recieved {} characters", input.len());

    let mut buffer: VecDeque<char> = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        println!("{}{}", itertools::join(&buffer, ""), c);
        buffer.push_back(c);
        if buffer.len() >= BUFFER_SIZE {
            if all_unique(&buffer) {
                println!("Size {} marker ends at i={} char={}", BUFFER_SIZE, i, i + 1);
                return;
            }
            buffer.pop_front();
        }
    }
}
