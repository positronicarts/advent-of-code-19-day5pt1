const ADD : u32 = 1;
const MULTIPLY : u32 = 2;
const INPUT : u32 = 3;
const OUTPUT : u32 = 4;
const EXIT : u32 = 99;

const DIRECT : char = '1';
const INDIRECT : char = '0';

fn run(mut memory: Vec<i64>, input: i64) -> i64 {
    let mut index = 0;

    loop {
        let clone = memory.clone();

        let instruction = format!("{}", memory[index]);
        let mut instruction_chars : Vec<char> = instruction.chars().collect();
        let opcode = instruction_chars.pop().unwrap().to_digit(10).unwrap() + (instruction_chars.pop().unwrap_or('0').to_digit(10).unwrap()) * 10;
        print!("opcode {}:  ", opcode);
        let jump = match opcode {
            ADD => {
                // add
                let v1 = match instruction_chars.pop().unwrap_or('0') {
                    INDIRECT => clone[clone[index + 1] as usize],
                    _ => clone[index + 1]
                };
                let v2 = match instruction_chars.pop().unwrap_or('0') {
                    INDIRECT => clone[clone[index + 2] as usize],
                    _ => clone[index + 2]
                };
                println!("{} + {} -> {}", v1, v2, clone[index + 3]);
                memory[clone[index + 3] as usize] = v1 + v2;
                4
            },
            MULTIPLY => {
                // multiply
                let v1 = match instruction_chars.pop().unwrap_or('0') {
                    INDIRECT => clone[clone[index + 1] as usize],
                    _ => clone[index + 1]
                };
                let v2 = match instruction_chars.pop().unwrap_or('0') {
                    INDIRECT => clone[clone[index + 2] as usize],
                    _ => clone[index + 2]
                };
                println!("{} * {} -> {}", v1, v2, clone[index + 3]);
                memory[clone[index + 3] as usize] = v1 * v2;
                4
            },
            INPUT => {
                memory[clone[index + 1] as usize] = input;
                2
            },
            OUTPUT => {
                // output
                let v1 = match instruction_chars.pop().unwrap_or('0') {
                    INDIRECT => clone[clone[index + 1] as usize],
                    _ => clone[index + 1]
                };

                if v1 != 0 {
                    if clone[index + 2] as u32 == EXIT {
                        return v1;
                    } else {
                        panic!("Diagnostic error!");
                    }
                }
                2
            },
            99 => {
                break;
            },
            x => {
                panic!("WTF! Value {} in memory {:?}", x, memory);
            }
        };

        index += jump;
    }

    memory[0]
}

fn main() {
    let memory : Vec<i64> = std::fs::read_to_string("inputs.txt").unwrap().split(",").map(|input| input.clone().parse::<i64>().unwrap()).collect();
    print!("{}", run(memory, 1));
}
