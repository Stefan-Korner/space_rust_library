//*****************************************************************************
// (C) 2018, Stefan Korner, Austria                                           *
//                                                                            *
// The Space Rust Library is free software; you can redistribute it and/or    *
// modify it under the terms of the MIT License as published by the           *
// Massachusetts Institute of Technology.                                     *
//                                                                            *
// The Space Rust Library is distributed in the hope that it will be useful,  *
// but WITHOUT ANY WARRANTY; without even the implied warranty of             *
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the MIT License   *
// for more details.                                                          *
//*****************************************************************************
// Utilities - Cyclic Redundancy Check (CRC)                                  *
//*****************************************************************************

// calculates the CRC-16 from the byte array
pub fn calculate16(bytes: &[u8], size: usize) -> u16 {
    // 32 bit shift register for CRC generation
    // D0  - D15  :CRC shift register
    // D16        : MSB after shift
    // D17 - D31  : not used
    // shift register preset with all ones
    let mut shift_reg = 0x0000FFFF_u32;
    // generator polynom D0-D15: X^16 + X^12 + X^5 + X^0
    let polynom = 0x00001021_u32;
    for i in 0..size {
        let next_byte = bytes[i];
        // loop over 8 bit
        for bit_no in (0_u8..8_u8).rev() {
            // evaluate bit in data string
            let mask = 1 << bit_no;
            // set D16 in help var. EXOR with shift
            let h = if (next_byte & mask) > 0 {0x00010000_u32} else {0_u32};
            // clock the shift register
            shift_reg <<= 1;
            // evaluate the bit that falls out of the shift register,
            // simultaneously add the input data bit (rightmost + in diagram),
            // this covers the X^16 term
            if (h ^ (shift_reg & 0x00010000_u32)) > 0 {
                // check D16 in shift and then here, the level behind GATE A is one
                // add (i.e. XOR) the X^0 + X^5 + X^12 polynome to the shift register
                shift_reg ^= polynom;
            }
            // the else branch is empty, as the level behind gate A is 0
            // and XORing with zero has no effect
        }
    }
    (shift_reg & 0x0000FFFF_u32) as u16
}
