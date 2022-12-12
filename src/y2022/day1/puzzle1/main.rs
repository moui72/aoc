use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::str::Split;

fn main() {
    let file_path = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let elves: Split<&str> = contents.split("\n\n");
    let mut elf_calorie_count = Vec::with_capacity(elves.clone().count());
    let mut cals_to_elf: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut max_calories = 0;
    let mut max_cal_elf: usize = 0;

    for (i, elf) in elves.enumerate() {
        let snacks: Split<&str> = elf.split("\n");
        elf_calorie_count.push(0);
        let mut elf_snacks = snacks.clone().count();
        if elf_snacks > 0 {
            for snack in snacks {
                if snack.is_empty() {
                    elf_snacks -= 1;
                    continue;
                }
                elf_calorie_count[i] += snack.parse::<i32>().unwrap();
            }
            println!(
                "Elf {} has {} snacks with a total of {} calories",
                i, elf_snacks, elf_calorie_count[i]
            );
            if elf_calorie_count[i] > max_calories {
                max_calories = elf_calorie_count[i];
                max_cal_elf = i;
            }
            cals_to_elf
                .entry(elf_calorie_count[i])
                .and_modify(|e| e.push(i))
                .or_insert(vec![i]);
        } else {
            println!("Elf {} has no snacks", i);
        }
    }

    println!(
        "Elf {} has {} calories, which is the most of any elf",
        max_cal_elf, max_calories
    );
    let mut most_cals_elves: Vec<i32> = cals_to_elf.clone().into_keys().collect();
    most_cals_elves.sort_unstable();

    let mut top_n = 3;
    let mut top_n_elf_calories: i32 = 0;
    let mut top_n_elves: Vec<String> = Vec::new();
    while top_n > 0 {
        let next = most_cals_elves.pop().unwrap();
        let num_elves_with_cals: usize = cals_to_elf[&next].len();
        for elf in &cals_to_elf[&next] {
            top_n_elves.push(elf.to_string())
        }
        let multiplier = min(num_elves_with_cals, top_n);
        top_n -= multiplier;
        top_n_elf_calories += next * multiplier as i32;
    }
    println!(
        "The 3 elves ({}) with the most cals have a total of {}",
        top_n_elves.join(" "),
        top_n_elf_calories
    )
}
