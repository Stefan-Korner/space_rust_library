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
// CUC TIME - CCSDS unsegmented time code - Unit tests                        *
//*****************************************************************************
use ccsds::cuc_time;
use test_helpers::assert;
use time;

///////////////
// functions //
///////////////

pub fn assert_dump_cuc_time(
    val_name: &str,
    val: cuc_time::Time,
    expected: cuc_time::Time) {
    println!("{} = {}", val_name, val);
    assert_eq!(val.dump_str(), expected.dump_str());
}

fn test_new_from_timespec_error(p_field: u8, timespec: time::Timespec) {
    let error_message = cuc_time::Time::new_from_timespec(p_field, timespec).
        expect_err("expected error did not happen");
    println!("expected: {} for {},{}", error_message, timespec.sec, timespec.nsec);
}

fn test_new_from_timespec_ok(
    timespec: time::Timespec,
    expected: cuc_time::Time) {
    let p_field = expected.get_p_field();
    let cuc_time = cuc_time::Time::new_from_timespec(p_field, timespec).unwrap();
    assert_dump_cuc_time(
        &format!("({},{})", timespec.sec, timespec.nsec), cuc_time, expected);
}

fn test_to_timespec(
    cuc_time: cuc_time::Time,
    expected_timespec_sec: i64,
    expected_timespec_nsec: i32) {
    let timespec = cuc_time.to_timespec();
    assert::dump_timespec(
        "cuc time",
        timespec,
        expected_timespec_sec,
        expected_timespec_nsec);
}

pub fn test() {
    let xtime = cuc_time::Time::new_l2_time_4_3();
    println!("xtime = {}", xtime);
    test_new_from_timespec_error(cuc_time::L2_TIME_4_3, time::Timespec::new(0x100000000, 0));
    test_new_from_timespec_error(cuc_time::L2_TIME_4_3, time::Timespec::new(-1, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0, 0),
        cuc_time::Time::new_l2_time_4_3());
    test_new_from_timespec_ok(
        time::Timespec::new(1, 0),
        cuc_time::Time::new_l2_time_4_3_init(0, 0, 0, 1, 0, 0, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 0),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 0, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 500000000),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 128, 0, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 60),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 0, 1));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 600),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 0, 10));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 15259),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 1, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 152588),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 10, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 3906250),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 1, 0, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 39062500),
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 10, 0, 0));
    test_to_timespec(cuc_time::Time::new_l2_time_4_3(), 0, 0);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(0, 0, 0, 1, 0, 0, 0),
        1, 0);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 0, 0),
        0x01020304, 0);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 128, 0, 0),
        0x01020304, 500000000);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 0, 1),
        0x01020304, 59 /*60*/);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 0, 10),
        0x01020304, 596 /*600*/);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 1, 0),
        0x01020304, 15258 /*15259*/);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 0, 10, 0),
        0x01020304, 152587 /*152588*/);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 1, 0, 0),
        0x01020304, 3906250);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 10, 0, 0),
        0x01020304, 39062500);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_0_init(1, 2, 3, 4),
        0x01020304, 0);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_1_init(1, 2, 3, 4, 5),
        0x01020304, 19531250);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_2_init(1, 2, 3, 4, 5, 6),
        0x01020304, 19622802);
    test_to_timespec(
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 5, 6, 7),
        0x01020304, 19623219);
}
