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
// CCSDS Stack - CCSDS Packet Module                                          *
//*****************************************************************************
use std::ops;
use util::du;
use util::du::DUintf;

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

// methods implementation
impl<'a> Packet<'a> {
    //////////////////
    // constructors //
    //////////////////

    // default constructor
    pub fn new() -> Packet<'a> {
        Packet {
            buffer: du::HybridVector::new()
        }
    }
    // copy constructor
    pub fn new_clone(value: &Vec<u8>) -> Packet<'a> {
        Packet {
            buffer: du::HybridVector::new_clone(value)
        }
    }
    // allocating constructor
    pub fn new_alloc(size: usize) -> Packet<'a> {
        Packet {
            buffer: du::HybridVector::new_alloc(size)
        }
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
