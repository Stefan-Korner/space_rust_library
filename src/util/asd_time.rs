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
// Utilities - ASD Time Conversions                                           *
//                                                                            *
// ASD time format: YYYY.DDD.hh.mm.ss ............. no seconds fraction       *
//                  YYYY.DDD.hh.mm.ss.MMM ......... milli seconds fraction    *
//                  YYYY.DDD.hh.mm.ss.MMMMMM ...... micro seconds fraction    *
//                  YYYY.DDD.hh.mm.ss.NNNNNNNNN ... nano seconds fraction     *
//                  ^    ^   ^  ^  ^                                          *
//                  |    |   |  |  +--------------- seconds 0...59            *
//                  |    |   |  +------------------ minutes 0...59            *
//                  |    |   +--------------------- hours   0...23            *
//                  |    +------------------------- days in year 1...365/366  *
//                  +------------------------------ years                     *
// Conversions are from/to time::Timespec (from Crate time)                   *
//*****************************************************************************
use time;
use util::exception;

///////////////
// functions //
///////////////

// returns the ASD format YYYY.DDD.hh.mm.ss
pub fn get_time_str(timespec: time::Timespec) -> String {
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
pub fn get_time_str_with_milli(timespec: time::Timespec) -> String {
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
pub fn get_time_str_with_micro(timespec: time::Timespec) -> String {
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
pub fn get_time_str_with_nano(timespec: time::Timespec) -> String {
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

// extracts a timespec from an ASD formated string
pub fn parse_time(time_str: &str) ->
    Result<time::Timespec, exception::Exception> {
    let time_str_len = time_str.len();
    if time_str_len < 17 {
        return Err(exception::raise(&format!("parse error: invalid string length {}", time_str_len)));
    }
    let seconds_part = &time_str[..17];
    let seconds_fraction = &time_str[17..];
    let nsec = match seconds_fraction.len() {
        0 => 0_i32,
        1 => 0_i32,
        2 => {
            let parse_result = seconds_fraction[1..2].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 100000000
        },
        3 => {
            let parse_result = seconds_fraction[1..3].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 10000000
        },
        4 => {
            let parse_result = seconds_fraction[1..4].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 1000000
        },
        5 => {
            let parse_result = seconds_fraction[1..5].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 100000
        },
        6 => {
            let parse_result = seconds_fraction[1..6].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 10000
        },
        7 => {
            let parse_result = seconds_fraction[1..7].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 1000
        },
        8 => {
            let parse_result = seconds_fraction[1..8].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 100
        },
        9 => {
            let parse_result = seconds_fraction[1..9].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap() * 10
        },
        10 => {
            let parse_result = seconds_fraction[1..10].parse::<i32>();
            if parse_result.is_err() {
                return Err(exception::raise("parse error in seconds fraction"));
             }
             parse_result.unwrap()
        },
        _ => {
             return Err(exception::raise("parse error in seconds fraction"));
        },
    };
    let parse_result = time::strptime(seconds_part, "%Y.%j.%H.%M.%S");
    if parse_result.is_err() {
        return Err(exception::raise("parse error in seconds part"));
    }
    let mut tm = parse_result.unwrap();
    // mday and mon are 0 after the parse -->
    // set it to 1st January
    tm.tm_mday = 1;
    // consider nano-seconds
    tm.tm_nsec = nsec;
    let mut timespec = tm.to_timespec();
    // to_timespec() does not consider the yday --> do this now
    let yday_compensation = tm.tm_yday * (24 * 60 * 60);
    timespec.sec += yday_compensation as i64;
    Ok(timespec)
}
