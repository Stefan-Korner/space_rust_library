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
// CCSDS TIME - Conversion                                                    *
//*****************************************************************************

//////////////////////
// global constants //
//////////////////////
pub const EPOCH_1958_SEC_DELTA: i32 = -378691200;
pub const CUCFINE3_TO_MICRO: f64 =  1000000_f64 / 16777216_f64;
pub const CUCFINE2_TO_MICRO: f64 =  1000000_f64 /    65536_f64;
pub const CUCFINE1_TO_MICRO: f64 =  1000000_f64 /      256_f64;
pub const MICRO_TO_CUCFINE3: f64 = 16777216_f64 /  1000000_f64;
pub const MICRO_TO_CUCFINE2: f64 =    65536_f64 /  1000000_f64;
pub const MICRO_TO_CUCFINE1: f64 =      256_f64 /  1000000_f64;

//////////////////////
// global variables //
//////////////////////
static mut EPOCH_DELTA_SECONDS: i32 = 0;

//////////////////////
// global functions //
//////////////////////

// set the agency defined mission timeline (epoch delta),
// - the value shall be negative for mission timelines before 01.01.1970
// + the value shall be positive for mission timelines after 01.01.1970
// this epoch delta is used or CUC L2 times (CUC L1 use timeline 1.1.1958)
pub fn set_epoch_delta(delta_seconds: i32) {
    unsafe {
        EPOCH_DELTA_SECONDS = delta_seconds;
    }
}

// returns the agency defined mission timeline (epoch delta)
pub fn get_epoch_delta() -> i32 {
    unsafe {
        EPOCH_DELTA_SECONDS
    }
}

/////////////////////////////////////////////
// CDS time: CCSDS day segmented time code //
/////////////////////////////////////////////
pub mod cds {
    // TODO: implement the details (see SpacePyLibrary on GITHUB)
}

///////////////////////////////////////////
// CUC time: CCSDS unsegmented time code //
///////////////////////////////////////////
pub mod cuc {

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

    ///////////////////////
    // struct definition //
    ///////////////////////
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
                p_field: 0,
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
    }

    ///////////////
    // functions //
    ///////////////
    // the data in the buffer must start with the P field,
    // time correlation with a mission timeline is performed
    // TODO: UTIL::AbsTime convert(const void* p_buffer) throw(UTIL::Exception);

    // the data in the buffer are without the P field,
    // time correlation with a mission timeline is performed
    // TODO: UTIL::AbsTime convert(const void* p_buffer, TimeCode p_pField)
    // TODO:  throw(UTIL::Exception);

    // the data in the buffer start with the P field,
    // time correlation with a mission timeline is performed
    // TODO: Time convert(const UTIL::AbsTime& p_time, TimeCode p_pField)
    // TODO:  throw(UTIL::Exception);
}
