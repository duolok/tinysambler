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

    // S Type Instructions
    Addi(Addi),
    Slti(Slti),
    Sltiu(Sltiu),
    Xori(Xori),
    Ori(Ori),
    Andi(Andi),

    Sw(Sw),
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

            }
            Instruction::SType(stype) => {
            }
            Instruction::BType(btype) => {
            }
            Instruction::UType(utype) => {
            }
            Instruction::JType(jtype) => {
            }



            // // I type instructions
            // Instruction::Addi(addi) => {
            //     let imm = (addi.imm as u32) & 0xFFF;
            //     0b0010011
            //         | ((addi.rd as u32) << 7)
            //         | (0b000 << 12)
            //         | ((addi.rs1 as u32) << 15)
            //         | (imm << 20)
            // }
            // Instruction::Slti(slti) => {
            //     let imm = (slti.imm as u32) & 0xFFF;
            //     0b0010011
            //         | ((slti.rd as u32) << 7)
            //         | (0b010 << 12)
            //         | ((slti.rs1 as u32) << 15)
            //         | (imm << 20)
            // }
            // Instruction::Sltiu(sltiu) => {
            //     let imm = (sltiu.imm as u32) & 0xFFF;
            //     0b0010011
            //         | ((sltiu.rd as u32) << 7)
            //         | (0b011 << 12)
            //         | ((sltiu.rs1 as u32) << 15)
            //         | (imm << 20)
            // }
            //
            // Instruction::Xori(xori) => {
            //     let imm = (xori.imm as u32) & 0xFFF;
            //     0b0010011
            //         | ((xori.rd as u32) << 7)
            //         | (0b100 << 12)
            //         | ((xori.rs1 as u32) << 15)
            //         | (imm << 20)
            // }
            // Instruction::Ori(ori) => {
            //     let imm = (ori.imm as u32) & 0xFFF;
            //     0b0010011
            //         | ((ori.rd as u32) << 7)
            //         | (0b110 << 12)
            //         | ((ori.rs1 as u32) << 15)
            //         | (imm << 20)
            // }
            // Instruction::Andi(andi) => {
            //     let imm = (andi.imm as u32) & 0xFFF;
            //     0b0010011
            //         | ((andi.rd as u32) << 7)
            //         | (0b111 << 12)
            //         | ((andi.rs1 as u32) << 15)
            //         | (imm << 20)
            // }
            //
            // // S type instructions
            // Instruction::Sw(sw) => {
            //     let imm11_5 = ((sw.offset as u32) & 0xFE0) << 20;
            //     let imm4_0 = ((sw.offset as u32) & 0x1F) << 7;
            //     0b0100011
            //         | imm4_0
            //         | (0b010 << 12)
            //         | ((sw.rs1 as u32) << 15)
            //         | ((sw.rs2 as u32) << 20)
            //         | imm11_5
            // }
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

        "Xori" => {
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
