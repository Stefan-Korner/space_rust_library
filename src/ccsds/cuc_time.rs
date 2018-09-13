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
// CUC TIME - CCSDS unsegmented time code                                     *
//                                                                            *
// Correlation to an epoch must be done explicitly via use of util::tco       *
//*****************************************************************************
use std::fmt;
use std::u32;
use time;
use util::exception;

///////////////
// constants //
///////////////
// supported time codes with embedded p-field
pub const L1_TIME_4_0: u8 = 0x1c; // epoch: 1.1.1958, 0 fine byte
pub const L1_TIME_4_1: u8 = 0x1d; // epoch: 1.1.1958, 1 fine byte
pub const L1_TIME_4_2: u8 = 0x1e; // epoch: 1.1.1958, 2 fine bytes
pub const L1_TIME_4_3: u8 = 0x1f; // epoch: 1.1.1958, 3 fine bytes
pub const L2_TIME_4_0: u8 = 0x2c; // epoch: agency-def., 0 fine b.
pub const L2_TIME_4_1: u8 = 0x2d; // epoch: agency-def., 1 fine b.
pub const L2_TIME_4_2: u8 = 0x2e; // epoch: agency-def., 2 fine b.
pub const L2_TIME_4_3: u8 = 0x2f; // epoch: agency-def., 3 fine b.
// supported time codes without embedded p-field
pub const T1_TIME_4_0: u8 = 0x9c; // epoch: 1.1.1958, 0 fine byte
pub const T1_TIME_4_1: u8 = 0x9d; // epoch: 1.1.1958, 1 fine byte
pub const T1_TIME_4_2: u8 = 0x9e; // epoch: 1.1.1958, 2 fine bytes
pub const T1_TIME_4_3: u8 = 0x9f; // epoch: 1.1.1958, 3 fine bytes
pub const T2_TIME_4_0: u8 = 0xac; // epoch: agency-def., 0 fine b.
pub const T2_TIME_4_1: u8 = 0xad; // epoch: agency-def., 1 fine b.
pub const T2_TIME_4_2: u8 = 0xae; // epoch: agency-def., 2 fine b.
pub const T2_TIME_4_3: u8 = 0xaf; // epoch: agency-def., 3 fine b.
// for internal use
const CUCFINE3_TO_NSEC: f64 = 1000000000_f64 /   16777216_f64;
const CUCFINE2_TO_NSEC: f64 = 1000000000_f64 /      65536_f64;
const CUCFINE1_TO_NSEC: f64 = 1000000000_f64 /        256_f64;
const NSEC_TO_CUCFINE3: f64 =   16777216_f64 / 1000000000_f64;
const NSEC_TO_CUCFINE2: f64 =      65536_f64 / 1000000000_f64;
const NSEC_TO_CUCFINE1: f64 =        256_f64 / 1000000000_f64;

//////////////////
// helper types //
//////////////////

// coarse time (seconds) with 4 bytes
#[derive(Copy, Clone, Debug)]
pub struct CoarseTime {
    pub t_coarse0: u8,
    pub t_coarse1: u8,
    pub t_coarse2: u8,
    pub t_coarse3: u8,
}
impl CoarseTime {
    // default constructor
    pub fn new() -> CoarseTime {
        CoarseTime {
            t_coarse0: 0,
            t_coarse1: 0,
            t_coarse2: 0,
            t_coarse3: 0,
        }
    }
    // init constructor
    pub fn new_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8) -> CoarseTime {
        CoarseTime {
            t_coarse0,
            t_coarse1,
            t_coarse2,
            t_coarse3,
        }
    }
    // initialization from seconds
    pub fn init_from_sec(&mut self, sec: i64) ->
        Result<(), exception::Exception> {
        if sec < 0 {
            return Err(exception::raise("CUC time supports only positive seconds"));
        }
        if sec > (u32::MAX as i64) {
            return Err(exception::raise(
                &format!("CUC time supports positive seconds up to {}", u32::MAX)));
        }
        // convert coarse time
        self.t_coarse3 = (sec & 0xFF) as u8;
        let sec = sec >> 8;
        self.t_coarse2 = (sec & 0xFF) as u8;
        let sec = sec >> 8;
        self.t_coarse1 = (sec & 0xFF) as u8;
        let sec = sec >> 8;
        self.t_coarse0 = (sec & 0xFF) as u8;
        Ok(())
    }
    // conversion to seconds
    pub fn to_sec(&self) -> i64 {
        ((self.t_coarse0 as i64) * 0x01000000_i64) +
        ((self.t_coarse1 as i64) * 0x00010000_i64) +
        ((self.t_coarse2 as i64) * 0x00000100_i64) +
        ((self.t_coarse3 as i64) * 0x00000001_i64)
    }
    // initialization from bytes
    pub fn init_from_bytes(&mut self, bytes: &[u8]) {
        self.t_coarse0 = bytes[0];
        self.t_coarse1 = bytes[1];
        self.t_coarse2 = bytes[2];
        self.t_coarse3 = bytes[3];
    }
    // update the contents to bytes
    pub fn update_to_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = self.t_coarse0;
        bytes[1] = self.t_coarse1;
        bytes[2] = self.t_coarse2;
        bytes[3] = self.t_coarse3;
    }
    // dumps the contents to a string
    pub fn dump_str(&self) -> String {
        format!("({},{},{},{})", self.t_coarse0, self.t_coarse1, self.t_coarse2, self.t_coarse3)
    }
}

// fine time (second fractions) with 0 bytes: used for consistency
#[derive(Copy, Clone, Debug)]
pub struct FineTime0 {
}
impl FineTime0 {
    // default constructor
    pub fn new() -> FineTime0 {
        FineTime0 {}
    }
    // init constructor
    pub fn new_init() -> FineTime0 {
        FineTime0 {}
    }
    // initialization from nano seconds
    pub fn init_from_nsec(&mut self, nsec: i32) ->
        Result<(), exception::Exception> {
        if nsec < 0 {
            return Err(exception::raise("CUC time supports only positive second fractions"));
        }
        // convert fine time --> nothing to do
        Ok(())
    }
    // conversion to nano seconds
    pub fn to_nsec(&self) -> i32 {0}
    // initialization from bytes
    pub fn init_from_bytes(&mut self, _bytes: &[u8]) {}
    // update the contents to bytes
    pub fn update_to_bytes(&self, _bytes: &mut [u8]) {
    }
    // dumps the contents to a string
    pub fn dump_str(&self) -> String {
        format!("()")
    }
}

// fine time (second fractions) with 1 byte
#[derive(Copy, Clone, Debug)]
pub struct FineTime1 {
    pub t_fine0: u8,
}
impl FineTime1 {
    // default constructor
    pub fn new() -> FineTime1 {
        FineTime1 {
            t_fine0: 0,
        }
    }
    // init constructor
    pub fn new_init(
        t_fine0: u8) -> FineTime1 {
        FineTime1 {
            t_fine0,
        }
    }
    // initialization from nano seconds
    pub fn init_from_nsec(&mut self, nsec: i32) ->
        Result<(), exception::Exception> {
        if nsec < 0 {
            return Err(exception::raise("CUC time supports only positive second fractions"));
        }
        // convert fine time
        let cuc_time_fine = ((nsec as f64) * NSEC_TO_CUCFINE1) as u64;
        self.t_fine0 = (cuc_time_fine & 0xFF) as u8;
        Ok(())
    }
    // conversion to nano seconds
    pub fn to_nsec(&self) -> i32 {
        let cuc_time_fine = (self.t_fine0 as u64) * 0x000001_u64;
        ((cuc_time_fine as f64) * CUCFINE1_TO_NSEC) as i32
    }
    // initialization from bytes
    pub fn init_from_bytes(&mut self, bytes: &[u8]) {
        self.t_fine0 = bytes[0];
    }
    // update the contents to bytes
    pub fn update_to_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = self.t_fine0;
    }
    // dumps the contents to a string
    pub fn dump_str(&self) -> String {
        format!("({})", self.t_fine0)
    }
}

// fine time (second fractions) with 2 bytes
#[derive(Copy, Clone, Debug)]
pub struct FineTime2 {
    pub t_fine0: u8,
    pub t_fine1: u8,
}
impl FineTime2 {
    // default constructor
    pub fn new() -> FineTime2 {
        FineTime2 {
            t_fine0: 0,
            t_fine1: 0,
        }
    }
    // init constructor
    pub fn new_init(
        t_fine0: u8,
        t_fine1: u8) -> FineTime2 {
        FineTime2 {
            t_fine0,
            t_fine1,
        }
    }
    // initialization from nano seconds
    pub fn init_from_nsec(&mut self, nsec: i32) ->
        Result<(), exception::Exception> {
        if nsec < 0 {
            return Err(exception::raise("CUC time supports only positive second fractions"));
        }
        // convert fine time
        let cuc_time_fine = ((nsec as f64) * NSEC_TO_CUCFINE2) as u64;
        self.t_fine1 = (cuc_time_fine & 0xFF) as u8;
        let cuc_time_fine = cuc_time_fine >> 8;
        self.t_fine0 = (cuc_time_fine & 0xFF) as u8;
        Ok(())
    }
    // conversion to nano seconds
    pub fn to_nsec(&self) -> i32 {
        let cuc_time_fine = ((self.t_fine0 as u64) * 0x000100_u64) +
                            ((self.t_fine1 as u64) * 0x000001_u64);
        ((cuc_time_fine as f64) * CUCFINE2_TO_NSEC) as i32
    }
    // initialization from bytes
    pub fn init_from_bytes(&mut self, bytes: &[u8]) {
        self.t_fine0 = bytes[0];
        self.t_fine1 = bytes[1];
    }
    // update the contents to bytes
    pub fn update_to_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = self.t_fine0;
        bytes[1] = self.t_fine1;
    }
    // dumps the contents to a string
    pub fn dump_str(&self) -> String {
        format!("({},{})", self.t_fine0, self.t_fine1)
    }
}

// fine time (second fractions) with 3 bytes
#[derive(Copy, Clone, Debug)]
pub struct FineTime3 {
    pub t_fine0: u8,
    pub t_fine1: u8,
    pub t_fine2: u8,
}
impl FineTime3 {
    // default constructor
    pub fn new() -> FineTime3 {
        FineTime3 {
            t_fine0: 0,
            t_fine1: 0,
            t_fine2: 0,
        }
    }
    // init constructor
    pub fn new_init(
        t_fine0: u8,
        t_fine1: u8,
        t_fine2: u8) -> FineTime3 {
        FineTime3 {
            t_fine0,
            t_fine1,
            t_fine2,
        }
    }
    // initialization from nano seconds
    pub fn init_from_nsec(&mut self, nsec: i32) ->
        Result<(), exception::Exception> {
        if nsec < 0 {
            return Err(exception::raise("CUC time supports only positive second fractions"));
        }
        // convert fine time
        let cuc_time_fine = ((nsec as f64) * NSEC_TO_CUCFINE3) as u64;
        self.t_fine2 = (cuc_time_fine & 0xFF) as u8;
        let cuc_time_fine = cuc_time_fine >> 8;
        self.t_fine1 = (cuc_time_fine & 0xFF) as u8;
        let cuc_time_fine = cuc_time_fine >> 8;
        self.t_fine0 = (cuc_time_fine & 0xFF) as u8;
        Ok(())
    }
    // conversion to nano seconds
    pub fn to_nsec(&self) -> i32 {
        let cuc_time_fine = ((self.t_fine0 as u64) * 0x010000_u64) +
                            ((self.t_fine1 as u64) * 0x000100_u64) +
                            ((self.t_fine2 as u64) * 0x000001_u64);
        ((cuc_time_fine as f64) * CUCFINE3_TO_NSEC) as i32
    }
    // initialization from bytes
    pub fn init_from_bytes(&mut self, bytes: &[u8]) {
        self.t_fine0 = bytes[0];
        self.t_fine1 = bytes[1];
        self.t_fine2 = bytes[2];
    }
    // update the contents to bytes
    pub fn update_to_bytes(&self, bytes: &mut [u8]) {
        bytes[0] = self.t_fine0;
        bytes[1] = self.t_fine1;
        bytes[2] = self.t_fine2;
    }
    // dumps the contents to a string
    pub fn dump_str(&self) -> String {
        format!("({},{},{})", self.t_fine0, self.t_fine1, self.t_fine2)
    }
}

//////////////////////
// helper functions //
//////////////////////

// tells if a time format has an explicit p-field
pub fn has_p_field(p_field: u8) -> bool {
    match p_field {
        L1_TIME_4_0 |
        L1_TIME_4_1 |
        L1_TIME_4_2 |
        L1_TIME_4_3 |
        L2_TIME_4_0 |
        L2_TIME_4_1 |
        L2_TIME_4_2 |
        L2_TIME_4_3 => true,
        _ => false,
    }
}
// netto data size without embedded p-field
pub fn get_data_size(p_field: u8) ->
    Result<usize, exception::Exception> {
    match p_field {
        L1_TIME_4_0 => Ok(4),
        L1_TIME_4_1 => Ok(5),
        L1_TIME_4_2 => Ok(6),
        L1_TIME_4_3 => Ok(7),
        L2_TIME_4_0 => Ok(4),
        L2_TIME_4_1 => Ok(5),
        L2_TIME_4_2 => Ok(6),
        L2_TIME_4_3 => Ok(7),
        T1_TIME_4_0 => Ok(4),
        T1_TIME_4_1 => Ok(5),
        T1_TIME_4_2 => Ok(6),
        T1_TIME_4_3 => Ok(7),
        T2_TIME_4_0 => Ok(4),
        T2_TIME_4_1 => Ok(5),
        T2_TIME_4_2 => Ok(6),
        T2_TIME_4_3 => Ok(7),
        _ => Err(exception::raise("invalid p-field for CUC time")),
    }
}
// brutto data size with embedded p-field (if embedded p-field)
pub fn get_full_data_size(p_field: u8) ->
    Result<usize, exception::Exception> {
    match p_field {
        L1_TIME_4_0 => Ok(5),
        L1_TIME_4_1 => Ok(6),
        L1_TIME_4_2 => Ok(7),
        L1_TIME_4_3 => Ok(8),
        L2_TIME_4_0 => Ok(5),
        L2_TIME_4_1 => Ok(6),
        L2_TIME_4_2 => Ok(7),
        L2_TIME_4_3 => Ok(8),
        T1_TIME_4_0 => Ok(4),
        T1_TIME_4_1 => Ok(5),
        T1_TIME_4_2 => Ok(6),
        T1_TIME_4_3 => Ok(7),
        T2_TIME_4_0 => Ok(4),
        T2_TIME_4_1 => Ok(5),
        T2_TIME_4_2 => Ok(6),
        T2_TIME_4_3 => Ok(7),
        _ => Err(exception::raise("invalid p-field for CUC time")),
    }
}



/////////////////////
// enum definition //
/////////////////////
#[derive(Copy, Clone, Debug)]
pub enum Time {
    // supported time codes with embedded p-field
    L1Time40 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime0,
    },
    L1Time41 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime1,
    },
    L1Time42 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime2,
    },
    L1Time43 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime3,
    },
    L2Time40 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime0,
    },
    L2Time41 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime1,
    },
    L2Time42 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime2,
    },
    L2Time43 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime3,
    },
    // supported time codes without embedded p-field
    T1Time40 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime0,
    },
    T1Time41 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime1,
    },
    T1Time42 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime2,
    },
    T1Time43 { // epoch: 1.1.1958
        coarse_time: CoarseTime,
        fine_time: FineTime3,
    },
    T2Time40 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime0,
    },
    T2Time41 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime1,
    },
    T2Time42 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime2,
    },
    T2Time43 { // epoch: agency-defined
        coarse_time: CoarseTime,
        fine_time: FineTime3,
    },
}

////////////////////////////
// methods implementation //
////////////////////////////
impl Time {
    // empty initialization constructors
    pub fn new_l1_time_4_0() -> Time {
        Time::L1Time40 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime0::new(),
        }
    }
    pub fn new_l1_time_4_1() -> Time {
        Time::L1Time41 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime1::new(),
        }
    }
    pub fn new_l1_time_4_2() -> Time {
        Time::L1Time42 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime2::new(),
        }
    }
    pub fn new_l1_time_4_3() -> Time {
        Time::L1Time43 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime3::new(),
        }
    }
    pub fn new_l2_time_4_0() -> Time {
        Time::L2Time40 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime0::new(),
        }
    }
    pub fn new_l2_time_4_1() -> Time {
        Time::L2Time41 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime1::new(),
        }
    }
    pub fn new_l2_time_4_2() -> Time {
        Time::L2Time42 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime2::new(),
        }
    }
    pub fn new_l2_time_4_3() -> Time {
        Time::L2Time43 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime3::new(),
        }
    }
    pub fn new_t1_time_4_0() -> Time {
        Time::T1Time40 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime0::new(),
        }
    }
    pub fn new_t1_time_4_1() -> Time {
        Time::T1Time41 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime1::new(),
        }
    }
    pub fn new_t1_time_4_2() -> Time {
        Time::T1Time42 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime2::new(),
        }
    }
    pub fn new_t1_time_4_3() -> Time {
        Time::T1Time43 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime3::new(),
        }
    }
    pub fn new_t2_time_4_0() -> Time {
        Time::T2Time40 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime0::new(),
        }
    }
    pub fn new_t2_time_4_1() -> Time {
        Time::T2Time41 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime1::new(),
        }
    }
    pub fn new_t2_time_4_2() -> Time {
        Time::T2Time42 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime2::new(),
        }
    }
    pub fn new_t2_time_4_3() -> Time {
        Time::T2Time43 {
            coarse_time: CoarseTime::new(),
            fine_time: FineTime3::new(),
        }
    }
    // init constructors
    pub fn new_l1_time_4_0_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8) -> Time {
        Time::L1Time40 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime0::new_init(),
        }
    }
    pub fn new_l1_time_4_1_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8) -> Time {
        Time::L1Time41 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime1::new_init(
                t_fine0),
        }
    }
    pub fn new_l1_time_4_2_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8) -> Time {
        Time::L1Time42 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime2::new_init(
                t_fine0,
                t_fine1),
        }
    }
    pub fn new_l1_time_4_3_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8,
        t_fine2: u8) -> Time {
        Time::L1Time43 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime3::new_init(
                t_fine0,
                t_fine1,
                t_fine2),
        }
    }
    pub fn new_l2_time_4_0_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8) -> Time {
        Time::L2Time40 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime0::new_init(),
        }
    }
    pub fn new_l2_time_4_1_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8) -> Time {
        Time::L2Time41 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime1::new_init(
                t_fine0),
        }
    }
    pub fn new_l2_time_4_2_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8) -> Time {
        Time::L2Time42 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime2::new_init(
                t_fine0,
                t_fine1),
        }
    }
    pub fn new_l2_time_4_3_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8,
        t_fine2: u8) -> Time {
        Time::L2Time43 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime3::new_init(
                t_fine0,
                t_fine1,
                t_fine2),
        }
    }
    pub fn new_t1_time_4_0_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8) -> Time {
        Time::T1Time40 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime0::new_init(),
        }
    }
    pub fn new_t1_time_4_1_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8) -> Time {
        Time::T1Time41 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime1::new_init(
                t_fine0),
        }
    }
    pub fn new_t1_time_4_2_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8) -> Time {
        Time::T1Time42 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime2::new_init(
                t_fine0,
                t_fine1),
        }
    }
    pub fn new_t1_time_4_3_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8,
        t_fine2: u8) -> Time {
        Time::T1Time43 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime3::new_init(
                t_fine0,
                t_fine1,
                t_fine2),
        }
    }
    pub fn new_t2_time_4_0_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8) -> Time {
        Time::T2Time40 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime0::new_init(),
        }
    }
    pub fn new_t2_time_4_1_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8) -> Time {
        Time::T2Time41 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime1::new_init(
                t_fine0),
        }
    }
    pub fn new_t2_time_4_2_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8) -> Time {
        Time::T2Time42 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime2::new_init(
                t_fine0,
                t_fine1),
        }
    }
    pub fn new_t2_time_4_3_init(
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8,
        t_fine2: u8) -> Time {
        Time::T2Time43 {
            coarse_time: CoarseTime::new_init(
                t_coarse0,
                t_coarse1,
                t_coarse2,
                t_coarse3),
            fine_time: FineTime3::new_init(
                t_fine0,
                t_fine1,
                t_fine2),
        }
    }
    // initialization from p_field
    pub fn new_from_p_field(p_field: u8) ->
        Result<Time, exception::Exception> {
        match p_field {
            L1_TIME_4_0 => Ok(Time::new_l1_time_4_0()),
            L1_TIME_4_1 => Ok(Time::new_l1_time_4_1()),
            L1_TIME_4_2 => Ok(Time::new_l1_time_4_2()),
            L1_TIME_4_3 => Ok(Time::new_l1_time_4_3()),
            L2_TIME_4_0 => Ok(Time::new_l2_time_4_0()),
            L2_TIME_4_1 => Ok(Time::new_l2_time_4_1()),
            L2_TIME_4_2 => Ok(Time::new_l2_time_4_2()),
            L2_TIME_4_3 => Ok(Time::new_l2_time_4_3()),
            T1_TIME_4_0 => Ok(Time::new_t1_time_4_0()),
            T1_TIME_4_1 => Ok(Time::new_t1_time_4_1()),
            T1_TIME_4_2 => Ok(Time::new_t1_time_4_2()),
            T1_TIME_4_3 => Ok(Time::new_t1_time_4_3()),
            T2_TIME_4_0 => Ok(Time::new_t2_time_4_0()),
            T2_TIME_4_1 => Ok(Time::new_t2_time_4_1()),
            T2_TIME_4_2 => Ok(Time::new_t2_time_4_2()),
            T2_TIME_4_3 => Ok(Time::new_t2_time_4_3()),
            _ => Err(exception::raise("invalid p-field for CUC time creation")),
        }
    }
    // initialization from p_field and timespec
    pub fn new_from_timespec(p_field: u8, timespec: time::Timespec) ->
        Result<Time, exception::Exception> {
        let mut time = match Time::new_from_p_field(p_field) {
            Err(err) => return Err(err),
            Ok(time) => time,
        };
        if let Err(err) = time.init_from_timespec(timespec) {
            return Err(err);
        };
        Ok(time)
    }
    // accessor to p_field
    pub fn get_p_field(self) -> u8 {
        match self {
            Time::L1Time40 {..} => L1_TIME_4_0,
            Time::L1Time41 {..} => L1_TIME_4_1,
            Time::L1Time42 {..} => L1_TIME_4_2,
            Time::L1Time43 {..} => L1_TIME_4_3,
            Time::L2Time40 {..} => L2_TIME_4_0,
            Time::L2Time41 {..} => L2_TIME_4_1,
            Time::L2Time42 {..} => L2_TIME_4_2,
            Time::L2Time43 {..} => L2_TIME_4_3,
            Time::T1Time40 {..} => T1_TIME_4_0,
            Time::T1Time41 {..} => T1_TIME_4_1,
            Time::T1Time42 {..} => T1_TIME_4_2,
            Time::T1Time43 {..} => T1_TIME_4_3,
            Time::T2Time40 {..} => T2_TIME_4_0,
            Time::T2Time41 {..} => T2_TIME_4_1,
            Time::T2Time42 {..} => T2_TIME_4_2,
            Time::T2Time43 {..} => T2_TIME_4_3,
        }
    }
    // initialization from timespec
    pub fn init_from_timespec(&mut self, timespec: time::Timespec) ->
        Result<(), exception::Exception> {
        // init coarse_time
        let sec = timespec.sec;
        if let Err(err) = match self {
            Time::L1Time40 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::L1Time41 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::L1Time42 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::L1Time43 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::L2Time40 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::L2Time41 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::L2Time42 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::L2Time43 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T1Time40 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T1Time41 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T1Time42 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T1Time43 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T2Time40 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T2Time41 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T2Time42 {coarse_time, ..} => coarse_time.init_from_sec(sec),
            Time::T2Time43 {coarse_time, ..} => coarse_time.init_from_sec(sec),
        } {
            return Err(err);
        };
        // init fine time
        let nsec = timespec.nsec;
        match self {
            Time::L1Time40 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::L1Time41 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::L1Time42 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::L1Time43 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::L2Time40 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::L2Time41 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::L2Time42 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::L2Time43 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T1Time40 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T1Time41 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T1Time42 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T1Time43 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T2Time40 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T2Time41 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T2Time42 {fine_time, ..} => fine_time.init_from_nsec(nsec),
            Time::T2Time43 {fine_time, ..} => fine_time.init_from_nsec(nsec),
        }
    }
    // conversion to timespec
    pub fn to_timespec(&self) -> time::Timespec {
        // convert seconds
        let sec = match self {
            Time::L1Time40 {coarse_time, ..} => coarse_time.to_sec(),
            Time::L1Time41 {coarse_time, ..} => coarse_time.to_sec(),
            Time::L1Time42 {coarse_time, ..} => coarse_time.to_sec(),
            Time::L1Time43 {coarse_time, ..} => coarse_time.to_sec(),
            Time::L2Time40 {coarse_time, ..} => coarse_time.to_sec(),
            Time::L2Time41 {coarse_time, ..} => coarse_time.to_sec(),
            Time::L2Time42 {coarse_time, ..} => coarse_time.to_sec(),
            Time::L2Time43 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T1Time40 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T1Time41 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T1Time42 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T1Time43 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T2Time40 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T2Time41 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T2Time42 {coarse_time, ..} => coarse_time.to_sec(),
            Time::T2Time43 {coarse_time, ..} => coarse_time.to_sec(),
        };
        // convert nano seconds
        let nsec = match self {
            Time::L1Time40 {fine_time, ..} => fine_time.to_nsec(),
            Time::L1Time41 {fine_time, ..} => fine_time.to_nsec(),
            Time::L1Time42 {fine_time, ..} => fine_time.to_nsec(),
            Time::L1Time43 {fine_time, ..} => fine_time.to_nsec(),
            Time::L2Time40 {fine_time, ..} => fine_time.to_nsec(),
            Time::L2Time41 {fine_time, ..} => fine_time.to_nsec(),
            Time::L2Time42 {fine_time, ..} => fine_time.to_nsec(),
            Time::L2Time43 {fine_time, ..} => fine_time.to_nsec(),
            Time::T1Time40 {fine_time, ..} => fine_time.to_nsec(),
            Time::T1Time41 {fine_time, ..} => fine_time.to_nsec(),
            Time::T1Time42 {fine_time, ..} => fine_time.to_nsec(),
            Time::T1Time43 {fine_time, ..} => fine_time.to_nsec(),
            Time::T2Time40 {fine_time, ..} => fine_time.to_nsec(),
            Time::T2Time41 {fine_time, ..} => fine_time.to_nsec(),
            Time::T2Time42 {fine_time, ..} => fine_time.to_nsec(),
            Time::T2Time43 {fine_time, ..} => fine_time.to_nsec(),
        };
        time::Timespec::new(sec, nsec)
    }
    // initialization from bytes, skips p-field (if embedded p-field)
    pub fn init_from_bytes(&mut self, bytes: &[u8]) {
        match self {
            Time::L1Time40 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);
            },
            Time::L1Time41 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);
            },
            Time::L1Time42 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);

            },
            Time::L1Time43 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);
            },
            Time::L2Time40 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);
            },
            Time::L2Time41 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);
            },
            Time::L2Time42 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);
            },
            Time::L2Time43 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes[1..]);
                fine_time.init_from_bytes(&bytes[5..]);
            },
            Time::T1Time40 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
            Time::T1Time41 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
            Time::T1Time42 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
            Time::T1Time43 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
            Time::T2Time40 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
            Time::T2Time41 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
            Time::T2Time42 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
            Time::T2Time43 {coarse_time, fine_time} => {
                coarse_time.init_from_bytes(&bytes);
                fine_time.init_from_bytes(&bytes[4..]);
            },
        };
    }
    // update the contents to bytes incl. p-field (if embedded p-field)
    pub fn update_to_bytes(&self, bytes: &mut [u8]) {
        match self {
            Time::L1Time40 {coarse_time, fine_time} => {
                bytes[0] = L1_TIME_4_0;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::L1Time41 {coarse_time, fine_time} => {
                bytes[0] = L1_TIME_4_1;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::L1Time42 {coarse_time, fine_time} => {
                bytes[0] = L1_TIME_4_2;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::L1Time43 {coarse_time, fine_time} => {
                bytes[0] = L1_TIME_4_3;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::L2Time40 {coarse_time, fine_time} => {
                bytes[0] = L2_TIME_4_0;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::L2Time41 {coarse_time, fine_time} => {
                bytes[0] = L2_TIME_4_1;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::L2Time42 {coarse_time, fine_time} => {
                bytes[0] = L2_TIME_4_2;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::L2Time43 {coarse_time, fine_time} => {
                bytes[0] = L2_TIME_4_3;
                coarse_time.update_to_bytes(&mut bytes[1..]);
                fine_time.update_to_bytes(&mut bytes[5..]);
            },
            Time::T1Time40 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
            Time::T1Time41 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
            Time::T1Time42 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
            Time::T1Time43 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
            Time::T2Time40 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
            Time::T2Time41 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
            Time::T2Time42 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
            Time::T2Time43 {coarse_time, fine_time} => {
                coarse_time.update_to_bytes(bytes);
                fine_time.update_to_bytes(&mut bytes[4..]);
            },
        };
    }
    // dumps the contents to a string
    pub fn dump_str(&self) -> String {
        match self {
            Time::L1Time40 {coarse_time, fine_time} => {
                format!("L1Time40({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::L1Time41 {coarse_time, fine_time} => {
                format!("L1Time41({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::L1Time42 {coarse_time, fine_time} => {
                format!("L1Time42({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::L1Time43 {coarse_time, fine_time} => {
                format!("L1Time43({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::L2Time40 {coarse_time, fine_time} => {
                format!("L2Time40({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::L2Time41 {coarse_time, fine_time} => {
                format!("L2Time41({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::L2Time42 {coarse_time, fine_time} => {
                format!("L2Time42({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::L2Time43 {coarse_time, fine_time} => {
                format!("L2Time43({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T1Time40 {coarse_time, fine_time} => {
                format!("T1Time40({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T1Time41 {coarse_time, fine_time} => {
                format!("T1Time41({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T1Time42 {coarse_time, fine_time} => {
                format!("T1Time42({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T1Time43 {coarse_time, fine_time} => {
                format!("T1Time43({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T2Time40 {coarse_time, fine_time} => {
                format!("T2Time40({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T2Time41 {coarse_time, fine_time} => {
                format!("T2Time41({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T2Time42 {coarse_time, fine_time} => {
                format!("T2Time42({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
            Time::T2Time43 {coarse_time, fine_time} => {
                format!("T2Time43({},{})", coarse_time.dump_str(), fine_time.dump_str())
            },
        }
    }
}
// trait implementation
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dump_str())
    }
}
