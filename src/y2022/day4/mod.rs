use crate::util;
use std::collections::HashSet;

fn range_to_vec(range: &str) -> Vec<i32> {
    let ends: Vec<&str> = range.split('-').collect();
    (ends[0].parse().unwrap()..ends[1].parse::<i32>().unwrap() + 1).collect()
}

pub fn solve() {
    let input = util::load_input_file("src/y2022/day4/input.txt");
    let pairs = input.split("\n");
    let mut overlaps = 0;
    for pair in pairs {
        if pair.is_empty() {
            break;
        }
        let ranges: Vec<&str> = pair.split(",").collect();
        let range1: HashSet<i32> = HashSet::from_iter(range_to_vec(ranges[0]).iter().cloned());
        let range2: HashSet<i32> = HashSet::from_iter(range_to_vec(ranges[1]).iter().cloned());

        if range1.is_subset(&range2) || range1.is_superset(&range2) {
            overlaps += 1;
            println!("Overlap found: {}", pair);
            println!("{:?}\n{:?}", range1, range2);
        }
    }
    println!("Found {} overlaps", overlaps);
}

pub fn solve2() {
    let input = util::load_input_file("src/y2022/day4/input.txt");
    let pairs = input.split("\n");
    let mut overlaps = 0;
    for pair in pairs {
        if pair.is_empty() {
            break;
        }
        let ranges: Vec<&str> = pair.split(",").collect();
        let range1: HashSet<i32> = HashSet::from_iter(range_to_vec(ranges[0]).iter().cloned());
        let range2: HashSet<i32> = HashSet::from_iter(range_to_vec(ranges[1]).iter().cloned());

        if !range1.is_disjoint(&range2) {
            overlaps += 1;
            println!("Overlap found: {}", pair);
        }
    }
    println!("Found {} overlaps", overlaps);
}
