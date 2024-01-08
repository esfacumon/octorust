use crate::chip8::instruction::Instruction;
use crate::chip8::low_level_operations::get_nibble;
use crate::chip8::stack::Stack;

use crate::chip8::constants::{
    MIN_ADDRESS,
    MAX_ADDRESS,
    MEMORY_SIZE,
    FONT_OFFSET,
    ROM_OFFSET,
    WIDTH,
    HEIGHT,
};

use crate::chip8::errors::{SubroutineError, RegisterError};

use std::fs::File;
use std::io::Read;


// const ROM_PATH: &str ="/Users/fas/dev/octorust/roms/3-corax+.ch8";
const ROM_PATH: &str ="/Users/fas/dev/octorust/roms/IBMLogo.ch8";

pub struct Chip8 {
    pub pixel_array: [[bool; WIDTH]; HEIGHT],
    memory: [u8; MEMORY_SIZE],
    index: u16,
    pc: u16,
    stack: Stack<u16>,
    // delay_timer: u8,
    // sound_timer: u8,
    v: [u8; 16]
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            pixel_array: [[false; WIDTH]; HEIGHT],
            memory: [0; MEMORY_SIZE],
            index: 0,
            pc: ROM_OFFSET,
            stack: Stack::new(),
            v: [0; 16],
            // delay: 0,
            // sound: 0,
        };

        chip8.load_font();
        chip8.load_rom(ROM_PATH).expect("Failed loading rom");
        chip8
    }

    
    pub fn fetch(&mut self) -> u16 {
        println!("fet::PC: {}", self.pc);
        
        let instruction = Chip8::read_memory_address(self.memory, self.pc);
        println!("fet::INST READ: 0x{:04X}", instruction);
        
        self.pc += 0x02;
        
        instruction
    }


    pub fn decode(&mut self, instruction: u16) -> Instruction {
        let first_nibble = get_nibble(instruction, 1);

        match first_nibble {
            0x0 => {
                match instruction {
                    0x00E0 =>  Instruction::ClearScreen,
                    0x00EE => Instruction::ReturnSubroutine,
                    _ => Instruction::ClearScreen,
                }
            },
            0x1 => {
                let addr: u16 = instruction % 0x1000;
                Instruction::Jump { addr }
            },
            0x2 => {
                let addr: u16 = instruction % 0x1000;
                Instruction::CallSubroutine { addr }
            },
            0x3 => {
                let register_x: usize = (instruction & 0x0100) as usize;
                let value: u8 = (instruction & 0x0011) as u8;
                Instruction::SkipIfEqual { register_x, value }
            },
            0x4 => {
                let register_x: usize = (instruction & 0x0100) as usize;
                let value: u8 = (instruction & 0x0011) as u8;
                Instruction::SkipIfNotEqual { register_x, value }
            },
            0x5 => {
                if get_nibble(instruction, 4) != 0 {
                    panic!("Invalid instruction: {}", instruction);
                }
                let register_x: usize = (instruction & 0x0100) as usize;
                let register_y: usize = (instruction & 0x0010) as usize;
                Instruction::SkipIfRegistersEqual { register_x, register_y }
            },
            0x9 => {
                if get_nibble(instruction, 4) != 0 {
                    panic!("Invalid instruction: {}", instruction);
                }
                let register_x: usize = (instruction & 0x0100) as usize;
                let register_y: usize = (instruction & 0x0010) as usize;
                Instruction::SkipIfRegistersNotEqual { register_x, register_y }
            },
            0x6 => {
                let register = get_nibble(instruction, 2) as usize;
                let value: u8 = (instruction % 0x0100) as u8;
                Instruction::Set { register, value }
            },
            0x7 => {
                let register = get_nibble(instruction, 2) as usize;
                let value: u8 = (instruction % 0x0100) as u8;
                Instruction::Add { register, value }
            },
            0xA => {
                let value: u16 = instruction % 0x1000;
                Instruction::SetI { value }
            },
            0xD => {
                let register_x = get_nibble(instruction, 2);
                let register_y = get_nibble(instruction, 3);
                let n = get_nibble(instruction, 4);
                Instruction::DisplayDraw { register_x, register_y, n }
            },
            0xE => {
                let keycode = get_nibble(instruction, 2);

                match get_nibble(instruction, 4) {

                    0xE => {
                        Instruction::SkipIfKeyPressed { keycode }
                    },
                    0x1 => {
                        Instruction::SkipIfKeyNotPressed { keycode }
                    },
                    _ => {
                        Instruction::Nop
                    },
                }
            },
            0xF => {
                let register_x = get_nibble(instruction, 2);
                Instruction::AddI { register_x: register_x.into() }
            },
            0x8 => {
                let register_x = get_nibble(instruction, 2);
                let register_y = get_nibble(instruction, 3);

                match get_nibble(instruction, 4) {
                    0x0 => {
                        Instruction::Nop
                    },
                    0x1 => {
                        Instruction::BinaryOrVX { register_x, register_y }
                    },
                    0x2 => {
                        Instruction::BinaryAndVX { register_x, register_y }
                    },
                    0x3 => {
                        Instruction::BinaryXorVX { register_x, register_y }
                    },
                    0x4 => {
                        Instruction::AddVX { register_x, register_y }
                    },
                    0x5 => {
                        Instruction::SubstractVXVY { register_x: register_x.into(), register_y: register_y.into() }
                    },
                    0x6 => {
                        Instruction::ShiftRight { register_x: register_x.into(), register_y: register_y.into() }
                    },
                    0x7 => {
                        Instruction::SubstractVYVX { register_x: register_x.into(), register_y: register_y.into() }
                    },
                    0xE => {
                        Instruction::ShiftLeft { register_x: register_x.into(), register_y: register_y.into() }
                    },
                    _ => Instruction::Nop
                }
            }
            _ => Instruction::Nop
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen => Chip8::clear_screen(&mut self.pixel_array),
            Instruction::FillScreen => Chip8::fill_screen(&mut self.pixel_array),
            Instruction::Jump { addr } => Chip8::jump(&mut self.pc, addr),
            Instruction::CallSubroutine { addr } => { 
                match Chip8::call_subroutine(&mut self.pc, &mut self.stack, addr) {
                    Ok(_) => (),
                    Err(e) => println!("Error: {:?}", e),
                }
            },
            Instruction::ReturnSubroutine => Chip8::return_subroutine(&mut self.pc, &mut self.stack),
            Instruction::Set { register, value } => Chip8::set(&mut self.v, register, value),
            Instruction::Add { register, value } => Chip8::add(&mut self.v, register, value).expect("ADD error"),
            Instruction::SetI { value } => Chip8::set_i(&mut self.index, value),
            Instruction::DisplayDraw { register_x, register_y, n } => Chip8::display(self, register_x as usize, register_y as usize, n),
            Instruction::BinaryOrVX { register_x, register_y } => Chip8::binary_or_vx(self, register_x as usize, register_y as usize),
            Instruction::BinaryAndVX { register_x, register_y } => Chip8::binary_and_vx(self, register_x as usize, register_y as usize),
            Instruction::BinaryXorVX { register_x, register_y } => Chip8::binary_xor_vx(self, register_x as usize, register_y as usize),
            Instruction::AddVX { register_x, register_y } => Chip8::add_vx(self, register_x as usize, register_y as usize),
            Instruction::Nop => println!("Nop"),
            Instruction::SkipIfEqual { register_x, value} => Chip8::skip_if_equal(self, register_x, value).expect("SkipIfEqual error"),
            Instruction::SkipIfNotEqual { register_x, value} => Chip8::skip_if_not_equal(self, register_x, value).expect("SkipIfNotEqual error"),
            Instruction::SkipIfRegistersEqual { register_x, register_y } => Chip8::skip_if_registers_equal(self, register_x, register_y).expect("SkipIfRegistersEqual error"),
            Instruction::SkipIfRegistersNotEqual { register_x, register_y } => Chip8::skip_if_registers_not_equal(self, register_x, register_y).expect("SkipIfRegistersNotEqual error"),
            Instruction::SubstractVXVY { register_x, register_y } => Chip8::substract_vx_vy(self, register_x, register_y),
            Instruction::SubstractVYVX { register_x, register_y } => Chip8::substract_vy_vx(self, register_x, register_y),
            Instruction::ShiftRight { register_x, register_y } => Chip8::shift_right(self, register_x, register_y),
            Instruction::ShiftLeft { register_x, register_y } => Chip8::shift_left(self, register_x, register_y),
            Instruction::AddI { register_x } => Chip8::add_i(self, register_x),
            Instruction::SkipIfKeyPressed { keycode } => Chip8::skip_if_key_pressed(self, keycode),
            Instruction::SkipIfKeyNotPressed { keycode } => Chip8::skip_if_key_not_pressed(self, keycode),
        }
    }

    fn load_font(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for (i, font_char) in font.iter().enumerate() {
            self.memory[i + FONT_OFFSET as usize] = *font_char;
        }
    }

    fn load_rom(&mut self, path: &str) -> std::io::Result<()>{
        let mut rom_file = File::open(path)?;
        let mut rom_buffer = Vec::new();
        
        rom_file.read_to_end(&mut rom_buffer)?;

        if ROM_OFFSET as usize + rom_buffer.len() > self.memory.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "ROM exceeds memory",
            ));
        }

        for (i, &byte) in rom_buffer.iter().enumerate() {
            self.memory[ROM_OFFSET as usize + i] = byte;
        }

        Ok(())
    }

    /**
    Reads a 16-bit instruction from a given memory address in the Chip-8's 4K memory.

    # Parameters

    - `memory`: The 4K memory array of the Chip-8.
    - `addr`: The memory address at which the instruction starts.

    # Returns

    Returns a 16-bit instruction combining the bytes stored at `addr` and `addr + 1`.
    */
    pub fn read_memory_address(memory: [u8; 4096], addr: u16) -> u16 {
        let bigger_byte: u8 = memory[addr as usize];
        let smaller_byte: u8 = memory[addr as usize + 1];

        ((bigger_byte as u16) << 8) | (smaller_byte as u16)
    }


    pub fn clear_screen(pixel_array: &mut [[bool; WIDTH]; HEIGHT]) {
        *pixel_array = [[false; WIDTH]; HEIGHT];
    }


    pub fn fill_screen(pixel_array: &mut [[bool; WIDTH]; HEIGHT]) {
        *pixel_array = [[true; WIDTH]; HEIGHT];
    }
 

    pub fn is_valid_address(addr: u16) -> bool {
        (MIN_ADDRESS..=MAX_ADDRESS).contains(&addr)
    }

    pub fn cycle(&mut self) {
        let instruction = self.fetch();
        let decoded = self.decode(instruction);
        self.execute(decoded);
    }
}

impl Chip8 {
    // Instruction methods


    pub fn jump(pc: &mut u16, addr: u16) {
        if Self::is_valid_address(addr) {
            *pc = addr;
        }
        else {
            // TODO: handle this correctly
            panic!("JUMP: INVALID JUMP ADDRESS");
        }
    }


    pub fn call_subroutine(pc: &mut u16, stack: &mut Stack<u16>, addr: u16) -> Result<(), SubroutineError> {
        if !Self::is_valid_address(addr) {
            return Err(SubroutineError::InvalidAddress(addr));
        }

        if stack.push(*pc).is_err() {
            return Err(SubroutineError::StackOverflow);
        }

        *pc = addr;
        Ok(())
    }


    pub fn return_subroutine(pc: &mut u16, stack: &mut Stack<u16>) {
        let return_addr = stack.pop().expect("Error: Empty stack");
        *pc = return_addr;
    }


    pub fn is_valid_register(register: usize) -> bool {
        register <= 17
    }


    pub fn set(v: &mut [u8; 16], register: usize, value: u8) {
        if Self::is_valid_register(register) {
            v[register] = value;
        }
        else {
            // TODO: Handle error
            panic!("SET: INVALID REGISTER");
        }
    }


    pub fn add(v: &mut [u8; 16], register: usize, addend: u8) -> Result<(), RegisterError> {
        if !Self::is_valid_register(register) {
            return Err(RegisterError::InvalidRegister(register));
        }
        v[register] = v[register].wrapping_add(addend);
        
        Ok(())
    }


    fn set_i(i: &mut u16, value: u16) {
        *i = value;
    }


    fn display(&mut self, register_x: usize, register_y: usize, n: u8) {
        let x: usize = (self.v[register_x] as usize) % WIDTH;
        let y: usize = (self.v[register_y] as usize) % HEIGHT;
        self.v[0xF] = 0;

        for row in 0..n {
            let sprite_row: u8 = self.memory[(self.index + row as u16) as usize];
            
            let mut pixel_row: u8 = 0;
            // convert pixel array row to u8 var
            for i in 0..8 {
                if y + (row as usize) < HEIGHT && x + (i as usize) < WIDTH && self.pixel_array[y + row as usize][x + i as usize] {
                    pixel_row |= 1 << (7 - i);
                }
            }

            let final_pixel_row: u8 = pixel_row ^ sprite_row;

            if (sprite_row & pixel_row) != 0000_0000 {
                self.v[0xF] = 1;
            }

            for i in 0..8 {
                if (x + i) < WIDTH &&
                        (y + row as usize) < HEIGHT &&
                        final_pixel_row != pixel_row {
                    self.pixel_array[y + row as usize][x + i] = ((final_pixel_row >> (7 - i)) & 0b0000_0001) == 1;
                }
            }
        }
    }


    fn binary_or_vx(&mut self, register_x: usize, register_y: usize) {
        self.v[register_x] |= self.v[register_y];
    }


    fn binary_and_vx(&mut self, register_x: usize, register_y: usize) {
        self.v[register_x] &= self.v[register_y];
    }


    fn binary_xor_vx(&mut self, register_x: usize, register_y: usize) {
        self.v[register_x] ^= self.v[register_y];
    }


    fn add_vx(&mut self, register_x: usize, register_y: usize) {
        if self.v[register_x].checked_add(self.v[register_x]).is_none() {
            self.v[0xF] = 1
        }
        else {
            self.v[0xF] = 0;
        }
        self.v[register_x] = self.v[register_x].wrapping_add(self.v[register_y]);
    }


    fn add_i(&mut self, register_x: usize) { // FX1E

        /*
        check if game rom == Spaceflight 2091!. is it OK?
        if self.index.checked_add(self.v[register_x]).is_none() {
            self.v[0xF] = 1
        }
        else {
            self.v[0xF] = 0;
        }
        */
        self.index = self.index.wrapping_add(self.v[register_x].into());
    }


    fn substract_vx_vy(&mut self, register_x: usize, register_y: usize) { // 8XY5
        if self.v[register_x] > self.v[register_y] {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }
        // is wrapping sub OK?
        self.v[register_x] = self.v[register_x].wrapping_sub(self.v[register_y]);
    }


    fn substract_vy_vx(&mut self, register_x: usize, register_y: usize) { // 8XY7
        if self.v[register_y] > self.v[register_x] {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }
        // is wrapping sub OK?
        self.v[register_x] = self.v[register_y].wrapping_sub(self.v[register_x]);
    }


    fn shift_right(&mut self, register_x: usize, register_y: usize) { // 8XY6
        // if chip8.mode != SUPER_CHIP && chip8.mode != CHIP_48 {
        self.v[register_x] = self.v[register_y];
        //}

        if self.v[register_x] & 0b0000_0001 == 0b0000_0001 {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }
        self.v[register_x] >>= 1;
    }


    fn shift_left(&mut self, register_x: usize, register_y: usize) { // 8XYE
        // if chip8.mode != SUPER_CHIP && chip8.mode != CHIP_48 {
        self.v[register_x] = self.v[register_y];
        //}

        if self.v[register_x] & 0b1000_0000 == 0b1000_0000 {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }
        self.v[register_x] <<= 1;
    }


    fn skip_if_equal(&mut self, register_x: usize, value: u8) -> Result<(), RegisterError> {
        if !Self::is_valid_register(register_x) {
            return Err(RegisterError::InvalidRegister(register_x));
        }

        if self.v[register_x] == value {
            self.pc += 0x02;
        }

        Ok(())
    }


    fn skip_if_not_equal(&mut self, register_x: usize, value: u8) -> Result<(), RegisterError> {
        if !Self::is_valid_register(register_x) {
            return Err(RegisterError::InvalidRegister(register_x));
        }

        if self.v[register_x] != value {
            self.pc += 0x02;
        }

        Ok(())
    }


    fn skip_if_registers_equal (&mut self, register_x: usize, register_y: usize) -> Result<(), RegisterError> {
        if !Self::is_valid_register(register_x) {
            return Err(RegisterError::InvalidRegister(register_x ));
        }

        if !Self::is_valid_register(register_y) {
            return Err(RegisterError::InvalidRegister(register_y));
        }

        if self.v[register_x] == self.v[register_y] {
            self.pc += 0x02;
        }

        Ok(())
    }


    fn skip_if_registers_not_equal (&mut self, register_x: usize, register_y: usize) -> Result<(), RegisterError> {
        if !Self::is_valid_register(register_x) {
            return Err(RegisterError::InvalidRegister(register_x));
        }

        if !Self::is_valid_register(register_y) {
            return Err(RegisterError::InvalidRegister(register_y));
        }
        
        if self.v[register_x] != self.v[register_y] {
            self.pc += 0x02;
        }
        
        Ok(())
    }


    fn skip_if_key_pressed (&mut self, _keycode: u8) {
        let is_key_pressed = true; // TODO: input handling
        if is_key_pressed {
            self.pc += 0x02;
        }
    }

    
    fn skip_if_key_not_pressed (&mut self, _keycode: u8) {
        let is_key_pressed = true;
        if !is_key_pressed {
            self.pc += 0x02;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nibble() {
        let instruction: u16 = 0x1234;
        assert_eq!(get_nibble(instruction, 1), 0x1);
        assert_eq!(get_nibble(instruction, 2), 0x2);
        assert_eq!(get_nibble(instruction, 3), 0x3);
        assert_eq!(get_nibble(instruction, 4), 0x4);
    }

    #[test]
    fn test_call_subroutine() {
        let mut pc: u16 = 0x100;
        let mut stack: Stack<u16> = Stack::new();
        let mut addr: u16 = 0x111;

        assert!(Chip8::call_subroutine(&mut pc, &mut stack, addr).is_ok());
        assert_eq!(pc, addr);
        assert_eq!(stack.pop().unwrap(), 0x100);

        pc = 0x1000;
        addr = 0x1111;
        assert!(Chip8::call_subroutine(&mut pc, &mut stack, addr).is_err());
    }

    #[test]
    fn test_add() {
        let mut v: [u8; 16];
        v = [1; 16];
        let addend = 5;

        assert!(Chip8::add(&mut v, 2, addend).is_ok());
        assert!(Chip8::add(&mut v, 18, addend).is_err());
        assert_eq!(v[2], 6);
    }

    #[test]
    fn test_set_i() {
        let mut i: u16 = 0;
        let value: u16 = 5;

        Chip8::set_i(&mut i, value);

        assert_eq!(i, 5);
    }
}