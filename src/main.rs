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
    fn encode(self, instruction : Instruction) -> u32 {
        match instruction {
            Addi => 0b0,
            Sw => 0b0,
        }
    }
}

fn split_string_into_lines(input: String)  ->  Vec<String> {
    input.lines().map(|line| line.trim().to_string()).collect()
}

fn split_string_by_whitespace(input: String) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}

fn assemble(input: Vec<u8>) -> Vec<u32> {
    unimplemented!();
} 

fn parse_instruction(tokens: Vec<&str>) -> Option<Instruction> {
    if tokens.is_empty()  { return None }

    match tokens[0] {
        "addi" => {
            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Addi(Addi { rd, rs1, imm }))
        }
        "sw" => {
            let rs1 = tokens[1][1..].parse().ok()?;
            let rs2 = tokens[2][1..].parse().ok()?;
            let offset = tokens[3].parse().ok()?;
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
