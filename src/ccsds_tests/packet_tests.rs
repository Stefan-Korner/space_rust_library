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
// CCSDS Stack - CCSDS Packet Module - Unit tests                             *
//*****************************************************************************
use ccsds::packet;
use util::du::DUintf;

pub fn test() {
    let packet = packet::Packet::new();
    println!("packet = {}", packet.dump_str());
    let tm_packet = packet::TMpacket::new();
    println!("tm_packet = {}", tm_packet.dump_str());
    let tc_packet = packet::TCpacket::new();
    println!("tc_packet = {}", tc_packet.dump_str());
}
