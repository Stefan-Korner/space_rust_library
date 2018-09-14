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
// PUS Packet Utilization Standard - PUS Packet Module                        *
//*****************************************************************************
use ccsds::c_packet;
use ccsds::c_packet::PacketIntf;
//use ccsds::cuc_time;
use std::ops;
//use std::u32;
//use util::crc;
use util::du;
use util::du::DUintf;
//use util::exception;

//######################
// Packet...PUS Packet #
//######################

///////////////////////////
// implementation struct //
///////////////////////////
pub struct Packet<'a> {
    buffer: du::HybridVector<'a>
}

// trait implementations
impl<'a> ops::Index<usize> for Packet<'a> {
    type Output = u8;
    fn index(&self, pos: usize) -> &u8 {
        self.at(pos)
    }
}

impl<'a> ops::IndexMut<usize> for Packet<'a> {
    fn index_mut(&mut self, pos: usize) -> &mut u8 {
        self.at_mut(pos)
    }
}

impl<'a> du::DUintf for Packet<'a> {
    // returns a read-only reference
    fn buffer_read_only(&self) -> &[u8] {
        self.buffer.read_only()
    }
    // returns a read-write reference 
    fn buffer_read_write(&mut self) -> &mut [u8] {
        self.buffer.read_write()
    }
    // change size
    fn resize(&mut self, new_size: usize) {
        self.buffer.resize(new_size);
    }
}

impl<'a> PacketIntf for Packet<'a> {
}

// methods implementation
impl<'a> Packet<'a> {
    //////////////////
    // constructors //
    //////////////////

    // default constructor
    pub fn new() -> Packet<'a> {
        let mut packet = Packet {
            buffer: du::HybridVector::new_alloc(c_packet::PRIMARY_HEADER_BYTE_SIZE + 1)
        };
        packet.set_packet_length().unwrap();
        packet
    }
    // copy constructor
    pub fn new_clone(value: &Vec<u8>) -> Packet<'a> {
        Packet {
            buffer: du::HybridVector::new_clone(value)
        }
    }
    // allocating constructor
    pub fn new_alloc(size: usize) -> Packet<'a> {
        let mut packet = Packet {
            buffer: du::HybridVector::new_alloc(size)
        };
        packet.set_packet_length().unwrap();
        packet
    }
    // move ownership
    pub fn new_owner(value: Vec<u8>) -> Packet<'a> {
        Packet {
            buffer: du::HybridVector::new_owner(value)
        }
    }
    // wraps data for read-only
    pub fn new_read_only(reference: &[u8]) -> Packet {
        Packet {
            buffer: du::HybridVector::new_read_only(reference)
        }
    }
    // wraps data for read-write
    pub fn new_read_write(reference: &mut [u8]) -> Packet {
        Packet {
            buffer: du::HybridVector::new_read_write(reference)
        }
    }
}

//##################################
// TMpacket...PUS Telemetry Packet #
//##################################

///////////////////////////
// implementation struct //
///////////////////////////
pub struct TMpacket<'a> {
    buffer: du::HybridVector<'a>
}

// trait implementations
impl<'a> ops::Index<usize> for TMpacket<'a> {
    type Output = u8;
    fn index(&self, pos: usize) -> &u8 {
        self.at(pos)
    }
}

impl<'a> ops::IndexMut<usize> for TMpacket<'a> {
    fn index_mut(&mut self, pos: usize) -> &mut u8 {
        self.at_mut(pos)
    }
}

impl<'a> du::DUintf for TMpacket<'a> {
    // returns a read-only reference
    fn buffer_read_only(&self) -> &[u8] {
        self.buffer.read_only()
    }
    // returns a read-write reference 
    fn buffer_read_write(&mut self) -> &mut [u8] {
        self.buffer.read_write()
    }
    // change size
    fn resize(&mut self, new_size: usize) {
        self.buffer.resize(new_size);
    }
}

impl<'a> PacketIntf for TMpacket<'a> {
}

// methods implementation
impl<'a> TMpacket<'a> {
    //////////////////
    // constructors //
    //////////////////

    // default constructor
    pub fn new() -> TMpacket<'a> {
        let mut packet = TMpacket {
            buffer: du::HybridVector::new_alloc(c_packet::PRIMARY_HEADER_BYTE_SIZE + 1)
        };
        packet.set_packet_length().unwrap();
        packet
    }
    // copy constructor
    pub fn new_clone(value: &Vec<u8>) -> TMpacket<'a> {
        TMpacket {
            buffer: du::HybridVector::new_clone(value)
        }
    }
    // allocating constructor
    pub fn new_alloc(size: usize) -> TMpacket<'a> {
        let mut packet = TMpacket {
            buffer: du::HybridVector::new_alloc(size)
        };
        packet.set_packet_length().unwrap();
        packet
    }
    // move ownership
    pub fn new_owner(value: Vec<u8>) -> TMpacket<'a> {
        TMpacket {
            buffer: du::HybridVector::new_owner(value)
        }
    }
    // wraps data for read-only
    pub fn new_read_only(reference: &[u8]) -> TMpacket {
        TMpacket {
            buffer: du::HybridVector::new_read_only(reference)
        }
    }
    // wraps data for read-write
    pub fn new_read_write(reference: &mut [u8]) -> TMpacket {
        TMpacket {
            buffer: du::HybridVector::new_read_write(reference)
        }
    }
}

//###################################
// TCpacket...PUS Telecomand Packet #
//###################################

///////////////////////////
// implementation struct //
///////////////////////////
pub struct TCpacket<'a> {
    buffer: du::HybridVector<'a>
}

// trait implementations
impl<'a> ops::Index<usize> for TCpacket<'a> {
    type Output = u8;
    fn index(&self, pos: usize) -> &u8 {
        self.at(pos)
    }
}

impl<'a> ops::IndexMut<usize> for TCpacket<'a> {
    fn index_mut(&mut self, pos: usize) -> &mut u8 {
        self.at_mut(pos)
    }
}

impl<'a> du::DUintf for TCpacket<'a> {
    // returns a read-only reference
    fn buffer_read_only(&self) -> &[u8] {
        self.buffer.read_only()
    }
    // returns a read-write reference 
    fn buffer_read_write(&mut self) -> &mut [u8] {
        self.buffer.read_write()
    }
    // change size
    fn resize(&mut self, new_size: usize) {
        self.buffer.resize(new_size);
    }
}

impl<'a> PacketIntf for TCpacket<'a> {
}

// methods implementation
impl<'a> TCpacket<'a> {
    //////////////////
    // constructors //
    //////////////////

    // default constructor
    pub fn new() -> TCpacket<'a> {
        let mut packet = TCpacket {
            buffer: du::HybridVector::new_alloc(c_packet::PRIMARY_HEADER_BYTE_SIZE + 1)
        };
        packet.set_packet_length().unwrap();
        packet
    }
    // copy constructor
    pub fn new_clone(value: &Vec<u8>) -> TCpacket<'a> {
        TCpacket {
            buffer: du::HybridVector::new_clone(value)
        }
    }
    // allocating constructor
    pub fn new_alloc(size: usize) -> TCpacket<'a> {
        let mut packet = TCpacket {
            buffer: du::HybridVector::new_alloc(size)
        };
        packet.set_packet_length().unwrap();
        packet
    }
    // move ownership
    pub fn new_owner(value: Vec<u8>) -> TCpacket<'a> {
        TCpacket {
            buffer: du::HybridVector::new_owner(value)
        }
    }
    // wraps data for read-only
    pub fn new_read_only(reference: &[u8]) -> TCpacket {
        TCpacket {
            buffer: du::HybridVector::new_read_only(reference)
        }
    }
    // wraps data for read-write
    pub fn new_read_write(reference: &mut [u8]) -> TCpacket {
        TCpacket {
            buffer: du::HybridVector::new_read_write(reference)
        }
    }
}
