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
// Utilities - Basic Time Conversions                                         *
//                                                                            *
// Note: To avoid name clashes with std::time and Crate time, this module is  *
//       named xtime instead of time.                                         *
//*****************************************************************************

use time;

///////////////
// functions //
///////////////

// returns the UNIX zero time
pub fn get_time(sec: i64, nsec: i32) -> time::Timespec {
    time::Timespec::new(sec, nsec)
}

// returns the UNIX zero time
pub fn get_zero_time() -> time::Timespec {
    time::Timespec::new(0, 0)
}

// returns the actual time
pub fn get_actual_time() -> time::Timespec {
    time::get_time()
}

// returns the ASD format YYYY.DDD.hh.mm.ss
pub fn get_asd_time_str(timespec: time::Timespec) -> String {
    let tm = time::at_utc(timespec);
    format!(
        "{:04}.{:03}.{:02}.{:02}.{:02}",
        tm.tm_year + 1900,
        tm.tm_yday + 1,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec)
}

// returns the ASD format YYYY.DDD.hh.mm.ss.MMM
pub fn get_asd_time_str_with_milli(timespec: time::Timespec) -> String {
    let tm = time::at_utc(timespec);
    format!(
        "{:04}.{:03}.{:02}.{:02}.{:02}.{:03}",
        tm.tm_year + 1900,
        tm.tm_yday + 1,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        tm.tm_nsec / 1000000)
}

// returns the ASD format YYYY.DDD.hh.mm.ss.MMMMMM
pub fn get_asd_time_str_with_micro(timespec: time::Timespec) -> String {
    let tm = time::at_utc(timespec);
    format!(
        "{:04}.{:03}.{:02}.{:02}.{:02}.{:06}",
        tm.tm_year + 1900,
        tm.tm_yday + 1,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        tm.tm_nsec / 1000)
}

// returns the ASD format YYYY.DDD.hh.mm.ss.NNNNNNNNN
pub fn get_asd_time_str_with_nano(timespec: time::Timespec) -> String {
    let tm = time::at_utc(timespec);
    format!(
        "{:04}.{:03}.{:02}.{:02}.{:02}.{:09}",
        tm.tm_year + 1900,
        tm.tm_yday + 1,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        tm.tm_nsec)
}
