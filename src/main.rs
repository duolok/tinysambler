struct Addi {
    rd: u8,
    rs1: u8,
    imm: i16,
}

struct Sw {
    rs1: u8,
    rs2: u8,
    offset: i16
}

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

fn split_string_into_lines(input: [u8])  ->  String {
    unimplemented!()
}

fn split_string_by_whitespace(input: [u8]) -> String {
    unimplemented!();
}

fn assemble(input: [u8]) -> Vec<u8> {
    unimplemented!();
} 

fn parse_instruction(tokens: Vec<Vec<u8>>) -> Instruction {
    unimplemented!();
}

fn main() {
    println!("Hello, world!");
}
