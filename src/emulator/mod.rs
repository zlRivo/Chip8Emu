use crate::disassembler::disassemble;
use std::cmp::{min, max};

pub struct Chip8 {
    memory: [u8; 0xFFF],
    freq: u32, // Number of instructions ran per second
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    vars: [u8; 0xF0],
    display: [[bool; 64]; 32],
    delay_timer: u8,
    sound_timer: u8
}

impl Chip8 {
    /// Returns a new instance
    pub fn new() -> Self {
        Self {
            memory: [0u8; 0xFFF],
            freq: 700,
            pc: 0x200,
            i: 0x0050,
            stack: Vec::new(),
            vars: [0u8; 0xF0],
            display: [[false; 64]; 32],
            delay_timer: 60,
            sound_timer: 60
        }
    }

    /// Loads the program (as a byte vector) into the emulator memory
    pub fn load_program(mut self, prog: Vec<u8>) -> Self {
        // Limit bytes
        let prog: Vec<u8> = prog.into_iter().take(0xFFF - 0x200).collect();
        // Write to memory
        for (i, b) in prog.iter().enumerate() {
            self.memory[0x200 + i] = *b;
        }
        self
    }

    /// Sets the font for the emulator within 0x000-0x200
    pub fn load_font(mut self, font: Vec<u8>, addr: u16) -> Self {
        // Limit bytes
        let font: Vec<u8> = font.into_iter().take(0x200 - addr as usize).collect();
        // Write to memory
        for (i, b) in font.iter().enumerate() {
            self.memory[addr as usize + i] = *b;
        }
        self
    }

    /// Set the frequency of the processor (Hz)
    pub fn set_freq(mut self, freq: u32) -> Self {
        self.freq = freq;
        self
    }

    /// Returns the frequency of the processor (Hz)
    pub fn get_freq(&self) -> u32 {
        self.freq
    }

    /// Returns byte pointed at the I register
    pub fn read_at_i(&self, offset: u8) -> Option<u8> {
        self.memory.get(self.i as usize + offset as usize).copied()
    }

    // Sets the value of the I register
    fn set_i(&mut self, val: u16) {
        self.i = val & 0xFFF;
    }

    /// Fetch the two succeeding bytes at pc
    pub fn fetch(&mut self) -> u16 {
        match (self.memory.get(self.pc as usize), self.memory.get((self.pc + 1) as usize)) {
            (Some(&b1), Some(&b2)) => {
                self.pc += 2; // Increment pc
                ((b1 as u16) << 8) | b2 as u16 // Return the two fetched bytes
            },
            (_, _) => panic!("Couldn't fetch two bytes at program counter !")
        }
    }

    // Push value onto stack and checks for overflow
    fn push_stack(&mut self, val: u16) {
        if self.stack.len() >= 16 { panic!("Stack overflow !"); }
        self.stack.push(val & 0xFFF);
    }
    
    // Pops last stack value
    fn pop_stack(&mut self) -> u16 {
        match self.stack.pop() {
            Some(v) => v,
            None => { panic!("No value in stack !") }
        }
    }

    /// Decodes a u16 instruction into hex
    pub fn decode_to_nibbles(instr: u16) -> (u8, u8, u8, u8) {
        let i = instr;
        (((i >> 12) & 0xF) as u8, ((i >> 8) & 0xF) as u8, ((i >> 4) & 0xF) as u8, (i & 0xF) as u8)
    }

    /// Executes a u16 instruction
    pub fn exec(&mut self, instr: u16) -> Result<(), ()> {
        let i = instr;
        let nibbles = Self::decode_to_nibbles(i);
        let (b2, imm_address) = ((i & 0xFF) as u8, (i & 0xFFF) as u16);
        
        // For debugging
        println!("{}", disassemble(instr));

        match nibbles {
            // (0x0, 0x0, 0xC, _) => format!("SCDOWN {:01X}", i & 0xF),
            (0x0, 0x0, 0xE, 0x0) => { // Clear screen
                self.clear_screen();
                Ok(())
            },
            (0x0, 0x0, 0xE, 0xE) => { // RTS (UNTESTED)
                self.pc = self.pop_stack();
                Ok(())
            },
            // (0x0, 0x0, 0xF, 0xB) => format!("SCRIGHT"),
            // (0x0, 0x0, 0xF, 0xC) => format!("SCLEFT"),
            // (0x0, 0x0, 0xF, 0xE) => format!("LOW"),
            // (0x0, 0x0, 0xF, 0xF) => format!("HIGH"),
            (0x1, _, _, _) => { // JMP
                self.jump_to(imm_address);
                Ok(())
            },
            (0x2, _, _, _) => { // JSR NNN (UNTESTED)
                self.push_stack(self.pc); // Push pc into stack
                self.jump_to(imm_address); // Jump to address
                Ok(())
            },
            (0x3, _, _, _) => { // SKEQ VX, NN (UNTESTED)
                match self.get_reg(nibbles.1) {
                    Some(vx) => {
                        if vx == b2 { self.pc += 2; } // Skip next instruction
                        Ok(())
                    },
                    None => Err(())
                }
            },
            (0x4, _, _, _) => { // SKNE VX, NN (UNTESTED)
                match self.get_reg(nibbles.1) {
                    Some(vx) => {
                        if vx != b2 { self.pc += 2; } // Skip next instruction
                        Ok(())
                    },
                    None => Err(())
                }
            },
            (0x5, _, _, 0x0) => { // SKEQ VX, VY (UNTESTED)
                if let (Some(vx), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    if vx == vy { self.pc += 2; } // Skip next instruction
                    Ok(())
                } else { return Err(()) }
            },
            (0x6, _, _, _) => { // MOV VX, NN
                self.set_reg(nibbles.1 & 0xF, b2)
            },
            (0x7, _, _, _) => { // ADD VX, NN
                // Get register value
                match self.get_reg(nibbles.1) {
                    Some(v) => {
                        return self.set_reg(nibbles.1 & 0xF, b2 + v) // No overflow check
                    },
                    None => return Err(())
                }
            },
            (0x8, _, _, 0x0) => { // MOV VX, VY (UNTESTED)
                if let (Some(_), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    self.set_reg(nibbles.1, vy)?;
                    Ok(())
                } else { return Err(()) }
            },
            (0x8, _, _, 0x1) => { // OR VX, VY (UNTESTED)
                if let (Some(vx), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    self.set_reg(nibbles.1, vx | vy)?;
                    Ok(())
                } else { return Err(()) }
            },
            (0x8, _, _, 0x2) => { // AND VX, VY (UNTESTED)
                if let (Some(vx), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    self.set_reg(nibbles.1, vx & vy)?;
                    Ok(())
                } else { return Err(()) }
            },
            (0x8, _, _, 0x3) => { // XOR VX, VY (UNTESTED)
                if let (Some(vx), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    self.set_reg(nibbles.1, vx ^ vy)?;
                    Ok(())
                } else { return Err(()) }
            },
            (0x8, _, _, 0x4) => { // ADD VX, VY (UNTESTED)
                if let (Some(vx), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    let val = (vx + vy) as u16; // u16 to prevent overflow
                    self.set_flag({
                        if val > 0xFF { 1 } // Set VF
                        else { 0 }
                    });
                    self.set_reg(nibbles.1, (val & 0xFF) as u8)?;
                    Ok(())
                } else { return Err(()) }
            },
            (0x8, _, _, 0x5) => { // SUB VX, VY (UNTESTED)
                if let (Some(vx), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    let diff = max(vx, vy) - min(vx, vy); // For knowing how much to subtract
                    let val = if vx < vy { // Check underflow
                        self.set_flag(0);
                        0xFF - diff
                    } else { self.set_flag(0); vx - diff };
                    self.set_reg(nibbles.1, val)?;
                    Ok(())
                } else { return Err(()) }
            },
            // (0x8, _, 0x0, 0x6) => format!("SHR V{:01X}", nibbles.1),
            // (0x8, _, _, 0x7) => format!("RSB V{:01X}, V{:01X}", nibbles.1, nibbles.2),
            // (0x8, _, 0x0, 0xE) => format!("SHL V{:01X}", nibbles.1),
            (0x9, _, _, 0x0) => { // SKNE VX, VY (UNTESTED)
                if let (Some(vx), Some(vy)) = (self.get_reg(nibbles.1), self.get_reg(nibbles.2)) {
                    if vx != vy { self.pc += 2; } // Skip next instruction
                    Ok(())
                } else { return Err(()) }
            },
            (0xA, _, _, _) => { // MVI I NNN (Sets i register)
                self.set_i(imm_address);
                Ok(())
            },
            // (0xB, _, _, _) => format!("JMI {:03X}", imm_address),
            // (0xC, _, _, _) => format!("RAND V{:01X}, {:02X}", nibbles.1, b2),
            // (0xD, _, _, 0x0) => format!("XSPRITE R{:01X}, R{:01X}", nibbles.1, nibbles.2),
            (0xD, _, _, _) => { // SRPITE VX, VY, N
                self.display(nibbles.1, nibbles.2, nibbles.3)
            },
            // (0xE, _, 0x9, 0xE) => format!("SKPR K{:01X}", nibbles.1),
            // (0xE, _, 0xA, 0x1) => format!("SKUP K{:01X}", nibbles.1),
            // (0xF, _, 0x0, 0x7) => format!("GDELAY V{:01X}", nibbles.1),
            // (0xF, _, 0x0, 0xA) => format!("KEY V{:01X}", nibbles.1),
            // (0xF, _, 0x1, 0x5) => format!("SDELAY V{:01X}", nibbles.1),
            // (0xF, _, 0x1, 0x8) => format!("SSOUND V{:01X}", nibbles.1),
            // (0xF, _, 0x1, 0xE) => format!("ADI V{:01X}", nibbles.1),
            // (0xF, _, 0x2, 0x9) => format!("FONT V{:01X}", nibbles.1),
            // (0xF, _, 0x3, 0x0) => format!("XFONT V{:01X}", nibbles.1),
            // (0xF, _, 0x3, 0x3) => format!("BCD V{:01X}", nibbles.1),
            // (0xF, _, 0x5, 0x5) => format!("STR V0-V{:01X}", nibbles.1),
            // (0xF, _, 0x6, 0x5) => format!("LDR V0-V{:01X}", nibbles.1),
            _ => return Err(())
        }
    }

    // Clears display by setting all display values to false
    fn clear_screen(&mut self) {
        self.display.map(|_| [false; 64]);
    }

    /// Returns the display values
    pub fn get_display(&self) -> [[bool; 64]; 32] {
        self.display
    }

    // Jump to address
    fn jump_to(&mut self, addr: u16) {
        self.pc = addr;
    }

    // Sets a register value
    fn set_reg(&mut self, reg: u8, val: u8) -> Result<(), ()> {
        match self.vars.get(reg as usize) {
            Some(_) => {
                self.vars[reg as usize] = val;
                Ok(())
            },
            None => Err(())
        }
    }

    /// Gets a register value
    pub fn get_reg(&self, reg: u8) -> Option<u8> {
        match self.vars.get(reg as usize) {
            Some(_) => {
                Some(self.vars[reg as usize])
            },
            None => None
        }
    }

    // Set flag register (VF)
    fn set_flag(&mut self, flag: u8) {
        self.set_reg(0xF, flag).unwrap(); // I know 0xF is a valid register so I use unwrap
    }

    // Draws onto the display
    fn display(&mut self, reg_x: u8, reg_y: u8, n: u8) -> Result<(), ()> {
        if let (Some(mut x), Some(mut y)) = (self.get_reg(reg_x), self.get_reg(reg_y)) {
            // To store I byte
            let mut sprite: u8;
            // Get horizontal and vertical position using modulo
            x = x % 64;
            y = y % 32;
            let mut row_x = x; // For resetting the x value

            let mut bit: u8; // For knowing which bit to read in the sprite
            let mut offset = 0; // I offset

            // Set flag to 0 by default
            self.set_flag(0);

            // Limit n
            let n = n & 0xF;

            // Write to screen
            for _ in 0..n {
                bit = 0b1 << 7;
                
                // Read byte located at I + Offset
                sprite = {
                    match self.read_at_i(offset) {
                        Some(v) => v,
                        None => panic!("Register I out of bounds !")
                    }
                };

                // Loop through each bit of the sprite
                for _ in 0..8 {
                    match self.display[y as usize].get(row_x as usize) {
                        Some(_) => {
                            if bit & sprite > 0x0 { // If the bit is set
                                let pixel: &mut bool = &mut self.display[y as usize][row_x as usize]; // Get reference to pixel on the display
                                *pixel = !*pixel; // Flip the pixel on display
                                if *pixel == false { self.set_flag(1); } // Set the VF Flag if we turned off the pixel
                            }
                        }, 
                        None => { // When out of bounds
                            break; // Break out of for loop (go to next row)
                        }
                    }
                    row_x += 1; // Increment X
                    bit = bit >> 1; // Shift the bit
                }
                
                offset += 1; // Increment offset
                row_x = x; // Reset X
                y += 1;
                if y >= 64 { break; } // Break loop if y is outside display
            }
            return Ok(());
        } else { return Err(()); }
    }
}
