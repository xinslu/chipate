use crate::font::FONT_SET;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;
pub struct Processor {
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    vram_changed: bool,
    ram: [u8; CHIP8_RAM],
    stack: [usize; 16],
    registers: [u8; 16],
    i: usize,
    pc: usize,
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    keypad_waiting: bool,
    keypad_register: usize,
}

impl Processor {
    pub fn new() -> Self {
        let mut ram = [0u8; CHIP8_RAM];
        for i in 0..FONT_SET.len() {
            ram[i] = FONT_SET[i];
        }

        Processor {
            vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            vram_changed: false,
            ram,
            stack: [0; 16],
            registers: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            keypad_waiting: false,
            keypad_register: 0,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.ram[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }
}
