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

fn assert_dump_cuc_time(
    val_name: &str,
    val: cuc_time::Time,
    expected_p_field: u8,
    expected_t_coarse0: u8,
    expected_t_coarse1: u8,
    expected_t_coarse2: u8,
    expected_t_coarse3: u8,
    expected_t_fine0: u8,
    expected_t_fine1: u8,
    expected_t_fine2: u8) {
    println!("{}.p_field = {}", val_name, val.p_field);
    println!("{}.t_coarse0 = {}", val_name, val.t_coarse0);
    println!("{}.t_coarse1 = {}", val_name, val.t_coarse1);
    println!("{}.t_coarse2 = {}", val_name, val.t_coarse2);
    println!("{}.t_coarse3 = {}", val_name, val.t_coarse3);
    println!("{}.t_fine0 = {}", val_name, val.t_fine0);
    println!("{}.t_fine1 = {}", val_name, val.t_fine1);
    println!("{}.t_fine2 = {}", val_name, val.t_fine2);
    assert_eq!(val.p_field, expected_p_field);
    assert_eq!(val.t_coarse0, expected_t_coarse0);
    assert_eq!(val.t_coarse1, expected_t_coarse1);
    assert_eq!(val.t_coarse2, expected_t_coarse2);
    assert_eq!(val.t_coarse3, expected_t_coarse3);
    assert_eq!(val.t_fine0, expected_t_fine0);
    assert_eq!(val.t_fine1, expected_t_fine1);
    assert_eq!(val.t_fine2, expected_t_fine2);
}

fn test_new_from_timespec_error(p_field: u8, timespec: time::Timespec) {
    let error_message = cuc_time::Time::new_from_timespec(p_field, timespec).
        expect_err("expected error did not happen");
    println!("expected: {} for {},{}", error_message, timespec.sec, timespec.nsec);
}

fn test_new_from_timespec_ok(
    timespec: time::Timespec,
    expected_p_field: u8,
    expected_t_coarse0: u8,
    expected_t_coarse1: u8,
    expected_t_coarse2: u8,
    expected_t_coarse3: u8,
    expected_t_fine0: u8,
    expected_t_fine1: u8,
    expected_t_fine2: u8) {
    let cuc_time = cuc_time::Time::new_from_timespec(expected_p_field, timespec).unwrap();
    assert_dump_cuc_time(
        &format!("({},{})", timespec.sec, timespec.nsec), 
        cuc_time,
        expected_p_field,
        expected_t_coarse0,
        expected_t_coarse1,
        expected_t_coarse2,
        expected_t_coarse3,
        expected_t_fine0,
        expected_t_fine1,
        expected_t_fine2);
}

fn test_to_timespec_error(cuc_time: cuc_time::Time) {
    let error_message = cuc_time.to_timespec().
        expect_err("expected error did not happen");
    println!("expected: {} for p_field {}", error_message, cuc_time.p_field);
}

fn test_to_timespec_ok(
    cuc_time: cuc_time::Time,
    expected_timespec_sec: i64,
    expected_timespec_nsec: i32) {
    let timespec = cuc_time.to_timespec().unwrap();
    assert::dump_timespec(
        "cuc time",
        timespec,
        expected_timespec_sec,
        expected_timespec_nsec);
}

pub fn test() {
    test_new_from_timespec_error(cuc_time::L2_TIME_4_3, time::Timespec::new(0x100000000, 0));
    test_new_from_timespec_error(cuc_time::L2_TIME_4_3, time::Timespec::new(-1, 0));
    test_new_from_timespec_ok(
        time::Timespec::new(0, 0),
        cuc_time::L2_TIME_4_3, 0, 0, 0, 0, 0, 0, 0);
    test_new_from_timespec_ok(
        time::Timespec::new(1, 0),
        cuc_time::L2_TIME_4_3, 0, 0, 0, 1, 0, 0, 0);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 0),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 0, 0);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 500000000),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 128, 0, 0);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 60),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 0, 1);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 600),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 0, 10);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 15259),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 1, 0);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 152588),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 10, 0);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 3906250),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 1, 0, 0);
    test_new_from_timespec_ok(
        time::Timespec::new(0x01020304, 39062500),
        cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 10, 0, 0);
    let error_time = cuc_time::Time::new_init(0, 0, 0, 0, 0, 0, 0, 0);
    test_to_timespec_error(error_time);
    let error_time = cuc_time::Time::new_init(255, 0, 0, 0, 0, 0, 0, 0);
    test_to_timespec_error(error_time);
    test_to_timespec_ok(cuc_time::Time::new(), 0, 0);

    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 0, 0, 0, 1, 0, 0, 0),
        1, 0);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 0, 0),
        0x01020304, 0);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 128, 0, 0),
        0x01020304, 500000000);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 0, 1),
        0x01020304, 59 /*60*/);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 0, 10),
        0x01020304, 596 /*600*/);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 1, 0),
        0x01020304, 15258 /*15259*/);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 0, 10, 0),
        0x01020304, 152587 /*152588*/);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 1, 0, 0),
        0x01020304, 3906250);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 10, 0, 0),
        0x01020304, 39062500);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_0, 1, 2, 3, 4, 5, 6, 7),
        0x01020304, 0);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_0, 1, 2, 3, 4, 0, 0, 0),
        0x01020304, 0);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_1, 1, 2, 3, 4, 5, 6, 7),
        0x01020304, 19531250);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_1, 1, 2, 3, 4, 5, 0, 0),
        0x01020304, 19531250);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_2, 1, 2, 3, 4, 5, 6, 7),
        0x01020304, 19622802);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_2, 1, 2, 3, 4, 5, 6, 0),
        0x01020304, 19622802);
    test_to_timespec_ok(
        cuc_time::Time::new_init(cuc_time::L2_TIME_4_3, 1, 2, 3, 4, 5, 6, 7),
        0x01020304, 19623219);
}
