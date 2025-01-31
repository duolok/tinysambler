#[derive(Debug)]
struct Addi {
    rd: u8,
    rs1: u8,
    imm: i16,
}

#[derive(Debug)]
struct Sw {
    rs1: u8,
    rs2: u8,
    offset: i16,
}

#[derive(Debug)]
enum Instruction {
    Addi(Addi),
    Sw(Sw),
}

impl Instruction {
    fn encode(&self) -> u32 {
        match self {
            Instruction::Addi(addi) => {
                0b0
            },
            Instruction::Sw(sw) => {
                0b0
            }
        }
    }
}

fn split_string_into_lines(input: String)  ->  Vec<String> {
    input.lines().map(|line| line.trim().to_string()).collect()
}

fn split_string_by_whitespace(input: String) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}


fn assemble(input: String) -> Vec<u32> {
    let lines = split_string_into_lines(input);
    let mut machine_codes = Vec::new();
    for line in lines {
        let tokens = split_string_by_whitespace(line);
        if let Some(instruction) = parse_instruction(tokens) {
            machine_codes.push(instruction.encode());
        }
    }
    machine_codes
} 


fn parse_instruction(tokens: Vec<String>) -> Option<Instruction> {
    if tokens.is_empty()  { return None }

    match tokens[0].as_str() {
        "addi" => {
            //rd and rs1 are registers, which are indicated by a letter  followed by a number.
            //the [1..] slice removes the letter, leaving the number which is parsed into a u8.

            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Addi(Addi { rd, rs1, imm }))
        }
        "sw" => {
            if tokens.len() != 3 {
                return None;
            }
            let rs2 = tokens[1][1..].parse().ok()?;

            // split the third token on '(' or ')' to separate the offset and the rs1 register
            let offset_and_rs1: Vec<&str> = tokens[2].split(|c| c == '(' || c == ')').collect();
            if offset_and_rs1.len() != 2 {
                return None
            }

            // parse the offset and rs1
            let offset = offset_and_rs1[0].parse().ok()?;
            let rs1 = offset_and_rs1[1][1..].parse().ok()?;

            Some(Instruction::Sw(Sw { rs1, rs2, offset }))
        }
        _ => None
    }
}

fn main() {
    let input = String::from("test split whitespace  \n test split whitespace");
    let ayo: Vec<String> = split_string_into_lines(input.clone());
    let cool: Vec<String> = split_string_by_whitespace(input.clone());

    println!("{:?}", ayo);
    println!("{:?}", cool);
}
