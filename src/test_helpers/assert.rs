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
// Assert helpers for unit tests                                              *
//*****************************************************************************
use time;

pub fn dump_u8(val_name: &str, val: u8, expected: u8) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

pub fn dump_u16(val_name: &str, val: u16, expected: u16) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

pub fn dump_u32(val_name: &str, val: u32, expected: u32) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

pub fn dump_u64(val_name: &str, val: u64, expected: u64) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

pub fn dump_usize(val_name: &str, val: usize, expected: usize) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

pub fn dump_string(val_name: &str, val: &str, expected: &str) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

pub fn dump_timespec(
    val_name: &str,
    val: time::Timespec,
    expected_sec: i64,
    expected_nsec: i32) {
    println!("{} = {},{}", val_name, val.sec, val.nsec);
    assert_eq!(val.sec, expected_sec);
    assert_eq!(val.nsec, expected_nsec);
}
