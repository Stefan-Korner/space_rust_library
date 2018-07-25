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
use std::u32;
use util::crc;
use util::du;
use util::du::DUintf;
use util::exception;

///////////////
// constants //
///////////////
pub const TM_PACKET_TYPE: u32 = 0;
pub const TC_PACKET_TYPE: u32 = 1;
pub const VERSION_NUMBER: u32 = 0;
pub const SEGMENTATION_CONTINUATION: u32 = 0;
pub const SEGMENTATION_FIRST: u32 = 1;
pub const SEGMENTATION_LAST: u32 = 2;
pub const SEGMENTATION_NONE: u32 = 3;
pub const CRC_BYTE_SIZE: usize = 2;
pub const PRIMARY_HEADER_BYTE_SIZE: usize = 6;
pub const N_BYTE_SIZE: usize = 4;
pub const TM_N_BYTE_SIZE: usize = 4;
pub const TC_N_BYTE_SIZE: usize = 0;
pub mod primary_header {
    use util::du;
    def_bit_accessor!(VERSION_NUMBER,          0,  3);
    def_bit_accessor!(PACKET_TYPE,             3,  1);
    def_bit_accessor!(DATA_FIELD_HEADER_FLAG,  4,  1);
    def_bit_accessor!(APPLICATION_PROCESS_ID,  5, 11);
    // byte 2
    def_bit_accessor!(SEGMENTATION_FLAGS,     16,  2);
    def_bit_accessor!(SEQUENCE_CONTROL_COUNT, 18, 14);
    def_unsigned_accessor!(PACKET_LENGTH,      4,  2);
}

//########################
// Packet...CCSDS Packet #
//########################

/////////////////////
// interface trait //
/////////////////////
pub trait PacketIntf: du::DUintf {

    //////////////////////////////////////////
    // access methods (convenience methods) //
    //////////////////////////////////////////

    fn get_version_number_field(&self) ->
        Result<u32, exception::Exception> {
        self.get_bits_acc(primary_header::VERSION_NUMBER)
    }
    fn set_version_number_field(&mut self, value: u32) ->
        Result<(), exception::Exception> {
        self.set_bits_acc(primary_header::VERSION_NUMBER, value)
    }
    fn get_packet_type_field(&self) ->
        Result<u32, exception::Exception> {
        self.get_bits_acc(primary_header::PACKET_TYPE)
    }
    fn set_packet_type_field(&mut self, value: u32) ->
        Result<(), exception::Exception> {
        self.set_bits_acc(primary_header::PACKET_TYPE, value)
    }
    fn get_data_field_header_flag_field(&self) ->
        Result<u32, exception::Exception> {
        self.get_bits_acc(primary_header::DATA_FIELD_HEADER_FLAG)
    }
    fn set_data_field_header_flag_field(&mut self, value: u32) ->
        Result<(), exception::Exception> {
        self.set_bits_acc(primary_header::DATA_FIELD_HEADER_FLAG, value)
    }
    fn get_application_process_id_field(&self) ->
        Result<u32, exception::Exception> {
        self.get_bits_acc(primary_header::APPLICATION_PROCESS_ID)
    }
    fn set_application_process_id_field(&mut self, value: u32) ->
        Result<(), exception::Exception> {
        self.set_bits_acc(primary_header::APPLICATION_PROCESS_ID, value)
    }
    fn get_segmentation_flags_field(&self) ->
        Result<u32, exception::Exception> {
        self.get_bits_acc(primary_header::SEGMENTATION_FLAGS)
    }
    fn set_segmentation_flags_field(&mut self, value: u32) ->
        Result<(), exception::Exception> {
        self.set_bits_acc(primary_header::SEGMENTATION_FLAGS, value)
    }
    fn get_sequence_control_count_field(&self) ->
        Result<u32, exception::Exception> {
        self.get_bits_acc(primary_header::SEQUENCE_CONTROL_COUNT)
    }
    fn set_sequence_control_count_field(&mut self, value: u32) ->
        Result<(), exception::Exception> {
        self.set_bits_acc(primary_header::SEQUENCE_CONTROL_COUNT, value)
    }
    fn get_packet_length_field(&self) ->
        Result<u32, exception::Exception> {
        self.get_unsigned_acc(primary_header::PACKET_LENGTH)
    }
    fn set_packet_length_field(&mut self, value: u32) ->
        Result<(), exception::Exception> {
        self.set_unsigned_acc(primary_header::PACKET_LENGTH, value)
    }

    ///////////////////
    // other methods //
    ///////////////////

    // sets the packetLength according to the data unit's buffer size
    fn set_packet_length(&mut self) ->
        Result<(), exception::Exception> {
        if self.size() < (PRIMARY_HEADER_BYTE_SIZE + 1) {
            return Err(exception::raise("packet size is too small"));
        }
        let length_value = self.size() - PRIMARY_HEADER_BYTE_SIZE - 1;
        if length_value > (u32::MAX as usize) {
            return Err(exception::raise("packet size is too large"));
        }
        self.set_packet_length_field(length_value as u32)
    }
    // checks the packetLength according to the data unit's buffer size
    fn check_packet_length(&self) ->
        Result<(bool), exception::Exception> {
        Ok((self.get_packet_length_field()? as usize) ==
           (self.size() - PRIMARY_HEADER_BYTE_SIZE - 1))
    }
    // sets the checksum out of the binary data,
    // buffer and packetLength must be correctly initialised
    fn set_checksum(&mut self) ->
        Result<(), exception::Exception> {
        if !self.check_packet_length()? {
            return Err(exception::raise("inconsistent packet length"));
        }
        let crc_pos = self.size() - CRC_BYTE_SIZE;
        let crc = crc::calculate16(self.buffer_read_only(), crc_pos);
        self.set_unsigned(crc_pos, CRC_BYTE_SIZE, crc as u32)
    }
    // checks the checksum out of the binary data,
    // buffer and, packetLength must be correctly initialised
    fn check_checksum(&self) ->
        Result<(bool), exception::Exception> {
        if !self.check_packet_length()? {
            return Ok(false);
        }
        let crc_pos = self.size() - CRC_BYTE_SIZE;
        let crc = crc::calculate16(self.buffer_read_only(), crc_pos);
        Ok(self.get_unsigned(crc_pos, CRC_BYTE_SIZE)? == (crc as u32))
    }
}

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
            buffer: du::HybridVector::new_alloc(PRIMARY_HEADER_BYTE_SIZE + 1)
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

//####################################
// TMpacket...CCSDS Telemetry Packet #
//####################################

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
            buffer: du::HybridVector::new_alloc(PRIMARY_HEADER_BYTE_SIZE + 1)
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

//#####################################
// TCpacket...CCSDS Telecomand Packet #
//#####################################

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
            buffer: du::HybridVector::new_alloc(PRIMARY_HEADER_BYTE_SIZE + 1)
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
