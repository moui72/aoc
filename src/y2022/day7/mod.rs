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
}
