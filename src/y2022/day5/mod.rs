use crate::stack::Stack;
use crate::util;

fn peek_stacks(stacks: &Vec<Stack<char>>) {
    for stack in stacks.iter() {
        print!("{}", stack.peek().unwrap_or(&'-'));
    }
    println!()
}

pub fn solve() {
    let input = util::load_input_file("src/y2022/day5/input.txt");
    let mut stacks: Vec<Stack<char>> = Vec::from([
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
    ]);
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut initial_state = parts[0].split("\n").collect::<Vec<_>>();
    initial_state.reverse();
    for line in initial_state {
        if line.is_empty() {
            break;
        }
        let split_line = line.clone().chars().collect::<Vec<_>>();
        let chunky = split_line.chunks(4);
        for (i, chunk) in chunky.enumerate() {
            if chunk[0] == '[' {
                stacks[i].push(chunk[1])
            }
        }
        println!("{:?}", line);
    }
    peek_stacks(&stacks);
    for crane_move in parts[1].split('\n') {
        if crane_move.is_empty() {
            break;
        }
        let instr: Vec<&str> = crane_move.split_whitespace().collect();
        let mut n: i32 = instr[1].parse().unwrap();
        let src: usize = instr[3].parse::<usize>().unwrap() - 1;
        let dst: usize = instr[5].parse::<usize>().unwrap() - 1;
        let mut items: Vec<char> = Vec::new();
        while n > 0 {
            items.push(stacks[src].pop().unwrap());
            n -= 1
        }
        while items.len() > 0 {
            stacks[dst].push(items.pop().unwrap());
        }
    }
    peek_stacks(&stacks);
}
