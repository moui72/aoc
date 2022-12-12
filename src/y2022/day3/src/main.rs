use std::str;

fn priority_from_byte(byte: u8) -> u8 {
    if byte > 95 {
        return byte - 96;
    } else {
        return byte - 38;
    }
}

fn find_redundant_priority(input: Vec<u8>) {
    let mut priority_sum: i32 = 0;
    let sacks = input.split(|c| c == &{ 0xA as u8 });

    for sack in sacks {
        let halfway = sack.len() / 2;
        let second_half = &sack[halfway..];
        for char in sack[..halfway].to_vec() {
            if second_half.contains(&char) {
                priority_sum += priority_from_byte(char) as i32;
                break;
            }
        }
    }
    println!("Total priority of wrongly sorted items is {}", priority_sum)
}

fn find_badge_priority(input: Vec<u8>) {
    let mut priority_sum: i32 = 0;
    let sacks: Vec<&[u8]> = input.split(|c| c == &{ 0xA as u8 }).collect();

    let teams = sacks.chunks(3);

    println!("There are {} elves and {} teams", sacks.len(), teams.len());

    for team in teams {
        for c in team[0] {
            if team[1].contains(c) && team[2].contains(c) {
                println!(
                    "Badge is {} ({})",
                    str::from_utf8(&[*c]).unwrap(),
                    priority_from_byte(*c)
                );
                priority_sum += priority_from_byte(*c) as i32;
                break;
            }
        }
    }
    println!("Total priority of badges is {}", priority_sum)
}

fn main() {
    let input = std::fs::read("input.txt").unwrap();
    find_redundant_priority(input.clone());
    println!("======");
    find_badge_priority(input.clone());
}
