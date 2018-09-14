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
// PUS Packet Utilization Standard - PUS Packet Module - Unit tests           *
//*****************************************************************************

use ccsds::c_packet;
use util::du;

// some accessors from util::du and ccsds::c_packet to check compile cleaness
def_unsigned_accessor!(UINT_ACC, 0, 0);
def_big_unsigned_accessor!(BUINT_ACC, 0, 0);
def_bit_accessor!(BIT_ACC,  0,  0);
def_byte_accessor!(BYTE_ACC, 0, 0);
def_cuc_time_accessor!(CUC_TIME_ACC, 0, 0);

pub fn test() {
}
