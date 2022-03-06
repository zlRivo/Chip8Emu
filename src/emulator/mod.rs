pub struct Chip8 {
    memory: [u8; 0xFFF],
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    vars: [u8; 0xF0],
    display: [[bool; 64]; 32],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    /// Returns a new instance
    fn new() -> Self {
        Self {
            memory: [0u8; 0xFFF],
            pc: 0x200,
            i: 0x0000,
            stack: Vec::new(),
            vars: [0u8; 0xF0],
            display: [[false; 64]; 32],
            delay_timer: 60,
            sound_timer: 60
        }
    }

    /// Loads the program (as a byte vector) into the emulator memory
    fn load_program(mut self, prog: Vec<u8>) -> Self {
        // Limit bytes
        let prog: Vec<u8> = prog.into_iter().take(0xFFF - 0x200).collect();
        // Write to memory
        for (i, b) in prog.iter().enumerate() {
            self.memory[0x200 + i] = *b;
        }
        self
    }

    /// Sets the font for the emulator
    fn load_font(mut self, font: Vec<u8>) -> Self {
        // Limit bytes
        let font: Vec<u8> = font.into_iter().take(0x9F - 0x50).collect();
        // Write to memory
        for (i, b) in font.iter().enumerate() {
            self.memory[0x50 + i] = *b;
        }
        self
    }
}
