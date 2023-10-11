# Octorust 👾🦀
 
## Description 🧬

`Octorust` is a CHIP-8 interpreter/emulator in the making, written in Rust. This project aims to offer a reliable and efficient implementation of the CHIP-8 system for educational and development purposes.

## Features 💫

- CHIP-8 instruction interpretation

## Development Status 🖌️

This project is in its early stages but aims to provide a solid foundation for CHIP-8 emulation.

### Basic instructions:

  | Status   | Code   | Instruction |
  |:--------:|:------:|-------------| 
  | ✅       | `00E0` | clear screen |
  | ✅       | `1NNN` | jump |
  | ✅       | `2NNN` | subroutine call |
  | ✅       | `00EE` | subroutine return |
  | ✅       | `6XNN` | set register VX |
  | ✅       | `7XNN` | add value to register VX |
  | ✅       | `ANNN` | set index register I
  | ⌛       | `DXYN` | display/draw
    
  Notes on display/draw: X and Y are the register index which store the coordinates to draw, X and Y respectively. N is the position of the sprite to write, starting from the position stored in register I (index).
  A pixel switch its value (from 0 to 1 or viceversa) if and only if sprite bit is 1. Otherwise, it stays on its original value.
  This instruction also writes on register `v[0xF]`, a value of 1 if the sprite switched a pixel array from 1 to 0. Otherwise, `V[0xF]` is set to 0. The resultant truth table is the following:

  | P0 | Sprite bit | PF | V[0xF] |
  |----|------------|---:|-------:|
  | 0  | 0          | 0  | 0      |
  | 0  | 1          | 1  | 0      |
  | 1  | 0          | 1  | 0      |
  | 1  | 1          | 0  | 1      |

  `P0` is the original pixel value and `PF` is the pixel value after applying its sprite bit.

  Which means that:
  `PF = P0 ⊕ Sprite bit`
  `V[0xF] = P0 & Sprite bit`
  
### Rest of instructions:
  🔜
### Input handling:
  🔜

## Thank you! 💕

Last but not least, I'd like to give a big thank you to everyone who helped along the way. 😊

- [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM). ❤️
- [Tobias V. Langhoff's Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/). ❤️
- A special thank you to [nifIheimr](https://github.com/nifIheimr) for providing the seed idea that ultimately blossomed into this project. ❤️
