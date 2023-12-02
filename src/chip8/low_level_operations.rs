// low_level_operations.rs

/**
Returns a specific 4-bit nibble (half-byte) from a 16-bit address.

# Returns
Returns the desired 4-bit nibble from the 16-bit memory address.

If an invalid `nibble_number` is provided (not in the range 1-4), the function returns 0.

# Note

The nibble positions are as follows for a 16-bit address:
* 1: Bits 15-12
* 2: Bits 11-8
* 3: Bits 7-4
* 4: Bits 3-0
*/
pub fn get_nibble(addr: u16, nibble_number: u8) -> u8 {
    let bit_mask: u16 = match nibble_number {
        1 => 0b1111_0000_0000_0000,
        2 => 0b0000_1111_0000_0000,
        3 => 0b0000_0000_1111_0000,
        4 => 0b0000_0000_0000_1111,
        _ => 0,
    };
    ((addr & bit_mask) >> (12 - (4 * (nibble_number - 1)))).try_into().unwrap()
}

