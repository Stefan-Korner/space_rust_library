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
// Utilities - ASD Time Conversions - Unit tests                              *
//*****************************************************************************
use test_helpers::assert;
use time;
use util::asd_time;

///////////////
// functions //
///////////////

fn assert_dump_timespec(
    val_name: &str, 
    val: time::Timespec,
    expected_sec: i64,
    expected_nsec: i32) {
    println!("{} = {},{}", val_name, val.sec, val.nsec);
    assert_eq!(val.sec, expected_sec);
    assert_eq!(val.nsec, expected_nsec);
}

fn print_time(title: &str, prefix: &str, timespec: time::Timespec) {
    println!("----- {} -----", title);
    println!("{}: timespec.sec = {}", prefix, timespec.sec);
    println!("{}: timespec.nsec = {}", prefix, timespec.nsec);
    let time_str = asd_time::get_time_str(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = asd_time::get_time_str_with_milli(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = asd_time::get_time_str_with_micro(timespec);
    println!("{} = {}", prefix, time_str);
    let time_str = asd_time::get_time_str_with_nano(timespec);
    println!("{} = {}", prefix, time_str);
}

fn parse_time(title: &str, time_str: &str) {
    let time = asd_time::parse_time(time_str).unwrap();
    print_time(title, time_str, time);
}

fn test_format_ok(timespec: time::Timespec, expected_time_str: &str) {
    let time_str = asd_time::get_time_str(timespec);
    assert::dump_string("time string", &time_str, expected_time_str);
}

fn test_format_ok_with_milli(timespec: time::Timespec, expected_time_str: &str) {
    let time_str = asd_time::get_time_str_with_milli(timespec);
    assert::dump_string("time string", &time_str, expected_time_str);
}

fn test_format_ok_with_micro(timespec: time::Timespec, expected_time_str: &str) {
    let time_str = asd_time::get_time_str_with_micro(timespec);
    assert::dump_string("time string", &time_str, expected_time_str);
}

fn test_format_ok_with_nano(timespec: time::Timespec, expected_time_str: &str) {
    let time_str = asd_time::get_time_str_with_nano(timespec);
    assert::dump_string("time string", &time_str, expected_time_str);
}

fn test_parse_error(time_str: &str) {
    let error_message = asd_time::parse_time(time_str).
        expect_err("expected error did not happen");
    println!("expected: {} for {}", error_message, time_str);
}

fn test_parse_ok(time_str: &str, expected_sec: i64, expected_nsec: i32) {
    let time = asd_time::parse_time(time_str).unwrap();
    assert_dump_timespec(time_str, time, expected_sec, expected_nsec);
}

pub fn test() {
    let zero_time = time::Timespec::new(0, 0);
    print_time("1970.001.00.00.00 / 0", "zero time", zero_time);
    let gps_time = time::Timespec::new(315964800, 0);
    print_time("1980.006.00.00.00 / 315964800", "GPS time", gps_time);
    // depends on execution time
    let actual_time = time::get_time();
    print_time("actual time", "actual time", actual_time);
    // format time tests
    test_format_ok(zero_time, "1970.001.00.00.00");
    test_format_ok_with_milli(zero_time, "1970.001.00.00.00.000");
    test_format_ok_with_micro(zero_time, "1970.001.00.00.00.000000");
    test_format_ok_with_nano(zero_time, "1970.001.00.00.00.000000000");
    test_format_ok(gps_time, "1980.006.00.00.00");
    test_format_ok_with_milli(gps_time, "1980.006.00.00.00.000");
    test_format_ok_with_micro(gps_time, "1980.006.00.00.00.000000");
    test_format_ok_with_nano(gps_time, "1980.006.00.00.00.000000000");
    let mut test_time = gps_time;
    test_time.nsec = 123456789;
    test_format_ok(test_time, "1980.006.00.00.00");
    test_format_ok_with_milli(test_time, "1980.006.00.00.00.123");
    test_format_ok_with_micro(test_time, "1980.006.00.00.00.123456");
    test_format_ok_with_nano(test_time, "1980.006.00.00.00.123456789");
    // parse time tests
    test_parse_error("");
    test_parse_error("blablabla");
    test_parse_error("length_17_is_OK__");
    test_parse_error("1980.006.01.02.xx");
    test_parse_error("1980.006.01.02.00.x");
    test_parse_error("1980.006.01.02.00.0123456789");
    // zero time
    test_parse_ok("1970.001.00.00.00", 0, 0);
    // GPS time
    test_parse_ok("1980.006.00.00.00", 315964800, 0);
    // other times
    test_parse_ok("1980.006.01.02.03", 315968523, 0);
    test_parse_ok("1980.006.01.02.03.", 315968523, 0);
    test_parse_ok("1980.006.01.02.03.1", 315968523, 100000000);
    test_parse_ok("1980.006.01.02.03.12", 315968523, 120000000);
    test_parse_ok("1980.006.01.02.03.123", 315968523, 123000000);
    test_parse_ok("1980.006.01.02.03.1234", 315968523, 123400000);
    test_parse_ok("1980.006.01.02.03.12345", 315968523, 123450000);
    test_parse_ok("1980.006.01.02.03.123456", 315968523, 123456000);
    test_parse_ok("1980.006.01.02.03.1234567", 315968523, 123456700);
    test_parse_ok("1980.006.01.02.03.12345678", 315968523, 123456780);
    test_parse_ok("1980.006.01.02.03.123456789", 315968523, 123456789);
    // parse and print
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
