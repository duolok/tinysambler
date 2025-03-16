use std::fmt::Write;

#[allow(unused_features)]
// add immediate instruction
#[derive(Debug)]
struct Addi {
    // destintaion register
    rd: u8,

    // source register
    rs1: u8,

    // immediate value
    imm: i16,
}

// store word
#[derive(Debug)]
struct Sw {
    // store register 1
    rs1: u8,

    // store register 2
    rs2: u8,

    // offset value
    offset: i16,
}

#[derive(Debug)]
struct Slti {
    rd: u8,
    rs1: u8,
    imm: u8,
}

#[derive(Debug)]
struct Sltiu {
    rd: u8,
    rs1: u8,
    imm: u8,
}

#[derive(Debug)]
struct Xori {
    rd: u8,
    rs1: u8,
    imm: u8,
}

#[derive(Debug)]
struct Ori {
    rd: u8,
    rs1: u8,
    imm: u8,
}

#[derive(Debug)]
struct Andi {
    rd: u8,
    rs1: u8,
    imm: u8,
}

#[derive(Debug)]
enum RTypeInstruction {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Srl,
    Sra,
    Xor,
    Or,
    And,
}

#[derive(Debug)]
enum ITypeInstruction {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Jalr,
}

#[derive(Debug)]
enum STypeInstruction {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug)]
enum BTypeInstruction {
    Beq,
    Bne,
    Blt,
    Bge,
    BLtu,
    Bgeu,
}

#[derive(Debug)]
enum UTypeInstruction {
    Lui,
    Auipc,
}

#[derive(Debug)]
enum JTypeInstruction {
    Jal,
}

#[derive(Debug)]
struct RType {
    instruction: RTypeInstruction,
    rd: u8,
    rs1: u8,
    rs2: u8,
}


#[derive(Debug)]
struct IType {
    instruction: ITypeInstruction,
    rd: u8,
    rs1: u8,
    imm: i16,
}

#[derive(Debug)]
struct SType {
    instruction: STypeInstruction,
    rs1: u8,
    rs2: u8,
    imm: i16,
}

#[derive(Debug)]
struct BType {
    instruction: BTypeInstruction,
    rs1: u8,
    rs2: u8,
    imm: i16,
}

#[derive(Debug)]
struct UType {
    instruction: UTypeInstruction,
    rd: u8,
    imm: i32,
}

#[derive(Debug)]
struct JType {
    instruction: JTypeInstruction,
    rd: u8,
    imm: i32,
}

#[derive(Debug)]
enum Instruction {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
}

impl Instruction {
    // translaates aassembly instructions into machine code for a RISC-V cpu
    fn encode(&self) -> u32 {
        match self {
            Instruction::RType(rtype) => {
                let funct3 = match rtype.instruction {
                    RTypeInstruction::Add | RTypeInstruction::Sub => 0b000,
                    RTypeInstruction::Sll => 0b001,
                    RTypeInstruction::Slt => 0b001,
                    RTypeInstruction::Sltu => 0b001,
                    RTypeInstruction::Xor => 0b001,
                    RTypeInstruction::Srl => 0b001,
                    RTypeInstruction::Sra => 0b001,
                    RTypeInstruction::Or => 0b001,
                    RTypeInstruction::And => 0b001,
                };

                let funct7 = match rtype.instruction {
                    RTypeInstruction::Add => 0b0000000,
                    RTypeInstruction::Sub => 0b0100000,
                    RTypeInstruction::Sll => 0b0000000,
                    RTypeInstruction::Slt => 0b0000000,
                    RTypeInstruction::Sltu => 0b0000000,
                    RTypeInstruction::Xor => 0b0000000,
                    RTypeInstruction::Srl => 0b0000000,
                    RTypeInstruction::Sra => 0b0100000,
                    RTypeInstruction::Or => 0b0000000,
                    RTypeInstruction::And => 0b0000000,
                };

               0b110011 | ((rtype.rd as u32) << 7) | (funct3 << 12)  | ((rtype.rs1 as u32) << 15) | ((rtype.rs2 as u32) << 20) | (funct7 << 25)

            }
            Instruction::IType(itype) => {
                let funct3 = match itype.instruction {
                    ITypeInstruction::Addi => 0b000,
                    ITypeInstruction::Slti => 0b010,
                    ITypeInstruction::Sltiu => 0b011,
                    ITypeInstruction::Xori => 0b100,
                    ITypeInstruction::Ori => 0b110,
                    ITypeInstruction::Andi => 0b111,
                    ITypeInstruction::Lb => 0b000,
                    ITypeInstruction::Lh => 0b001,
                    ITypeInstruction::Lw => 0b010,
                    ITypeInstruction::Lbu => 0b100,
                    ITypeInstruction::Lhu => 0b101,
                    ITypeInstruction::Jalr => 0b000,
                };


                let opcode = match itype.instruction {
                    ITypeInstruction::Addi | ITypeInstruction::Slti | ITypeInstruction::Sltiu | 
                    ITypeInstruction::Xori | ITypeInstruction::Ori  | ITypeInstruction::Andi => 0b0010011,
                    ITypeInstruction::Lb  | ITypeInstruction::Lh | ITypeInstruction::Lw |
                    ITypeInstruction::Lbu | ITypeInstruction::Lhu => 0b0000011,
                    ITypeInstruction::Jalr => 0b1100111,
                };

                let imm = (itype.imm as u32) & 0xFFF;

                opcode | ((itype.rd as u32) << 7) | (funct3 << 12) | ((itype.rs1 as u32) << 15) | (imm << 20)
            }
            Instruction::SType(stype) => {
                let funct3 = match stype.instruction {
                    STypeInstruction::Sb => 0b000,
                    STypeInstruction::Sh => 0b010,
                    STypeInstruction::Sw => 0b100,
                };
                

                let imm11_5 = ((stype.imm as u32) & 0xFE0) << 20;
                let imm4_0 = ((stype.imm as u32) & 0x1F) << 7;

                0b0100011 | imm4_0 | (funct3 << 12) | ((stype.rs1 as u32) << 15) | ((stype.rs2 as u32) << 20) | imm11_5
            }
            Instruction::BType(btype) => {
                let funct3 = match btype.instruction {
                    BTypeInstruction::Beq  => 0b000,
                    BTypeInstruction::Bne  => 0b001,
                    BTypeInstruction::Blt  => 0b100,
                    BTypeInstruction::Bge  => 0b101,
                    BTypeInstruction::BLtu => 0b110,
                    BTypeInstruction::Bgeu => 0b111,
                };

                let imm11 = ((btype.imm as u32) & 0x800) << 20;
                let imm4_1 = ((btype.imm as u32) & 0x1E) << 7;
                let imm10_5 = ((btype.imm as u32) & 0x7E0) << 20;
                let imm12 = ((btype.imm as u32) & 0x1000) << 19;

                0b1100011 | imm11 | imm4_1 | (funct3 << 12) | ((btype.rs1 as u32) << 15) | ((btype.rs2 as u32) << 20) | imm10_5 | imm12

            }
            Instruction::UType(utype) => {
                let opcode = match utype.instruction {
                    UTypeInstruction::Lui => 0b0110111,
                    UTypeInstruction::Auipc => 0b0010111
                };

                let imm31_12 = (utype.imm as u32) << 12 & 0xFFFFF000;

                opcode | ((utype.rd as u32) << 7) | imm31_12

            }
            Instruction::JType(jtype) => {
                let imm20 = ((jtype.imm as u32) & 0x80000)  << 11;
                let imm10_1 = ((jtype.imm as u32) & 0x7FE)  << 20;
                let imm11 = ((jtype.imm as u32) & 0x100000) << 9;
                let imm19_12 = ((jtype.imm as u32) & 0xFF000) << 1;
                let opcode: u32 = 0b1101111;

                opcode | ((jtype.rd as u32) << 7) | imm19_12 | imm11 | imm10_1 | imm20

            }

        }
    }
}

fn split_string_into_lines(input: String) -> Vec<String> {
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
    if tokens.is_empty() {
        return None;
    }

    match tokens[0].as_str() {
        "addi" => {
            //rd and rs1 are registers, which are indicated by a letter  followed by a number.
            //the [1..] slice removes the letter, leaving the number which is parsed into a u8.
            if tokens.len() != 4 {
                return None;
            }

            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Addi(Addi { rd, rs1, imm }))
        }

        "slti" => {
            if tokens.len() != 4 {
                return None;
            }

            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Slti(Slti { rd, rs1, imm }))
        }

        "sltiu" => {
            if tokens.len() != 4 {
                return None;
            }

            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Sltiu(Sltiu { rd, rs1, imm }))
        }

        "xori" => {
            if tokens.len() != 4 {
                return None;
            }

            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Xori(Xori { rd, rs1, imm }))
        }

        "ori" => {
            if tokens.len() != 4 {
                return None;
            }

            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Ori(Ori { rd, rs1, imm }))
        }

        "andi" => {
            if tokens.len() != 4 {
                return None;
            }

            let rd = tokens[1][1..].parse().ok()?;
            let rs1 = tokens[2][1..].parse().ok()?;
            let imm = tokens[3].parse().ok()?;
            Some(Instruction::Andi(Andi { rd, rs1, imm }))
        }

        "sw" => {
            if tokens.len() != 3 {
                return None;
            }
            let rs2 = tokens[1][1..].parse().ok()?;

            // split the third token on '(' or ')' to separate the offset and the rs1 register
            let offset_and_rs1: Vec<&str> = tokens[2].split(|c| c == '(' || c == ')').collect();
            if offset_and_rs1.len() != 2 {
                return None;
            }

            // parse the offset and rs1
            let offset = offset_and_rs1[0].parse().ok()?;
            let rs1 = offset_and_rs1[1][1..].parse().ok()?;

            Some(Instruction::Sw(Sw { rs1, rs2, offset }))
        }
        _ => None,
    }
}

fn main() -> Result<(), String> {
    let source_code = "addi x2 x2 -4\nsw x10 0(x2)";

    let machine_code = assemble(source_code.to_string());

    for code in machine_code {
        let mut binary = String::new();

        write!(&mut binary, "{:032b}", code).unwrap();

        println!("{}", binary);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addi_encoding() {
        let addi = Instruction::Addi(Addi {
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        let encoded = addi.encode();
        let expected = 0b00000000010000010000000100010011;
        assert_eq!(
            encoded, expected,
            "Failed encoding 'addi x2, x2, 4'\nExpected: {:#034b}\nActual:   {:#034b}",
            expected, encoded
        );
    }
}
