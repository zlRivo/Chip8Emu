/// Disassemble an instruction
pub fn disassemble(instr: u16) -> String {
    let i = instr;
    let nibbles = ((i >> 12) & 0xF, (i >> 8) & 0xF, (i >> 4) & 0xF, i & 0xF);
    let (b1, b2, imm_address) = ((i >> 8) & 0xFF, i & 0xFF, i & 0xFFF);
    
    match nibbles {
        (0x0, 0x0, 0xC, _) => format!("SCDOWN {:01X}", i & 0xF),
        (0x0, 0x0, 0xE, 0x0) => format!("CLS"),
        (0x0, 0x0, 0xE, 0xE) => format!("RTS"),
        (0x0, 0x0, 0xF, 0xB) => format!("SCRIGHT"),
        (0x0, 0x0, 0xF, 0xC) => format!("SCLEFT"),
        (0x0, 0x0, 0xF, 0xE) => format!("LOW"),
        (0x0, 0x0, 0xF, 0xF) => format!("HIGH"),
        (0x1, _, _, _) => format!("JMP {:03X}", imm_address),
        (0x2, _, _, _) => format!("JSR {:03X}", imm_address),
        (0x3, _, _, _) => format!("SKEQ V{:01X}, {:02X}", nibbles.1, b2),
        (0x4, _, _, _) => format!("SKNE V{:01X}, {:02X}", nibbles.1, b2),
        (0x5, _, _, 0x0) => format!("SKEQ V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x6, _, _, _) => format!("MOV V{:01X}, {:02X}", nibbles.1, b2),
        (0x7, _, _, _) => format!("ADD V{:01X}, {:02X}", nibbles.1, b2),
        (0x8, _, _, 0x0) => format!("MOV V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x8, _, _, 0x1) => format!("OR V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x8, _, _, 0x2) => format!("AND V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x8, _, _, 0x3) => format!("XOR V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x8, _, _, 0x4) => format!("ADD V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x8, _, _, 0x5) => format!("SUB V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x8, _, 0x0, 0x6) => format!("SHR V{:01X}", nibbles.1),
        (0x8, _, _, 0x7) => format!("RSB V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0x8, _, 0x0, 0xE) => format!("SHL V{:01X}", nibbles.1),
        (0x9, _, _, 0x0) => format!("SKNE V{:01X}, V{:01X}", nibbles.1, nibbles.2),
        (0xA, _, _, _) => format!("MVI {:03X}", imm_address),
        (0xB, _, _, _) => format!("JMI {:03X}", imm_address),
        (0xC, _, _, _) => format!("RAND V{:01X}, {:02X}", nibbles.1, b2),
        (0xD, _, _, 0x0) => format!("XSPRITE R{:01X}, R{:01X}", nibbles.1, nibbles.2), // Pattern order is important
        (0xD, _, _, _) => format!("SPRITE V{:01X}, V{:01X}, {:01X}", nibbles.1, nibbles.2, nibbles.3), // Pattern order is important
        (0xE, _, 0x9, 0xE) => format!("SKPR K{:01X}", nibbles.1),
        (0xE, _, 0xA, 0x1) => format!("SKUP K{:01X}", nibbles.1),
        (0xF, _, 0x0, 0x7) => format!("GDELAY V{:01X}", nibbles.1),
        (0xF, _, 0x0, 0xA) => format!("KEY V{:01X}", nibbles.1),
        (0xF, _, 0x1, 0x5) => format!("SDELAY V{:01X}", nibbles.1),
        (0xF, _, 0x1, 0x8) => format!("SSOUND V{:01X}", nibbles.1),
        (0xF, _, 0x1, 0xE) => format!("ADI V{:01X}", nibbles.1),
        (0xF, _, 0x2, 0x9) => format!("FONT V{:01X}", nibbles.1),
        (0xF, _, 0x3, 0x0) => format!("XFONT V{:01X}", nibbles.1),
        (0xF, _, 0x3, 0x3) => format!("BCD V{:01X}", nibbles.1),
        (0xF, _, 0x5, 0x5) => format!("STR V0-V{:01X}", nibbles.1),
        (0xF, _, 0x6, 0x5) => format!("LDR V0-V{:01X}", nibbles.1),
        _ => format!("Uninplemented instruction")
    }
}

/// Disassemble a vector of instructions
pub fn disassemble_all(instr_vec: &Vec<u16>) -> String {
    let mut output = Vec::<String>::new();
    let mut i = 0;
    for b in instr_vec {
        output.push(format!("0x{:04X}\t0x{:04X} -> {} ", i, b, disassemble(*b)));
        i += 2;
    }
    output.join("\n")
}

/// Pair vector of bytes
pub fn pair_bytes(vec: &Vec<u8>) -> Vec<u16> {
    let mut new_bytes = Vec::<u16>::new();
    let mut index = 0;
    while index <= vec.len() {
        // Check if there is a pair
        if let (Some(&b1), Some(&b2)) = (vec.get(index), vec.get(index + 1)) {
            // Convert pair to u16
            new_bytes.push(((b1 as u16) << 8) as u16 | b2 as u16);
        } else if let Some(b1) = vec.get(index) {
            // Convert byte to u16
            new_bytes.push(((*b1 as u16) << 8) as u16);
        }
        index += 2;
    }
    new_bytes
}