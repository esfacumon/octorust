// use crate::stack::Stack;

pub const MIN_ADDRESS: u16 = 0x001;
pub const MAX_ADDRESS: u16 = 0xFFF;
pub enum Instruction {
    ClearScreen,
    FillScreen,
    Jump { addr: u16}
    // ...
}

pub struct Chip8 {
    pub pixel_array: [[bool; 640]; 320],
    memory: [u8; 4096],
    // index: u16,
    pc: u16,
    // stack: Stack<u16>,
    // delay: u8,
    // sound: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            pixel_array: [[false; 640]; 320],
            memory: [0; 4096],
            // index: 0,
            pc: 0,
            // stack: Stack::new(),
            // delay: 0,
            // sound: 0,
        };
        chip8.memory[3] = 0b0000_0001;

        // Clear screen:
        chip8.memory[4] = 0x00;
        chip8.memory[5] = 0xE0;

        // JUMP:
        chip8.memory[6] = 0x10;
        chip8.memory[7] = 0x02;
        chip8
    }

    
    /**
     * Read addr stored in PC from memory and returns it
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
                    _ => Instruction::FillScreen,
                }
            }
            0x1 => {
                let addr: u16 = instruction % 0x1000;
                Instruction::Jump { addr }
            },
            _ => Instruction::FillScreen
        }
    }


    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen => Chip8::clear_screen(&mut self.pixel_array),
            Instruction::FillScreen => Chip8::fill_screen(&mut self.pixel_array),
            Instruction::Jump { addr } => Chip8::jump(&mut self.pc, addr),
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
        // TODO: update limits
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


    pub fn cycle(&mut self) {
        let instruction = self.fetch(); // not writing on memory so it's immutable. By reference bc we don't want it to take ownership. it shouldn't matter anyways.
        let decoded = self.decode(instruction);
        self.execute(decoded);
    }

}
