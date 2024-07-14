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
    offset: i16
}

#[derive(Debug)]
struct Instruction {
    addi: Addi,
    sw: Sw,
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

fn parse_instruction(tokens: Vec<Vec<u8>>) -> Instruction {
    unimplemented!();
}

fn main() {
    let input = String::from("test split whitespace  \n test split whitespace");
    let ayo: Vec<String> = split_string_into_lines(input.clone());
    let cool: Vec<String> = split_string_by_whitespace(input.clone());

    println!("{:?}", ayo);
    println!("{:?}", cool);
}
