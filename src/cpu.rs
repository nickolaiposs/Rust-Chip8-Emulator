#![allow(non_snake_case)]

use crate::font::FONT_SET;
use rand::Rng;

pub const DISPLAY_SIZE: (usize, usize) = (64, 32);
const RAM_SIZE: usize = 4096;
const EMPTY_SCREEN: [[u8; DISPLAY_SIZE.1]; DISPLAY_SIZE.0] = [[0; DISPLAY_SIZE.1]; DISPLAY_SIZE.0];

pub struct CPU {
    ram: [u8; RAM_SIZE],
    vram: [[u8; DISPLAY_SIZE.1]; DISPLAY_SIZE.0],
    render: bool,
    v_reg: [u8; 16],
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    keys: [bool; 16],
}

impl CPU {
    pub fn new() -> Self {
        let mut ram: [u8; RAM_SIZE] = [0; RAM_SIZE];

        //assigning the font values at the beginning of the ram
        for (i, value) in FONT_SET.iter().enumerate() {
            ram[i] = *value;
        }

        Self {
            ram,
            vram: EMPTY_SCREEN,
            render: false,
            v_reg: [0; 16],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200, //this is where CHIP8 programs start
            sp: 0,
            stack: [0; 16],
            keys: [false; 16],
        }
    }

    pub fn tick(&mut self, keys: [bool; 16]) {
        self.keys = keys;
        self.render = false;

        let op = self.get_op();
        self.execute_op(op);

        //timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
     }

    pub fn get_vram(&mut self) -> &[[u8; DISPLAY_SIZE.1]; DISPLAY_SIZE.0] {
        &self.vram
    }

    pub fn new_screen(&mut self) -> bool {
        self.render
    }

    pub fn load(&mut self, game_data: &Vec<u8>) {
        //loading game data into ram starting at 0x200
        for (i, &byte) in game_data.iter().enumerate() {
            self.ram[0x200 + i] = byte;
        }
    }

    fn get_op(&mut self) -> u16 {
        let op = ((self.ram[self.pc as usize] as u16) << 8) | (self.ram[(self.pc + 1) as usize] as u16);
        self.next();

        op
    }

    fn execute_op(&mut self, op: u16) {
        let op_code = (
            ((op & 0xf000) >> 12) as u8,
            ((op & 0x0f00) >> 8) as u8,
            ((op & 0x00f0) >> 4) as u8,
            (op & 0x000f) as u8,
        );

        let nnn = (op & 0xfff) as u16;
        let n = op_code.3;
        let x = op_code.1;
        let y = op_code.2;
        let kk = (op & 0xff) as u8;

        match op_code {
            (0x0, 0x0, 0xe, 0x0) => self.op_00E0(),
            (0x0, 0x0, 0xe, 0xe) => self.op_00EE(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x, y),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xe) => self.op_8xyE(x, y),
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),
            (0xa, _, _, _) => self.op_Annn(nnn),
            (0xb, _, _, _) => self.op_Bnnn(nnn),
            (0xc, _, _, _) => self.op_Cxkk(x, kk),
            (0xd, _, _, _) => self.op_Dxyn(x, y, n),
            (0xe, _, 0x9, 0xe) => self.op_Ex9E(x),
            (0xe, _, 0xa, 0x1) => self.op_ExA1(x),
            (0xf, _, 0x0, 0x7) => self.op_Fx07(x),
            (0xf, _, 0x0, 0xa) => self.op_Fx0A(x),
            (0xf, _, 0x1, 0x5) => self.op_Fx15(x),
            (0xf, _, 0x1, 0x8) => self.op_Fx18(x),
            (0xf, _, 0x1, 0xe) => self.op_Fx1E(x),
            (0xf, _, 0x2, 0x9) => self.op_Fx29(x),
            (0xf, _, 0x3, 0x3) => self.op_Fx33(x),
            (0xf, _, 0x5, 0x5) => self.op_Fx55(x),
            (0xf, _, 0x6, 0x5) => self.op_Fx65(x),
            _ => (),
        }
    }

    fn next(&mut self) {
        self.pc += 2;
    }

    fn wait(&mut self) {
        self.pc -= 2;
    }

    fn stack_pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn stack_push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn op_00E0(&mut self) {
        self.vram = EMPTY_SCREEN;
    }

    fn op_00EE(&mut self) {
        self.pc = self.stack_pop();
    }

    fn op_1nnn(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn op_2nnn(&mut self, addr: u16) {
        self.stack_push(self.pc);
        self.pc = addr;
    }

    fn op_3xkk(&mut self, x: u8, byte: u8) {
        if self.v_reg[x as usize] == byte {
            self.next();
        }
    }

    fn op_4xkk(&mut self, x: u8, byte: u8) {
        if self.v_reg[x as usize] != byte {
            self.next();
        }
    }

    fn op_5xy0(&mut self, x: u8, y: u8) {
        if self.v_reg[x as usize] == self.v_reg[y as usize] {
            self.next();
        }
    }

    fn op_6xkk(&mut self, x: u8, byte: u8) {
        self.v_reg[x as usize] = byte;
    }

    fn op_7xkk(&mut self, x: u8, byte: u8) {
        self.v_reg[x as usize] = self.v_reg[x as usize].wrapping_add(byte);
    }

    fn op_8xy0(&mut self, x: u8, y: u8) {
        self.v_reg[x as usize] = self.v_reg[y as usize];
    }

    fn op_8xy1(&mut self, x: u8, y: u8) {
        self.v_reg[x as usize] = self.v_reg[x as usize] | self.v_reg[y as usize];
    }

    fn op_8xy2(&mut self, x: u8, y: u8) {
        self.v_reg[x as usize] = self.v_reg[x as usize] & self.v_reg[y as usize];
    }

    fn op_8xy3(&mut self, x: u8, y: u8) {
        self.v_reg[x as usize] = self.v_reg[x as usize] ^ self.v_reg[y as usize];
    }

    fn op_8xy4(&mut self, x: u8, y: u8) {
        let result = (self.v_reg[x as usize] as u16) + (self.v_reg[y as usize] as u16);
        self.v_reg[x as usize] = result as u8;

        let carry = if result > 255 { 1 } else { 0 };
        self.v_reg[0xf] = carry;
    }

    fn op_8xy5(&mut self, x: u8, y: u8) {
        self.v_reg[0xf] = if self.v_reg[x as usize] > self.v_reg[y as usize] {
            1
        } else {
            0
        };

        self.v_reg[x as usize] = self.v_reg[x as usize].wrapping_sub(self.v_reg[y as usize]);
    }

    fn op_8xy6(&mut self, x: u8, _y: u8) {
        self.v_reg[0xf] = self.v_reg[x as usize] & 1;
        self.v_reg[x as usize] >>= 1;
    }

    fn op_8xy7(&mut self, x: u8, y: u8) {
        self.v_reg[0xf] = if self.v_reg[y as usize] > self.v_reg[x as usize] {
            1
        } else {
            0
        };
        self.v_reg[x as usize] = self.v_reg[y as usize].wrapping_sub(self.v_reg[x as usize]);
    }

    fn op_8xyE(&mut self, x: u8, _y: u8) {
        self.v_reg[0xf] = (self.v_reg[x as usize] >> 7) & 1;
        self.v_reg[x as usize] <<= 1;
    }

    fn op_9xy0(&mut self, x: u8, y: u8) {
        if self.v_reg[x as usize] != self.v_reg[y as usize] {
            self.next();
        }
    }

    fn op_Annn(&mut self, addr: u16) {
        self.i_reg = addr;
    }

    fn op_Bnnn(&mut self, addr: u16) {
        self.pc = (self.v_reg[0] as u16) + addr;
    }

    fn op_Cxkk(&mut self, x: u8, byte: u8) {
        let rand_byte: u8 = rand::thread_rng().gen();

        self.v_reg[x as usize] = rand_byte & byte;
    }

    fn op_Dxyn(&mut self, x: u8, y: u8, n: u8) {
        let mut erased: u8 = 0;

        for byte in 0..n {
            let mem_addr = (self.i_reg + (byte as u16)) as usize;
            let y_coord = ((self.v_reg[y as usize] + byte) as usize) % DISPLAY_SIZE.1;

            for bit in 0..8 {
                let x_coord = ((self.v_reg[x as usize] as usize + bit as usize)) % DISPLAY_SIZE.0;
                let pixel = (self.ram[mem_addr] >> (7 - bit)) & 1;

                erased |= pixel & self.vram[x_coord][y_coord];
                self.vram[x_coord][y_coord] ^= pixel;
            }
        }

        self.v_reg[0xf] = erased;
        self.render = true;
    }

    fn op_Ex9E(&mut self, x: u8) {
        if self.keys[self.v_reg[x as usize] as usize] {
            self.next();
        }
    }

    fn op_ExA1(&mut self, x: u8) {
        if !self.keys[self.v_reg[x as usize] as usize] {
            self.next();
        }
    }

    fn op_Fx07(&mut self, x: u8) {
        self.v_reg[x as usize] = self.delay_timer;
    }

    fn op_Fx0A(&mut self, x: u8) {
        let mut key_pressed: bool = false;
        
        for i in 0..self.keys.len() {
            if self.keys[i] {
                self.v_reg[x as usize] = i as u8;
                key_pressed = true;
            }
        }

        if !key_pressed {
            self.wait();
        }
    }

    fn op_Fx15(&mut self, x: u8) {
        self.delay_timer = self.v_reg[x as usize];
    }

    fn op_Fx18(&mut self, x: u8) {
        self.sound_timer = self.v_reg[x as usize];
    }

    fn op_Fx1E(&mut self, x: u8) {
        self.i_reg = self.i_reg.wrapping_add(self.v_reg[x as usize] as u16);
    }

    fn op_Fx29(&mut self, x: u8) {
        self.i_reg = (self.v_reg[x as usize] as u16) * 5;
    }

    fn op_Fx33(&mut self, x: u8) {
        self.ram[self.i_reg as usize] = self.v_reg[x as usize] / 100;
        self.ram[(self.i_reg as usize) + 1] = (self.v_reg[x as usize] / 10) % 10;
        self.ram[(self.i_reg as usize) + 2] = self.v_reg[x as usize] % 10;
    }

    fn op_Fx55(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.ram[(self.i_reg as usize) + i] = self.v_reg[i];
        }
    }

    fn op_Fx65(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.v_reg[i] = self.ram[(self.i_reg as usize) + i];
        }
    }
}