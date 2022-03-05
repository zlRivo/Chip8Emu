pub fn disassemble(i: u16) -> String {
    let hexes = ((i >> 8) & 0xF0, (i >> 8) & 0x0F, i & 0xF0, i & 0x0F);
    
    match hexes {
        (0x0, 0x0, 0xC, _) => format!("SCDOWN {:01X}", i & 0xF),
        (0x0, 0x0, 0xE, 0x0) => format!("CLS"),
        (0x0, 0x0, 0xE, 0xE) => format!("RTS"),
        (0x0, 0x0, 0xF, 0xB) => format!("SCRIGHT"),
        (0x0, 0x0, 0xF, 0xC) => format!("SCLEFT"),
        (0x0, 0x0, 0xF, 0xE) => format!("LOW"),
        (0x0, 0x0, 0xF, 0xF) => format!("HIGH"),
        (0x1, _, _, _) => format!("JMP {:03X}", i & 0xFFF),
        (0x2, _, _, _) => format!("JSR {:03X}", i & 0xFFF),
        (0x3, _, _, _) => format!("SKEQ V{:01X}, {:02X}", hexes.1, i & 0xFF),
        (0x4, _, _, _) => format!("SKNE V{:01X}, {:02X}", hexes.1, i & 0xFF),
        (0x5, _, _, 0x0) => format!("SKEQ V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x6, _, _, _) => format!("MOV V{:01X}, {:02X}", hexes.1, i & 0xFF),
        (0x7, _, _, _) => format!("ADD V{:01X}, {:02X}", hexes.1, i & 0xFF),
        (0x8, _, _, 0x0) => format!("MOV V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x8, _, _, 0x1) => format!("OR V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x8, _, _, 0x2) => format!("AND V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x8, _, _, 0x3) => format!("XOR V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x8, _, _, 0x4) => format!("ADD V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x8, _, _, 0x5) => format!("SUB V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x8, _, 0x0, 0x6) => format!("SHR V{:01X}", hexes.1),
        (0x8, _, _, 0x7) => format!("RSB V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0x8, _, 0x0, 0xE) => format!("SHL V{:01X}", hexes.1),
        (0x9, _, _, 0x0) => format!("SKNE V{:01X}, V{:01X}", hexes.1, hexes.2),
        (0xA, _, _, _) => format!("MVI {:03X}", i & 0xFFF),
        (0xB, _, _, _) => format!("JMI {:03X}", i & 0xFFF),
        (0xC, _, _, _) => format!("RAND V{:01X}, {:02X}", hexes.1, i & 0xFF),
        (0xD, _, _, _) => format!("SPRITE V{:01X}, V{:01X}, {:01X}", hexes.1, hexes.2, hexes.3),
        _ => format!("Uninplemented instruction")
    }
}
