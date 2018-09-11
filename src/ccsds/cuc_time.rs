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

///////////////////////
// struct definition //
///////////////////////
#[derive(Debug)]
pub struct Time {
    pub p_field: u8,
    pub t_coarse0: u8,
    pub t_coarse1: u8,
    pub t_coarse2: u8,
    pub t_coarse3: u8,
    pub t_fine0: u8,
    pub t_fine1: u8,
    pub t_fine2: u8
}

////////////////////////////
// methods implementation //
////////////////////////////
impl Time {
    // default constructor
    pub fn new() -> Time {
        Time {
            p_field: L2_TIME_4_0,
            t_coarse0: 0,
            t_coarse1: 0,
            t_coarse2: 0,
            t_coarse3: 0,
            t_fine0: 0,
            t_fine1: 0,
            t_fine2: 0
        }
    }
    // initialization constructor
    pub fn new_init(
        p_field: u8,
        t_coarse0: u8,
        t_coarse1: u8,
        t_coarse2: u8,
        t_coarse3: u8,
        t_fine0: u8,
        t_fine1: u8,
        t_fine2: u8) -> Time {
        Time {
            p_field,
            t_coarse0,
            t_coarse1,
            t_coarse2,
            t_coarse3,
            t_fine0,
            t_fine1,
            t_fine2
        }
    }
    // initialization from timespec
    pub fn new_from_timespec(p_field: u8, timespec: time::Timespec) ->
        Result<Time, exception::Exception> {
        let mut sec = timespec.sec;
        if sec < 0 {
            return Err(exception::raise("CUC time supports only positive seconds"));
        }
        if sec > (u32::MAX as i64) {
            return Err(exception::raise(
                &format!("CUC time supports positive seconds up to {}", u32::MAX)));
        }
        if timespec.nsec < 0 {
            return Err(exception::raise("CUC time supports only positive second fractions"));
        }
        // convert fine time
        let mut t_fine0 = 0_u8;
        let mut t_fine1 = 0_u8;
        let mut t_fine2 = 0_u8;
        if (p_field == L1_TIME_4_3) || (p_field == L2_TIME_4_3) {
            let nsec = timespec.nsec as f64;
            let mut cuc_time_fine = (nsec * NSEC_TO_CUCFINE3) as u64;
            t_fine2 = (cuc_time_fine & 0xFF) as u8;
            cuc_time_fine >>= 8;
            t_fine1 = (cuc_time_fine & 0xFF) as u8;
            cuc_time_fine >>= 8;
            t_fine0 = (cuc_time_fine & 0xFF) as u8;
        } else if (p_field == L1_TIME_4_2) || (p_field == L2_TIME_4_2) {
            let nsec = timespec.nsec as f64;
            let mut cuc_time_fine = (nsec * NSEC_TO_CUCFINE2) as u64;
            t_fine1 = (cuc_time_fine & 0xFF) as u8;
            cuc_time_fine >>= 8;
            t_fine0 = (cuc_time_fine & 0xFF) as u8;
        } else if (p_field == L1_TIME_4_1) || (p_field == L2_TIME_4_1) {
            let nsec = timespec.nsec as f64;
            let cuc_time_fine = (nsec * NSEC_TO_CUCFINE1) as u64;
            t_fine0 = (cuc_time_fine & 0xFF) as u8;
        } else if (p_field != L1_TIME_4_0) && (p_field != L2_TIME_4_0) {
            return Err(exception::raise("invalid P field for CUC time creation"));
        }
        // convert coarse time
        let t_coarse3 = (sec & 0xFF) as u8;
        sec >>= 8;
        let t_coarse2 = (sec & 0xFF) as u8;
        sec >>= 8;
        let t_coarse1 = (sec & 0xFF) as u8;
        sec >>= 8;
        let t_coarse0 = (sec & 0xFF) as u8;
        // create CUC time
        Ok(Time {
            p_field,
            t_coarse0,
            t_coarse1,
            t_coarse2,
            t_coarse3,
            t_fine0,
            t_fine1,
            t_fine2
        })
    }
    // conversion to timespec
    pub fn to_timespec(&self) ->
        Result<time::Timespec, exception::Exception> {
        // convert fine time
        let mut timespec_nsec = 0_i32;
        if (self.p_field == L1_TIME_4_3) || (self.p_field == L2_TIME_4_3) {
            let cuc_time_fine = ((self.t_fine0 as u64) * 0x010000_u64) +
                                ((self.t_fine1 as u64) * 0x000100_u64) +
                                ((self.t_fine2 as u64) * 0x000001_u64);
            let nsec = (cuc_time_fine as f64) * CUCFINE3_TO_NSEC;
            timespec_nsec = nsec as i32;
        } else if (self.p_field == L1_TIME_4_2) || (self.p_field == L2_TIME_4_2) {
            let cuc_time_fine = ((self.t_fine0 as u64) * 0x000100_u64) +
                                ((self.t_fine1 as u64) * 0x000001_u64);
            let nsec = (cuc_time_fine as f64) * CUCFINE2_TO_NSEC;
            timespec_nsec = nsec as i32;
        } else if (self.p_field == L1_TIME_4_1) || (self.p_field == L2_TIME_4_1) {
            let cuc_time_fine = (self.t_fine0 as u64) * 0x000001_u64;
            let nsec = (cuc_time_fine as f64) * CUCFINE1_TO_NSEC;
            timespec_nsec = nsec as i32;
        } else if (self.p_field != L1_TIME_4_0) && (self.p_field != L2_TIME_4_0) {
            return Err(exception::raise("invalid P field for Timespec creation"));
        }
        // convert coarse time
        let sec = ((self.t_coarse0 as i64) * 0x01000000_i64) +
                  ((self.t_coarse1 as i64) * 0x00010000_i64) +
                  ((self.t_coarse2 as i64) * 0x00000100_i64) +
                  ((self.t_coarse3 as i64) * 0x00000001_i64);
        Ok(time::Timespec::new(sec, timespec_nsec))
    }
}
