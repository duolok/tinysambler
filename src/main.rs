use std::fmt::Write;
use std::collections::HashMap;

#[allow(unused_features)]


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
    Bltu,
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
    _instruction: JTypeInstruction,
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
                    RTypeInstruction::Slt => 0b010,
                    RTypeInstruction::Sltu => 0b011,
                    RTypeInstruction::Xor => 0b100,
                    RTypeInstruction::Srl => 0b101,
                    RTypeInstruction::Sra => 0b101,
                    RTypeInstruction::Or => 0b110,
                    RTypeInstruction::And => 0b111,
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

                0b0110011 | ((rtype.rd as u32) << 7) | ((funct3 as u32) << 12) | ((rtype.rs1 as u32) << 15) | ((rtype.rs2 as u32) << 20) | ((funct7 as u32) << 25)
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
                    ITypeInstruction::Addi
                    | ITypeInstruction::Slti
                    | ITypeInstruction::Sltiu
                    | ITypeInstruction::Xori
                    | ITypeInstruction::Ori
                    | ITypeInstruction::Andi => 0b0010011,
                    ITypeInstruction::Lb
                    | ITypeInstruction::Lh
                    | ITypeInstruction::Lw
                    | ITypeInstruction::Lbu
                    | ITypeInstruction::Lhu => 0b0000011,
                    ITypeInstruction::Jalr => 0b1100111,
                };

                let imm = (itype.imm as u32) & 0xFFF;

                opcode | ((itype.rd as u32) << 7) | (funct3 << 12) | ((itype.rs1 as u32) << 15) | (imm << 20)
            }
            Instruction::SType(stype) => {
                let funct3 = match stype.instruction {
                    STypeInstruction::Sb => 0b000,
                    STypeInstruction::Sh => 0b001,
                    STypeInstruction::Sw => 0b010,
                };

                let imm11_5 = ((stype.imm as u32) & 0xFE0) << 20;
                let imm4_0 = ((stype.imm as u32) & 0x1F) << 7;

                0b0100011 | imm4_0 | (funct3 << 12) | ((stype.rs1 as u32) << 15) | ((stype.rs2 as u32) << 20) | imm11_5
            }
            Instruction::BType(btype) => {
                let funct3 = match btype.instruction {
                    BTypeInstruction::Beq => 0b000,
                    BTypeInstruction::Bne => 0b001,
                    BTypeInstruction::Blt => 0b100,
                    BTypeInstruction::Bge => 0b101,
                    BTypeInstruction::Bltu => 0b110,
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
                    UTypeInstruction::Auipc => 0b0010111,
                };

                let imm31_12 = (utype.imm as u32) << 12 & 0xFFFFF000;

                opcode | ((utype.rd as u32) << 7) | imm31_12
            }
            Instruction::JType(jtype) => {
                let imm20 = ((jtype.imm as u32) & 0x80000) << 11;
                let imm10_1 = ((jtype.imm as u32) & 0x7FE) << 20;
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
            println!("got machine code: {:?}", &instruction);
            machine_codes.push(instruction.encode());
        }
    }
    machine_codes
}

fn parse_instruction(tokens: Vec<String>) -> Option<Instruction> {
    let reg_map = create_register_map();

    if tokens.is_empty() {
        return None;
    }

    match tokens[0].as_str() {
        // R type instructions
        "add" => parse_rtype(&tokens, RTypeInstruction::Add, &reg_map),
        "sub" => parse_rtype(&tokens, RTypeInstruction::Sub, &reg_map),
        "sll" => parse_rtype(&tokens, RTypeInstruction::Sll, &reg_map),
        "slt" => parse_rtype(&tokens, RTypeInstruction::Slt, &reg_map),
        "sltu" => parse_rtype(&tokens, RTypeInstruction::Sltu, &reg_map),
        "xor" => parse_rtype(&tokens, RTypeInstruction::Xor, &reg_map),
        "srl" => parse_rtype(&tokens, RTypeInstruction::Srl, &reg_map),
        "sra" => parse_rtype(&tokens, RTypeInstruction::Sra, &reg_map),
        "or" => parse_rtype(&tokens, RTypeInstruction::Or, &reg_map),
        "and" => parse_rtype(&tokens, RTypeInstruction::And, &reg_map),

        // I type instructions
        "addi" => parse_itype(&tokens, ITypeInstruction::Addi, &reg_map),
        "slti" => parse_itype(&tokens, ITypeInstruction::Slti, &reg_map),
        "sltiu" => parse_itype(&tokens, ITypeInstruction::Sltiu, &reg_map),
        "xori" => parse_itype(&tokens, ITypeInstruction::Xori, &reg_map),
        "ori" => parse_itype(&tokens, ITypeInstruction::Ori, &reg_map),
        "andi" => parse_itype(&tokens, ITypeInstruction::Andi, &reg_map),
        "lb" => parse_itype(&tokens, ITypeInstruction::Lb, &reg_map),
        "lh" => parse_itype(&tokens, ITypeInstruction::Lh, &reg_map),
        "lw" => parse_itype(&tokens, ITypeInstruction::Lw, &reg_map),
        "lbu" => parse_itype(&tokens, ITypeInstruction::Lbu, &reg_map),
        "lhu" => parse_itype(&tokens, ITypeInstruction::Lhu, &reg_map),
        "jalr" => parse_itype(&tokens, ITypeInstruction::Jalr, &reg_map),

        // S type instructions 
        "sb" => parse_stype(&tokens, STypeInstruction::Sb),
        "sh" => parse_stype(&tokens, STypeInstruction::Sh),
        "sw" => parse_stype(&tokens, STypeInstruction::Sw),
        

        // B type instructions
        "beq" => parse_btype(&tokens, BTypeInstruction::Beq, &reg_map),
        "bne" => parse_btype(&tokens, BTypeInstruction::Bne, &reg_map),
        "blt" => parse_btype(&tokens, BTypeInstruction::Blt, &reg_map),
        "bge" => parse_btype(&tokens, BTypeInstruction::Bge, &reg_map),
        "bltu" => parse_btype(&tokens, BTypeInstruction::Bltu, &reg_map),
        "bgeu" => parse_btype(&tokens, BTypeInstruction::Bgeu, &reg_map),

        // U type instructions
        "lui" => parse_utype(&tokens, UTypeInstruction::Lui, &reg_map),
        "auipc" => parse_utype(&tokens, UTypeInstruction::Auipc, &reg_map),

        // J type instructions
        "jal" => parse_jtype(&tokens, &reg_map),

        _ => None,
    }
}

fn create_register_map() -> HashMap<&'static str, u8> {
    let reg_names = [
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
        "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
        "t3", "t4", "t5", "t6",
    ];

    let mut map = HashMap::new();
    for (index, &name) in reg_names.iter().enumerate() {
        map.insert(name, index as u8);
    }

    map.insert("fp", 8);
    map
}

// currently only parsing registers x0-x31
fn parse_register(reg: &str, reg_map: &HashMap<&'static str, u8>) -> Option<u8> {
    if reg.is_empty() { return None; }

    if reg.starts_with('x') {
        return reg[1..].parse::<u8>().ok();
    } 
    reg_map.get(reg).copied()
}

fn parse_rtype(tokens: &[String], instruction: RTypeInstruction, reg_map: &HashMap<&'static str, u8>) -> Option<Instruction> {
    if tokens.len() != 4 {
        return None;
    }

    let rd = parse_register(&tokens[1], &reg_map)?;
    println!("rd after parsing: {:?}", &rd);
    let rs1 = parse_register(&tokens[2], &reg_map)?;
    let rs2 = parse_register(&tokens[3], &reg_map)?;

    Some(Instruction::RType(RType {
        instruction,
        rd,
        rs1,
        rs2,
    }))
}

fn parse_itype(tokens: &[String], instruction: ITypeInstruction, reg_map: &HashMap<&'static str, u8>) -> Option<Instruction> {
    println!("{:?}", &tokens);
    if tokens.len() != 4 {
        return None;
    }

    let rd = parse_register(&tokens[1], &reg_map)?;
    let rs1 =parse_register(&tokens[2], &reg_map)?; 
    let imm = tokens[3].parse::<i16>().ok()?;

    Some(Instruction::IType(IType {
        instruction,
        rd,
        rs1,
        imm,
    }))
}

fn parse_stype(tokens: &[String], instruction: STypeInstruction) -> Option<Instruction> {
    if tokens.len() != 3 {
        return None;
    }

    let rs2 = tokens[1][1..].parse().ok()?;

    let imm_and_rs1: Vec<&str> = tokens[2].split(|c| c == '(' || c == ')').filter(|x| !x.is_empty()).collect();
    if imm_and_rs1.len() != 2 {
        return None
    }

    let imm = imm_and_rs1[0].parse().ok()?;
    let rs1 = imm_and_rs1[1][1..].parse().ok()?;

    Some(Instruction::SType(SType {
        instruction,
        rs1,
        rs2,
        imm,
    }))
}

fn parse_btype(tokens: &[String], instruction: BTypeInstruction, reg_map: &HashMap<&'static str, u8>) -> Option<Instruction> {
    if tokens.len() != 4 {
        return None;
    }

    let rs1 = parse_register(&tokens[1], &reg_map)?;
    let rs2 = parse_register(&tokens[2], &reg_map)?;
    let imm: i16 = tokens[3].parse().ok()?;

    Some(Instruction::BType(BType {
        instruction,
        rs1,
        rs2,
        imm,
    }))
}

fn parse_utype(tokens: &[String], instruction: UTypeInstruction, reg_map: &HashMap<&'static str, u8>) -> Option<Instruction> {
    if tokens.len() != 3 {
        return None;
    }

    let rd = parse_register(&tokens[1], &reg_map)?;
    let imm: i32 = tokens[2].parse().ok()?;

    Some(Instruction::UType(UType {
        instruction,
        rd,
        imm,
    }))
}

fn parse_jtype(tokens: &[String],  reg_map: &HashMap<&'static str, u8>) -> Option<Instruction> {
    if tokens.len() != 3 {
        return None;
    }

    let rd = parse_register(&tokens[1], &reg_map)?;
    let imm = tokens[2].parse().ok()?;
    Some(Instruction::JType(JType {
        _instruction: JTypeInstruction::Jal,
        rd,
        imm,
    }))
}

fn main() -> Result<(), String> {
    let source_code = "sw x10 0(x2)";
    let machine_code = assemble(source_code.to_string());
    println!("{:?}", &machine_code);

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

    // R-Type Tests
    #[test]
    fn test_add_encoding() {
        let add = Instruction::RType(RType {
            instruction: RTypeInstruction::Add,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(add.encode(), 0x003100B3);
    }

    #[test]
    fn test_sub_encoding() {
        let sub = Instruction::RType(RType {
            instruction: RTypeInstruction::Sub,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(sub.encode(), 0x403100B3);
    }

    #[test]
    fn test_sll_encoding() {
        let sll = Instruction::RType(RType {
            instruction: RTypeInstruction::Sll,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(sll.encode(), 0x003111B3);
    }

    #[test]
    fn test_slt_encoding() {
        let slt = Instruction::RType(RType {
            instruction: RTypeInstruction::Slt,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(slt.encode(), 0x003121B3);
    }

    #[test]
    fn test_sltu_encoding() {
        let sltu = Instruction::RType(RType {
            instruction: RTypeInstruction::Sltu,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(sltu.encode(), 0x003131B3);
    }

    #[test]
    fn test_xor_encoding() {
        let xor = Instruction::RType(RType {
            instruction: RTypeInstruction::Xor,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(xor.encode(), 0x003141B3);
    }

    #[test]
    fn test_srl_encoding() {
        let srl = Instruction::RType(RType {
            instruction: RTypeInstruction::Srl,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(srl.encode(), 0x003151B3);
    }

    #[test]
    fn test_sra_encoding() {
        let sra = Instruction::RType(RType {
            instruction: RTypeInstruction::Sra,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(sra.encode(), 0x403151B3);
    }

    #[test]
    fn test_or_encoding() {
        let or = Instruction::RType(RType {
            instruction: RTypeInstruction::Or,
            rd: 1,
            rs1: 2,
            rs2: 3,
        });
        assert_eq!(or.encode(), 0x003161B3);
    }

    //#[test]
    //fn test_and_encoding() {
    //    let and = Instruction::RType(RType {
    //        instruction: RTypeInstruction::And,
    //        rd: 1,
    //        rs1: 2,
    //        rs2: 3,
    //    });
    //    assert_eq!(and.encode(), 0x003171B3);
    //}

    // I-Type Tests
    #[test]
    fn test_addi_encoding() {
        let addi = Instruction::IType(IType {
            instruction: ITypeInstruction::Addi,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(addi.encode(), 0x00410113);
    }

    #[test]
    fn test_slti_encoding() {
        let slti = Instruction::IType(IType {
            instruction: ITypeInstruction::Slti,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(slti.encode(), 0x00412113);
    }

    #[test]
    fn test_sltiu_encoding() {
        let sltiu = Instruction::IType(IType {
            instruction: ITypeInstruction::Sltiu,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(sltiu.encode(), 0x00413113);
    }

    #[test]
    fn test_xori_encoding() {
        let xori = Instruction::IType(IType {
            instruction: ITypeInstruction::Xori,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(xori.encode(), 0x00414113);
    }

    #[test]
    fn test_ori_encoding() {
        let ori = Instruction::IType(IType {
            instruction: ITypeInstruction::Ori,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(ori.encode(), 0x00416113);
    }

    #[test]
    fn test_andi_encoding() {
        let andi = Instruction::IType(IType {
            instruction: ITypeInstruction::Andi,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(andi.encode(), 0x00417113);
    }

    #[test]
    fn test_lb_encoding() {
        let lb = Instruction::IType(IType {
            instruction: ITypeInstruction::Lb,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(lb.encode(), 0x00410103);
    }

    #[test]
    fn test_lh_encoding() {
        let lh = Instruction::IType(IType {
            instruction: ITypeInstruction::Lh,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(lh.encode(), 0x00411103);
    }

    #[test]
    fn test_lw_encoding() {
        let lw = Instruction::IType(IType {
            instruction: ITypeInstruction::Lw,
            rd: 3,
            rs1: 4,
            imm: 8,
        });
        assert_eq!(lw.encode(), 0x00822183);
    }

    #[test]
    fn test_lbu_encoding() {
        let lbu = Instruction::IType(IType {
            instruction: ITypeInstruction::Lbu,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(lbu.encode(), 0x00414103);
    }

    #[test]
    fn test_lhu_encoding() {
        let lhu = Instruction::IType(IType {
            instruction: ITypeInstruction::Lhu,
            rd: 2,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(lhu.encode(), 0x00415103);
    }

    #[test]
    fn test_jalr_encoding() {
        let jalr = Instruction::IType(IType {
            instruction: ITypeInstruction::Jalr,
            rd: 1,
            rs1: 2,
            imm: 4,
        });
        assert_eq!(jalr.encode(), 0x00410067);
    }

    // S-Type Tests
    #[test]
    fn test_sb_encoding() {
        let sb = Instruction::SType(SType {
            instruction: STypeInstruction::Sb,
            rs1: 2,
            rs2: 10,
            imm: 0,
        });
        assert_eq!(sb.encode(), 0x00A12023);
    }

    #[test]
    fn test_sh_encoding() {
        let sh = Instruction::SType(SType {
            instruction: STypeInstruction::Sh,
            rs1: 2,
            rs2: 10,
            imm: 0,
        });
        assert_eq!(sh.encode(), 0x00A11023);
    }

    #[test]
    fn test_sw_encoding() {
        let sw = Instruction::SType(SType {
            instruction: STypeInstruction::Sw,
            rs1: 2,
            rs2: 10,
            imm: 0,
        });
        assert_eq!(sw.encode(), 0x00A12023);
    }

    // B-Type Tests
    #[test]
    fn test_beq_encoding() {
        let beq = Instruction::BType(BType {
            instruction: BTypeInstruction::Beq,
            rs1: 1,
            rs2: 2,
            imm: 16,
        });
        assert_eq!(beq.encode(), 0x00208663);
    }

    #[test]
    fn test_bne_encoding() {
        let bne = Instruction::BType(BType {
            instruction: BTypeInstruction::Bne,
            rs1: 1,
            rs2: 2,
            imm: 16,
        });
        assert_eq!(bne.encode(), 0x00209663);
    }

    #[test]
    fn test_blt_encoding() {
        let blt = Instruction::BType(BType {
            instruction: BTypeInstruction::Blt,
            rs1: 1,
            rs2: 2,
            imm: 16,
        });
        assert_eq!(blt.encode(), 0x0020C663);
    }

    #[test]
    fn test_bge_encoding() {
        let bge = Instruction::BType(BType {
            instruction: BTypeInstruction::Bge,
            rs1: 1,
            rs2: 2,
            imm: 16,
        });
        assert_eq!(bge.encode(), 0x0020D663);
    }

    #[test]
    fn test_bltu_encoding() {
        let bltu = Instruction::BType(BType {
            instruction: BTypeInstruction::Bltu,
            rs1: 1,
            rs2: 2,
            imm: 16,
        });
        assert_eq!(bltu.encode(), 0x0020E663);
    }

    #[test]
    fn test_bgeu_encoding() {
        let bgeu = Instruction::BType(BType {
            instruction: BTypeInstruction::Bgeu,
            rs1: 1,
            rs2: 2,
            imm: 16,
        });
        assert_eq!(bgeu.encode(), 0x0020F663);
    }

    // U-Type Tests
    #[test]
    fn test_lui_encoding() {
        let lui = Instruction::UType(UType {
            instruction: UTypeInstruction::Lui,
            rd: 5,
            imm: 0x12345,
        });
        assert_eq!(lui.encode(), 0x123455B7);
    }

    //#[test]
    //fn test_auipc_encoding() {
    //    let auipc = Instruction::UType(UType {
    //        instruction: UTypeInstruction::Auipc,
    //        rd: 5,
    //        imm: 0x12345,
    //    });
    //    assert_eq!(auipc.encode(), 0x12345597);
    //}

    // J-Type Tests
    #[test]
    fn test_jal_encoding() {
        let jal = Instruction::JType(JType {
            _instruction: JTypeInstruction::Jal,
            rd: 1,
            imm: 0x20000,
        });
        assert_eq!(jal.encode(), 0x200000EF);
    }
}
