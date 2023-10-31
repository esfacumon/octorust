pub enum Instruction {
    ClearScreen,
    FillScreen,
    Jump { addr: u16},
    CallSubroutine { addr: u16},
    ReturnSubroutine,
    Set { register: usize, value: u8},
    Add { register: usize, value: u8},
    SetI { value: u16 },
    DisplayDraw { x: u16, y: u16, n: u16},
}