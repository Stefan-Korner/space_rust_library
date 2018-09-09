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
// Utilities - Basic Time Conversions - Unit tests                            *
//*****************************************************************************

use time;
use util::xtime;

///////////////
// functions //
///////////////

pub fn print_time(title: &str, prefix: &str, timespec: time::Timespec) {
    println!("----- {} -----", title);
    println!("{}: timespec.sec = {}", prefix, timespec.sec);
    println!("{}: timespec.nsec = {}", prefix, timespec.nsec);
    let time_str = xtime::get_asd_time_str(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = xtime::get_asd_time_str_with_milli(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = xtime::get_asd_time_str_with_micro(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = xtime::get_asd_time_str_with_nano(timespec);
    println!("{} = {}", prefix, time_str);
}

pub fn parse_time(title: &str, asd_time: &str) {
    let time = xtime::parse_asd_time(asd_time).unwrap();
    print_time(title, asd_time, time);
}

pub fn test() {
    let zero_time = xtime::get_zero_time();
    print_time("1970.001.00.00.00 / 0", "zero time", zero_time);
    let gps_time = xtime::get_time(315964800, 0);
    print_time("1980.006.00.00.00 / 315964800", "GPS time", gps_time);
    // depends on execution time
    let actual_time = xtime::get_actual_time();
    print_time("actual time", "actual time", actual_time);
    // parse ASD time
    parse_time("zero time 1970.001.00.00.00 / 0", "1970.001.00.00.00");
    parse_time("GPS time: 1980.006.00.00.00 / 315964800", "1980.006.00.00.00");
    parse_time("1980.006.01.02.03", "1980.006.01.02.03");
    parse_time("1980.006.01.02.03.", "1980.006.01.02.03.");
    parse_time("1980.006.01.02.03.1", "1980.006.01.02.03.1");
    parse_time("1980.006.01.02.03.12", "1980.006.01.02.03.12");
    parse_time("1980.006.01.02.03.123", "1980.006.01.02.03.123");
    parse_time("1980.006.01.02.03.1234", "1980.006.01.02.03.1234");
    parse_time("1980.006.01.02.03.12345", "1980.006.01.02.03.12345");
    parse_time("1980.006.01.02.03.123456", "1980.006.01.02.03.123456");
    parse_time("1980.006.01.02.03.1234567", "1980.006.01.02.03.1234567");
    parse_time("1980.006.01.02.03.12345678", "1980.006.01.02.03.12345678");
    parse_time("1980.006.01.02.03.123456789", "1980.006.01.02.03.123456789");
}
