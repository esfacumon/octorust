use crate::stack::Stack;
use crate::errors::{SubroutineError, RegisterError};

// TODO:  address limits
pub const MIN_ADDRESS: u16 = 0x001;
pub const MAX_ADDRESS: u16 = 0xFFF;
pub const MEMORY_SIZE: usize = 0x1000;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 320;

pub enum Instruction {
    ClearScreen,
    FillScreen,
    Jump { addr: u16},
    CallSubroutine { addr: u16},
    ReturnSubroutine,
    Set { register: usize, value: u8},
    // ...
}

pub struct Chip8 {
    pub pixel_array: [[bool; WIDTH]; HEIGHT],
    memory: [u8; MEMORY_SIZE],
    // i: u16, // index register
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
            // index: 0,
            pc: 0,
            stack: Stack::new(),
            v: [0; 16],
            // delay: 0,
            // sound: 0,
        };
        chip8.memory[3] = 0b0000_0001;

        // set v[1] = 23:
        chip8.memory[4] = 0x61;
        chip8.memory[5] = 0x23;

        // JUMP:
        chip8.memory[6] = 0x10;
        chip8.memory[7] = 0x0A;

        // SUBROUTINE CALL
        chip8.memory[10] = 0x22;
        chip8.memory[11] = 0x00;

        // SUBROUTINE RETURN
        chip8.memory[0x20A] = 0x00;
        chip8.memory[0x20B] = 0xEE;

        // inner subroutine call
        chip8.memory[518] = 0x25;
        chip8.memory[519] = 0x00;
        
        // inner return
        chip8.memory[0x504] = 0x00;
        chip8.memory[0x505] = 0xEE;

        chip8
    }

    
    /**
    Read addr stored in PC from memory and returns its value

    # Returns
    Instruction code
     */
    pub fn fetch(&mut self) -> u16 {
        println!("fet::PC: {}", self.pc);
        
        let instruction = Chip8::read_memory_address(self.memory, self.pc);
        println!("fet::INST READ: 0x{:04X}", instruction);
        
        self.pc += 0x02;
        
        return instruction;
    }


    pub fn decode(&mut self, instruction: u16) -> Instruction {
        let first_nibble = Chip8::get_nibble(instruction, 1);

        match first_nibble {
            0x0 => {
                match instruction {
                    0x00E0 =>  Instruction::ClearScreen,
                    0x00EE => Instruction::ReturnSubroutine,
                    _ => Instruction::FillScreen,
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
            0x6 => {
                let register = Chip8::get_nibble(instruction, 2) as usize;
                let value: u8 = (instruction % 0x0100) as u8;
                Instruction::Set { register, value }
            }
            _ => Instruction::FillScreen
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
            Instruction::Set { register, value } => Chip8::set(&mut self.v, register, value)
        }
    }

    /**
    Reads a 16-bit instruction from a given memory address in the Chip-8's 4K memory.

    # Parameters

    - `memory`: The 4K memory array of the Chip-8.
    - `addr`: The memory address at which the instruction starts.

    # Returns

    Returns a 16-bit instruction combining the bytes stored at `addr` and `addr + 1`.

    # Example

    ```
    let memory: [u8; 4096] = [/* your Chip-8 memory here */];
    let addr: u16 = 0x200;
    let instruction = read_memory_address(memory, addr);
    ```
    */
    pub fn read_memory_address(memory: [u8; 4096], addr: u16) -> u16 {
        let bigger_byte: u8 = memory[addr as usize];
        let smaller_byte: u8 = memory[addr as usize + 1];

        let instruction = ((bigger_byte as u16) << 8) | (smaller_byte as u16);
        return instruction;
    }


    /**
    Returns a specific 4-bit nibble (half-byte) from a 16-bit address.

    # Parameters

    - `addr`: The memory address from which to extract the nibble.
    - `nibble_number`: The position of the desired nibble. Ranges from 1 to 4.

    # Returns

    Returns the desired 4-bit nibble from the 16-bit memory address.

    If an invalid `nibble_number` is provided (not in the range 1-4), the function returns 0.
    
    # Example

    ```rust
    let addr: u16 = 0xABCD;
    let nibble = get_nibble(addr, 1);  // Should return 0xA
    ```

    # Note
    
    The nibble positions are as follows for a 16-bit address:
    * 1: Bits 15-12
    * 2: Bits 11-8
    * 3: Bits 7-4
    * 4: Bits 3-0

    # TODO

    - Handle invalid `nibble_number` inputs more gracefully, possibly with an error message or custom error type.
    */
    pub fn get_nibble(addr: u16, nibble_number: u8) -> u16 {
        let bit_mask: u16;
        match nibble_number {
            1 => bit_mask = 0b1111_0000_0000_0000,
            2 => bit_mask = 0b0000_1111_0000_0000,
            3 => bit_mask = 0b0000_0000_1111_0000,
            4 => bit_mask = 0b0000_0000_0000_1111,
            _ => bit_mask = 0,
        }
        (addr & bit_mask) >> 12 - (4 * (nibble_number - 1))
    }


    pub fn clear_screen(pixel_array: &mut [[bool; 640]; 320]) {
        println!("EXE: CLEAR SCREEN");
        *pixel_array = [[false; 640]; 320];
    }


    pub fn fill_screen(pixel_array: &mut [[bool; 640]; 320]) {
        println!("EXE: FILL SCREEN");
        *pixel_array = [[true; 640]; 320];
        
        /*
        for y in 0..159 {
            for x in 0..640{
                pixel_array[y][x] = true;
            }
        }
        */
    }
 

    pub fn is_valid_address(addr: u16) -> bool {
        addr >= MIN_ADDRESS && addr <= MAX_ADDRESS
    }


    /**
    Makes PC point to given address

    # Parameters

    - `pc`: Current program counter
    - `addr`: Address to jump
    */
    pub fn jump(pc: &mut u16, addr: u16) {
        println!("EXE: JUMP INSTRUCTION");
        
        if Self::is_valid_address(addr) {
            *pc = addr;
        }
        else {
            println!("EXE: INVALID JUMP ADDRESS");
        }
    }

    pub fn call_subroutine(pc: &mut u16, stack: &mut Stack<u16>, addr: u16) -> Result<(), SubroutineError> {
        println!("EXE: CALL INSTRUCTION");

        if !Self::is_valid_address(addr) {
            return Err(SubroutineError::InvalidAddress(addr));
        }

        if let Err(_) = stack.push(*pc) {
            return Err(SubroutineError::StackOverflow);
        }

        println!("STACK LENGTH: {:?}", stack.len());
        *pc = addr;
        Ok(())
    }

    pub fn return_subroutine(pc: &mut u16, stack: &mut Stack<u16>) {
        println!("EXE: RETURN INSTRUCTION");

        let return_addr = stack.pop().expect("Error: Empty stack");
        *pc = return_addr;
    }

    pub fn is_valid_register(register: usize) -> bool {
        register <= 17
    }

    pub fn set(v: &mut [u8; 16], register: usize, value: u8) {
        println!("EXE: SET INSTRUCTION");
        if Self::is_valid_register(register) {
            v[register] = value;
            println!("v[{}] = {} || valor real = {}", register, value, v[1]);
        }
        else {
            // TODO: Handle error (panic?)
        }
    }

    /**
    Add `addend` to `register`. If overflows, it just wraps without any carry register affected
     */
    pub fn add(v: &mut [u8; 16], register: usize, addend: u8) -> Result<(), RegisterError>{
        
        if !Self::is_valid_register(register) {
            return Err(RegisterError::InvalidRegister(register as u8));
        }
        v[register] = v[register].wrapping_add(addend);
        Ok(())
    }


    pub fn cycle(&mut self) {
        let instruction = self.fetch();
        let decoded = self.decode(instruction);
        self.execute(decoded);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nibble() {
        let instruction: u16 = 0x1234;
        assert_eq!(Chip8::get_nibble(instruction, 1), 0x1);
        assert_eq!(Chip8::get_nibble(instruction, 2), 0x2);
        assert_eq!(Chip8::get_nibble(instruction, 3), 0x3);
        assert_eq!(Chip8::get_nibble(instruction, 4), 0x4);
    }

    #[test]
    fn test_call_subroutine() {
        let mut pc: u16 = 0x100;
        let mut stack: Stack<u16> = Stack::new();
        let mut addr: u16 = 0x111;

        assert!(Chip8::call_subroutine(&mut pc, &mut stack, addr).is_ok());
        assert_eq!(pc, addr);
        assert_eq!(stack.pop().unwrap(), 0x100);


        // testing error handling
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
}