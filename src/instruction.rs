enum Instruction {
    Jump(u16),
    SetRegister(u8, u8),
    AddToRegister(u8, u8),
    SetIndexRegister(u16),
    DisplayDraw(u8, u8, u8),
}
 