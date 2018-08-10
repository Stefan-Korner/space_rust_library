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
use std::cmp;
use std::ops;
use util::exception;

///////////////
// constants //
///////////////

// index = [first_bit_in_byte_pos][last_bit_in_byte_pos]
const BIT_FILTER: [[u8; 8]; 8] = [
    [0x7f, 0x3f, 0x1f, 0x0f, 0x07, 0x03, 0x01, 0x00],
    [   0, 0xbf, 0x9f, 0x8f, 0x87, 0x83, 0x81, 0x80],
    [   0,    0, 0xdf, 0xcf, 0xc7, 0xc3, 0xc1, 0xc0],
    [   0,    0,    0, 0xef, 0xe7, 0xe3, 0xe1, 0xe0],
    [   0,    0,    0,    0, 0xf7, 0xf3, 0xf1, 0xf0],
    [   0,    0,    0,    0,    0, 0xfb, 0xf9, 0xf8],
    [   0,    0,    0,    0,    0,    0, 0xfd, 0xfc],
    [   0,    0,    0,    0,    0,    0,    0, 0xfe]
];

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
        pub const $acc_name: du::BitAccessor = du::BitAccessor {bit_pos: $bit_pos, bit_length: $bit_length};
    };
}

pub struct ByteAccessor {
    pub byte_pos: usize,
    pub byte_length: usize
}
#[macro_export]
macro_rules! def_byte_accessor {
    ($acc_name: ident, $byte_pos: expr, $byte_length: expr) => {
        pub const $acc_name: du::ByteAccessor = du::ByteAccessor {byte_pos: $byte_pos, byte_length: $byte_length};
    };
}

pub struct UnsignedAccessor {
    pub byte_pos: usize,
    pub byte_length: usize
}
#[macro_export]
macro_rules! def_unsigned_accessor {
    ($acc_name: ident, $byte_pos: expr, $byte_length: expr) => {
        pub const $acc_name: du::UnsignedAccessor = du::UnsignedAccessor {byte_pos: $byte_pos, byte_length: $byte_length};
    };
}

pub struct BigUnsignedAccessor {
    pub byte_pos: usize,
    pub byte_length: usize
}
#[macro_export]
macro_rules! def_big_unsigned_accessor {
    ($acc_name: ident, $byte_pos: expr, $byte_length: expr) => {
        pub const $acc_name: du::BigUnsignedAccessor = du::BigUnsignedAccessor {byte_pos: $byte_pos, byte_length: $byte_length};
    };
}

//#################
// DU...Data Unit #
//#################

/////////////////////
// interface trait //
/////////////////////
pub trait DUintf {
    // returns a read-only reference, must be implemented in struct
    fn buffer_read_only(&self) -> &[u8];
    // returns a read-write reference, must be implemented in struct 
    fn buffer_read_write(&mut self) -> &mut [u8];
    // change size, must be implemented in struct
    fn resize(&mut self, new_size: usize);

    ///////////////////////
    // general accessors //
    ///////////////////////

    // returns the size
    fn size(&self) -> usize {
        self.buffer_read_only().len()
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
    fn get_bits(&self, bit_pos: usize, bit_length: usize) ->
        Result<u32, exception::Exception> {
        // performance optimizations:
        // - divide by 8 is replaced by >> 3 (performance)
        // - modulo 8 is replaced by & 7 (performance)
        // consistency checks
        if (bit_length == 0) || (bit_length > 32) {
            return Err(exception::raise("invalid bit_length"));
        }
        let last_bit_pos = bit_pos + bit_length - 1;
        let last_byte_pos = last_bit_pos >> 3;
        if last_byte_pos >= self.size() {
            return Err(exception::raise("bit_pos/bit_length out of buffer"));
        }
        // accumulate the number starting with the first byte
        let mut byte_pos = bit_pos >> 3;
        let byte = self.buffer_read_only()[byte_pos];
        // first byte: filter the highest bits that do not belong to the value
        let first_bit_in_byte_pos = bit_pos & 7;
        let bit_filter = (1_u8 << (8 - first_bit_in_byte_pos)) - 1;
        let mut value = (byte & bit_filter) as u32;
        // next bytes...
        byte_pos += 1;
        while byte_pos <= last_byte_pos {
            let byte = self.buffer_read_only()[byte_pos];
            value = (value << 8) + (byte as u32);
            byte_pos += 1;
        }
        // last byte: remove the lowest bits that do not belong to the value
        let last_bit_in_byte_pos = last_bit_pos & 7;
        value >>= 7 - last_bit_in_byte_pos;
        Ok(value)
    }
    fn set_bits(&mut self, bit_pos: usize, bit_length: usize, value: u32) ->
        Result<(), exception::Exception> {
        // performance optimizations:
        // - divide by 8 is replaced by >> 3 (performance)
        // - modulo 8 is replaced by & 7 (performance)
        // consistency checks
        if (bit_length == 0) || (bit_length > 32) {
            return Err(exception::raise("invalid bit_length"));
        }
        let max_value = (1_u64 << bit_length) - 1;
        if (value as u64) > max_value {
            return Err(exception::raise("value out of range"));
        }
        let last_bit_pos = bit_pos + bit_length - 1;
        let last_byte_pos = last_bit_pos >> 3;
        if last_byte_pos >= self.size() {
            return Err(exception::raise("bit_pos/bit_length out of buffer"));
        }
        // set zero-bits in the buffer where the value aligns
        let first_byte_pos = bit_pos >> 3;
        let first_bit_in_byte_pos = bit_pos & 7;
        let last_bit_in_byte_pos = last_bit_pos & 7;
        let mut byte_pos = first_byte_pos;
        if first_byte_pos == last_byte_pos {
            self.buffer_read_write()[byte_pos] &= BIT_FILTER[first_bit_in_byte_pos][last_bit_in_byte_pos];
        }
        else
        {
            self.buffer_read_write()[byte_pos] &= BIT_FILTER[first_bit_in_byte_pos][7];
            byte_pos += 1;
            while byte_pos < last_byte_pos {
                self.buffer_read_write()[byte_pos] = 0;
                byte_pos += 1;
            }
            self.buffer_read_write()[byte_pos] &= BIT_FILTER[0][last_bit_in_byte_pos];
        }
        // fill value with trailing zero-bits to align with the position
        let mut aligned_value = value as u64;
        aligned_value <<= 7 - last_bit_in_byte_pos;
        // decompose the aligned_value and add it to the buffer
        // starting at byte_pos, which is at the last byte
        while byte_pos >= first_byte_pos {
            let byte = (aligned_value & 0xff) as u8;
            self.buffer_read_write()[byte_pos] += byte;
            aligned_value >>= 8;
            if byte_pos == 0 {
                break;
            }
            byte_pos -= 1;
        }
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

    // byte aligned access
    fn get_bytes(&self, byte_pos: usize, byte_length: usize) ->
        Result<&[u8], exception::Exception> {
        // consistency checks
        if byte_length == 0 {
            return Err(exception::raise("invalid byte_length"));
        }
        let end_pos = byte_pos + byte_length;
        if end_pos > self.size() {
            return Err(exception::raise("byte_pos/byte_length out of buffer"));
        }
        Ok(&self.buffer_read_only()[byte_pos..end_pos])
    }
    fn set_bytes(&mut self, byte_pos: usize, byte_length: usize, bytes: &[u8]) ->
        Result<(), exception::Exception> {
        // consistency checks
        if byte_length == 0 {
            return Err(exception::raise("invalid byte_length"));
        }
        if (byte_pos + byte_length) > self.size() {
            return Err(exception::raise("byte_pos/byte_length out of buffer"));
        }
        // copy the minimum of bytes defined by byte_length and bytes
        let num_bytes = cmp::min(byte_length, bytes.len());
        let mut src_byte_pos = 0;
        let mut dest_byte_pos = byte_pos;
        while src_byte_pos < num_bytes {
            self.buffer_read_write()[dest_byte_pos] = bytes[src_byte_pos];
            src_byte_pos += 1;
            dest_byte_pos += 1;
        }
        Ok(())
    }
    fn get_bytes_acc(&self, acc: ByteAccessor) ->
        Result<&[u8], exception::Exception> {
        self.get_bytes(acc.byte_pos, acc.byte_length)
    }
    fn set_bytes_acc(&mut self, acc: ByteAccessor, bytes: &[u8]) ->
        Result<(), exception::Exception> {
        self.set_bytes(acc.byte_pos, acc.byte_length, bytes)
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
            let byte = (next_value & 0xff) as u8;
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
            let byte = (next_value & 0xff) as u8;
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
pub enum HybridVector<'a> {
    Owner(Vec<u8>),
    ReadWrite(&'a mut [u8]),
    ReadOnly(&'a [u8]),
}
impl<'a> HybridVector<'a> {
    // default constructor
    pub fn new() -> HybridVector<'a> {
        HybridVector::Owner(Vec::new())
    }
    // copy constructor
    pub fn new_clone(value: &Vec<u8>) -> HybridVector<'a> {
        HybridVector::Owner(value.to_vec())
    }
    // allocating constructor
    pub fn new_alloc(size: usize) -> HybridVector<'a> {
        HybridVector::Owner(vec![0; size])
    }
    // move ownership
    pub fn new_owner(value: Vec<u8>) -> HybridVector<'a> {
        HybridVector::Owner(value)
    }
    // wraps data for read-only
    pub fn new_read_only(reference: &[u8]) -> HybridVector {
        HybridVector::ReadOnly(reference)
    }
    // wraps data for read-write
    pub fn new_read_write(reference: &mut [u8]) -> HybridVector {
        HybridVector::ReadWrite(reference)
    }
    // returns a read-only reference
    pub fn read_only(&self) -> &[u8] {
        match self {
            &HybridVector::Owner(ref read_write_ref) => {
                read_write_ref.as_slice()
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
    pub fn read_write(&mut self) -> &mut [u8] {
        match self {
            &mut HybridVector::Owner(ref mut read_write_ref) => {
                read_write_ref.as_mut_slice()
            },
            &mut HybridVector::ReadWrite(ref mut read_write_ref) => {
                read_write_ref
            },
            _ => {
                panic!("must not happen");
            },
        }
    }
    // returns a read-write reference
    pub fn mut_vec(&mut self) -> &mut Vec<u8> {
        match self {
            &mut HybridVector::Owner(ref mut read_write_ref) => {
                read_write_ref
            },
            _ => {
                panic!("must not happen");
            },
        }
    }
    // change size
    pub fn resize(&mut self, new_size: usize) {
        let old_size = self.read_only().len();
        if old_size < new_size {
            let additional = new_size - old_size;
            self.mut_vec().reserve(additional);
            let mut i = 0;
            while i < new_size {
                self.mut_vec().push(0);
                i = i + 1;
            }
        } else {
            unsafe {
                self.mut_vec().set_len(new_size);
            }
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
    pub fn new_read_only(reference: &[u8]) -> DU {
        DU {
            buffer: HybridVector::new_read_only(reference)
        }
    }
    // wraps data for read-write
    pub fn new_read_write(reference: &mut [u8]) -> DU {
        DU {
            buffer: HybridVector::new_read_write(reference)
        }
    }
}
