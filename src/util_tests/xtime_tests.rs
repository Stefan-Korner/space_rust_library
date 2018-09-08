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

pub fn print_time(prefix: &str, timespec: time::Timespec) {
    let time_str = xtime::get_asd_time_str(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = xtime::get_asd_time_str_with_milli(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = xtime::get_asd_time_str_with_micro(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = xtime::get_asd_time_str_with_nano(timespec);
    println!("{} = {}", prefix, time_str);
}

pub fn test() {
    // 1980.006.00.00.00
    let gps_time = xtime::get_time(315964800, 0);
    print_time("gps time", gps_time);
    // 1970.001.00.00.00
    let zero_time = xtime::get_zero_time();
    print_time("zero time", zero_time);
    // depends on execution time
    let actual_time = xtime::get_actual_time();
    print_time("actual time", actual_time);
}
