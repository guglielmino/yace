|Opcode | Description |
|---|---|
| 0x00E0 | Clear the screen. |
| 0x00EE | Return from a subroutine. |
| 0x0000 | Calls subroutine at address NNN. |
| 0x1NNN | Jumps to address NNN. |
| 0x2NNN | Calls subroutine at address NNN. |
| 0x3XNN | Skips the next instruction if VX equals NN. |
| 0x4XNN | Skips the next instruction if VX doesn't equal NN. |
| 0x5XY0 | Skips the next instruction if VX equals VY. |
| 0x6XNN | Sets VX to the value NN. |
| 0x7XNN | Adds NN to VX. |
| 0x8XY0 | Sets VX to the value of VY. |
| 0x8XY1 | Sets VX to VX OR VY. |
| 0x8XY2 | Sets VX to VX AND VY. |
| 0x8XY3 | Sets VX to VX XOR VY. |
| 0x8XY4 | Adds VY to VX. VF is set to 1 when there is a carry, and to 0 when there is not. |
| 0x8XY5 | Subtracts VY from VX. VF is set to 0 when there is a borrow, and to 1 when there is not. |
| 0x8XY6 | Shifts VX right by one bit. VF is set to the least significant bit of VX before the shift. |
| 0x8XY7 | Sets VX to VY minus VX. VF is set to 0 when there is a borrow, and to 1 when there is not. |
| 0x8XYE | Shifts VX left by one bit. VF is set to the most significant bit of VX before the shift. |
| 0x9XY0 | Skips the next instruction if VX doesn't equal VY. |
| 0xANNN | Sets I to the address NNN. |
| 0xBNNN | Jumps to the address NNN plus V0. |
| 0xCXNN | Sets VX to a random number AND NN. |
| 0xDXYN | Draws a sprite at coordinate (VX, VY) that is N pixels wide and N pixels high. Sprites are XORed onto the existing screen. If a sprite pixel is set, the corresponding screen pixel is flipped from black to white and vice-versa. If a sprite pixel is unset, the corresponding screen pixel is left unchanged. VF is set to 1 if any screen pixels are flipped from white to black, and to 0 if none are flipped. |
| 0xEX9E | Skips the next instruction if the key with the value of VX is pressed. |
| 0xEXA1 | Skips the next instruction if the key with the value of VX is not pressed. |
| 0xFX07 | Sets VX to the value of the delay timer. |
| 0xFX0A | A key press is awaited, and then stored in VX. |
| 0xFX15 | Sets the delay timer to VX. |
| 0xFX18 | Sets the sound timer to VX. |
| 0xFX1E | Adds VX to I. |
| 0xFX29 | Sets I to the location of the sprite for the character in VX. |
| 0xFX33 | Stores the binary-coded decimal representation of VX, with the most significant digit at I, the next most significant digit at I + 1, and so on. |
| 0xFX55 | Stores the registers V0 through VX in memory starting at location I. |
| 0xFX65 | Reads the registers V0 through VX from memory starting at location I. |
