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
// Utilities - Data Unit                                                      *
//*****************************************************************************
use std::ops;
use util::exception;

///////////////////////////////////
// accessors for different types //
///////////////////////////////////

pub struct BitAccessor {
    pub bit_pos: usize,
    pub bit_length: usize
}
#[macro_export]
macro_rules! def_bit_accessor {
    ($acc_name: ident, $bit_pos: expr, $bit_length: expr) => {
        const $acc_name: du::BitAccessor = du::BitAccessor {bit_pos: $bit_pos, bit_length: $bit_length};
    };
}

pub struct ByteAccessor {
    pub byte_pos: usize,
    pub byte_length: usize
}
#[macro_export]
macro_rules! def_byte_accessor {
    ($acc_name: ident, $byte_pos: expr, $byte_length: expr) => {
        const $acc_name: du::ByteAccessor = du::ByteAccessor {byte_pos: $byte_pos, byte_length: $byte_length};
    };
}

pub struct UnsignedAccessor {
    pub byte_pos: usize,
    pub byte_length: usize
}
#[macro_export]
macro_rules! def_unsigned_accessor {
    ($acc_name: ident, $byte_pos: expr, $byte_length: expr) => {
        const $acc_name: du::UnsignedAccessor = du::UnsignedAccessor {byte_pos: $byte_pos, byte_length: $byte_length};
    };
}

pub struct BigUnsignedAccessor {
    pub byte_pos: usize,
    pub byte_length: usize
}
#[macro_export]
macro_rules! def_big_unsigned_accessor {
    ($acc_name: ident, $byte_pos: expr, $byte_length: expr) => {
        const $acc_name: du::BigUnsignedAccessor = du::BigUnsignedAccessor {byte_pos: $byte_pos, byte_length: $byte_length};
    };
}

/////////////////////
// interface trait //
/////////////////////
pub trait DUintf {
    // returns a read-only reference, must be implemented in struct
    fn buffer_read_only(&self) -> &Vec<u8>;
    // returns a read-write reference, must be implemented in struct 
    fn buffer_read_write(&mut self) -> &mut Vec<u8>;

    ///////////////////////
    // general accessors //
    ///////////////////////

    // returns the size
    fn size(&self) -> usize {
        self.buffer_read_only().len()
    }
    // change size
    fn resize(&mut self, new_size: usize) {
        let old_size = self.size();
        if old_size < new_size {
            let additional = new_size - old_size;
            self.buffer_read_write().reserve(additional);
            let mut i = 0;
            while i < new_size {
                self.buffer_read_write().push(0);
                i = i + 1;
            }
        } else {
            unsafe {
                self.buffer_read_write().set_len(new_size);
            }
        }
    }
    // dumps the buffer to a string
    fn dump_str(&self) -> String {
        let mut ret_val = String::new();
        let mut length = self.size();
        if length == 0 {
            ret_val += "EMPTY";
            return ret_val;
        }
        let data_buffer = self.buffer_read_only();
        // display only the first 64K bytes
        if length > 65536 {
            length = 65536;
        }
        let mut i: usize = 0;
        while i < length {
            ret_val += &format!("\n{:04x} ", i);
            let mut ascii_data = String::new();
            let mut j: usize = 0;
            while j < 16 {
                let offset = i + j;
                if offset >= length {
                    break;
                }
                let next_byte = data_buffer[offset];
                ret_val += &format!("{:02x} ", next_byte);
                if (next_byte < 32) || (next_byte > 127) {
                    ascii_data.push('.');
                } else {
                    ascii_data.push(next_byte as char);
                }
                j += 1;
            }
            // fillup the space if the line was incomplete
            while j < 16 {
                ret_val += "   ";
                j += 1;
            }
            // append the ASCII representation of the line
            ret_val += &ascii_data;
            i += 16;
        }
        ret_val
    }
    // dumps the buffer to standard out
    fn dump(&self, prefix: &str) {
        println!("{} = {}", prefix, self.dump_str());
    }

    /////////////////////
    // field accessors //
    /////////////////////

    // access one byte
    fn at(&self, pos: usize) -> &u8 {
        let size = self.size(); 
        if pos >= size {
            panic!("pos out of bounds: the size is {} but the pos is {}", size, pos);
        }
        &self.buffer_read_only()[pos]
    }
    fn at_mut(&mut self, pos: usize) -> &mut u8 {
        let size = self.size(); 
        if pos >= size {
            panic!("pos out of bounds: the size is {} but the pos is {}", size, pos);
        }
        &mut self.buffer_read_write()[pos]
    }

    // bit aligned access
    fn get_bits(&self, _bit_pos: usize, _bit_length: usize) ->
        Result<u32, exception::Exception> {
        Ok(1)
    }
    fn set_bits(&mut self, _bit_pos: usize, _bit_length: usize, _value: u32) ->
        Result<(), exception::Exception> {
        Ok(())
    }
    fn get_bits_acc(&self, acc: BitAccessor) ->
        Result<u32, exception::Exception> {
        self.get_bits(acc.bit_pos, acc.bit_length)
    }
    fn set_bits_acc(&mut self, acc: BitAccessor, value: u32) ->
        Result<(), exception::Exception> {
        self.set_bits(acc.bit_pos, acc.bit_length, value)
    }

    // unsigned integer access
    fn get_unsigned(&self, byte_pos: usize, byte_length: usize) ->
        Result<u32, exception::Exception> {
        // consistency checks
        if (byte_length == 0) || (byte_length > 4) {
            return Err(exception::raise("invalid byte_length"));
        }
        let last_byte_pos = byte_pos + byte_length - 1;
        if last_byte_pos >= self.size() {
            return Err(exception::raise("byte_pos/byte_length out of buffer"));
        }
        // accumulate the number starting with the first byte
        let mut value: u32 = 0;
        let mut next_byte_pos = byte_pos;
        while next_byte_pos <= last_byte_pos {
            let byte = self.buffer_read_only()[next_byte_pos];
            value = (value << 8) + (byte as u32);
            next_byte_pos += 1;
        }
        Ok(value)
    }
    fn set_unsigned(&mut self, byte_pos: usize, byte_length: usize, value: u32) ->
        Result<(), exception::Exception> {
        // consistency checks
        if (byte_length == 0) || (byte_length > 4) {
            return Err(exception::raise("invalid byte_length"));
        }
        if ((byte_length == 1) && (value > 255)) ||
           ((byte_length == 2) && (value > 65535)) ||
           ((byte_length == 3) && (value > 16777215)) {
            return Err(exception::raise("value out of range"));
        }
        if (byte_pos + byte_length) > self.size() {
            return Err(exception::raise("byte_pos/byte_length out of buffer"));
        }
        // decompose the value and add it to the buffer
        // starting at next_byte_pos, which is at the last byte
        let first_byte_pos = byte_pos;
        let mut next_byte_pos = first_byte_pos + byte_length - 1;
        let mut next_value = value;
        while next_byte_pos >= first_byte_pos {
            let byte = (next_value & 0xFF) as u8;
            self.buffer_read_write()[next_byte_pos] = byte;
            next_value >>= 8;
            if next_byte_pos == 0 {
                break;
            }
            next_byte_pos -= 1;
        }
        Ok(())
    }
    fn get_unsigned_acc(&self, acc: UnsignedAccessor) ->
        Result<u32, exception::Exception> {
        self.get_unsigned(acc.byte_pos, acc.byte_length)
    }
    fn set_unsigned_acc(&mut self, acc: UnsignedAccessor, value: u32) ->
        Result<(), exception::Exception> {
        self.set_unsigned(acc.byte_pos, acc.byte_length, value)
    }

    // big unsigned integer access
    fn get_big_unsigned(&self, byte_pos: usize, byte_length: usize) ->
        Result<u64, exception::Exception> {
        // consistency checks
        if (byte_length == 0) || (byte_length > 8) {
            return Err(exception::raise("invalid byte_length"));
        }
        let last_byte_pos = byte_pos + byte_length - 1;
        if last_byte_pos >= self.size() {
            return Err(exception::raise("byte_pos/byte_length out of buffer"));
        }
        // accumulate the number starting with the first byte
        let mut value: u64 = 0;
        let mut next_byte_pos = byte_pos;
        while next_byte_pos <= last_byte_pos {
            let byte = self.buffer_read_only()[next_byte_pos];
            value = (value << 8) + (byte as u64);
            next_byte_pos += 1;
        }
        Ok(value)
    }
    fn set_big_unsigned(&mut self, byte_pos: usize, byte_length: usize, value: u64) ->
        Result<(), exception::Exception> {
        // consistency checks
        if (byte_length == 0) || (byte_length > 8) {
            return Err(exception::raise("invalid byte_length"));
        }
        if ((byte_length == 1) && (value > 255)) ||
           ((byte_length == 2) && (value > 65535)) ||
           ((byte_length == 3) && (value > 16777215)) ||
           ((byte_length == 4) && (value > 4294967295)) ||
           ((byte_length == 5) && (value > 1099511627775)) ||
           ((byte_length == 6) && (value > 281474976710655)) ||
           ((byte_length == 7) && (value > 72057594037927935)) {
            return Err(exception::raise("value out of range"));
        }
        if (byte_pos + byte_length) > self.size() {
            return Err(exception::raise("byte_pos/byte_length out of buffer"));
        }
        // decompose the value and add it to the buffer
        // starting at next_byte_pos, which is at the last byte
        let first_byte_pos = byte_pos;
        let mut next_byte_pos = first_byte_pos + byte_length - 1;
        let mut next_value = value;
        while next_byte_pos >= first_byte_pos {
            let byte = (next_value & 0xFF) as u8;
            self.buffer_read_write()[next_byte_pos] = byte;
            next_value >>= 8;
            if next_byte_pos == 0 {
                break;
            }
            next_byte_pos -= 1;
        }
        Ok(())
    }
    fn get_big_unsigned_acc(&self, acc: BigUnsignedAccessor) ->
        Result<u64, exception::Exception> {
        self.get_big_unsigned(acc.byte_pos, acc.byte_length)
    }
    fn set_big_unsigned_acc(&mut self, acc: BigUnsignedAccessor, value: u64) ->
        Result<(), exception::Exception> {
        self.set_big_unsigned(acc.byte_pos, acc.byte_length, value)
    }
}

/////////////////
// helper type //
/////////////////

// Data Unit's internal vector that supports different type of ownerships
enum HybridVector<'a> {
    Owner(Vec<u8>),
    ReadWrite(&'a mut Vec<u8>),
    ReadOnly(&'a Vec<u8>),
}
impl<'a> HybridVector<'a> {
    // default constructor
    fn new() -> HybridVector<'a> {
        HybridVector::Owner(Vec::new())
    }
    // copy constructor
    fn new_clone(value: &Vec<u8>) -> HybridVector<'a> {
        HybridVector::Owner(value.to_vec())
    }
    // allocating constructor
    fn new_alloc(size: usize) -> HybridVector<'a> {
        HybridVector::Owner(vec![0; size])
    }
    // move ownership
    fn new_owner(value: Vec<u8>) -> HybridVector<'a> {
        HybridVector::Owner(value)
    }
    // wraps data for read-only
    fn new_read_only(reference: &Vec<u8>) -> HybridVector {
        HybridVector::ReadOnly(reference)
    }
    // wraps data for read-write
    fn new_read_write(reference: &mut Vec<u8>) -> HybridVector {
        HybridVector::ReadWrite(reference)
    }
    // returns a read-only reference
    fn read_only(&self) -> &Vec<u8> {
        match self {
            &HybridVector::Owner(ref read_write_ref) => {
                read_write_ref
            },
            &HybridVector::ReadWrite(ref read_write_ref) => {
                read_write_ref
            },
            &HybridVector::ReadOnly(ref read_only_ref) => {
                read_only_ref
            },
        }
    }
    // returns a read-write reference
    fn read_write(&mut self) -> &mut Vec<u8> {
        match self {
            &mut HybridVector::Owner(ref mut read_write_ref) => {
                read_write_ref
            },
            &mut HybridVector::ReadWrite(ref mut read_write_ref) => {
                read_write_ref
            },
            _ => {
                panic!("must not happen");
            },
        }
    }
}

///////////////////////////
// implementation struct //
///////////////////////////
pub struct DU<'a> {
    buffer: HybridVector<'a>
}

// trait implementations
impl<'a> ops::Index<usize> for DU<'a> {
    type Output = u8;
    fn index(&self, pos: usize) -> &u8 {
        self.at(pos)
    }
}

impl<'a> ops::IndexMut<usize> for DU<'a> {
    fn index_mut(&mut self, pos: usize) -> &mut u8 {
        self.at_mut(pos)
    }
}

impl<'a> DUintf for DU<'a> {
    // returns a read-only reference
    fn buffer_read_only(&self) -> &Vec<u8> {
        self.buffer.read_only()
    }
    // returns a read-write reference 
    fn buffer_read_write(&mut self) -> &mut Vec<u8> {
        self.buffer.read_write()
    }
}

// methods implementation
impl<'a> DU<'a> {
    //////////////////
    // constructors //
    //////////////////

    // default constructor
    pub fn new() -> DU<'a> {
        DU {
            buffer: HybridVector::new()
        }
    }
    // copy constructor
    pub fn new_clone(value: &Vec<u8>) -> DU<'a> {
        DU {
            buffer: HybridVector::new_clone(value)
        }
    }
    // allocating constructor
    pub fn new_alloc(size: usize) -> DU<'a> {
        DU {
            buffer: HybridVector::new_alloc(size)
        }
    }
    // move ownership
    pub fn new_owner(value: Vec<u8>) -> DU<'a> {
        DU {
            buffer: HybridVector::new_owner(value)
        }
    }
    // wraps data for read-only
    pub fn new_read_only(reference: &Vec<u8>) -> DU {
        DU {
            buffer: HybridVector::new_read_only(reference)
        }
    }
    // wraps data for read-write
    pub fn new_read_write(reference: &mut Vec<u8>) -> DU {
        DU {
            buffer: HybridVector::new_read_write(reference)
        }
    }
}
