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
// CCSDS Stack - CCSDS Packet Module - Unit tests                             *
//*****************************************************************************
use ccsds::cuc_time;
use ccsds::c_packet;
use ccsds::c_packet::PacketIntf;
use ccsds_tests::cuc_time_tests;
use util::du::DUintf;

def_cuc_time_accessor!(CUC_TIME_ACC1, 16, cuc_time::L2_TIME_4_3);
def_cuc_time_accessor!(CUC_TIME_ACC_L1_4_0, 16, cuc_time::L1_TIME_4_0);
def_cuc_time_accessor!(CUC_TIME_ACC_L1_4_1, 16, cuc_time::L1_TIME_4_1);
def_cuc_time_accessor!(CUC_TIME_ACC_L1_4_2, 16, cuc_time::L1_TIME_4_2);
def_cuc_time_accessor!(CUC_TIME_ACC_L1_4_3, 16, cuc_time::L1_TIME_4_3);
def_cuc_time_accessor!(CUC_TIME_ACC_L2_4_0, 16, cuc_time::L2_TIME_4_0);
def_cuc_time_accessor!(CUC_TIME_ACC_L2_4_1, 16, cuc_time::L2_TIME_4_1);
def_cuc_time_accessor!(CUC_TIME_ACC_L2_4_2, 16, cuc_time::L2_TIME_4_2);
def_cuc_time_accessor!(CUC_TIME_ACC_L2_4_3, 16, cuc_time::L2_TIME_4_3);
def_cuc_time_accessor!(CUC_TIME_ACC_T1_4_0, 16, cuc_time::T1_TIME_4_0);
def_cuc_time_accessor!(CUC_TIME_ACC_T1_4_1, 16, cuc_time::T1_TIME_4_1);
def_cuc_time_accessor!(CUC_TIME_ACC_T1_4_2, 16, cuc_time::T1_TIME_4_2);
def_cuc_time_accessor!(CUC_TIME_ACC_T1_4_3, 16, cuc_time::T1_TIME_4_3);
def_cuc_time_accessor!(CUC_TIME_ACC_T2_4_0, 16, cuc_time::T2_TIME_4_0);
def_cuc_time_accessor!(CUC_TIME_ACC_T2_4_1, 16, cuc_time::T2_TIME_4_1);
def_cuc_time_accessor!(CUC_TIME_ACC_T2_4_2, 16, cuc_time::T2_TIME_4_2);
def_cuc_time_accessor!(CUC_TIME_ACC_T2_4_3, 16, cuc_time::T2_TIME_4_3);
def_cuc_time_accessor!(CUC_TIME_ACC_ERR1, 16, 0);
def_cuc_time_accessor!(CUC_TIME_ACC_ERR2, 25, cuc_time::L2_TIME_4_3);
def_cuc_time_accessor!(CUC_TIME_ACC_ERR3, 26, cuc_time::T2_TIME_4_3);
def_cuc_time_accessor!(CUC_TIME_ACC_ERR4, 16, cuc_time::L2_TIME_4_3);

pub fn assert_dump_packet(val_name: &str, val: &c_packet::Packet, expected: &str) {
    println!("{} = {}", val_name, val.dump_str());
    assert_eq!(val.dump_str(), expected);
}

pub fn assert_dump_tm_packet(val_name: &str, val: &c_packet::TMpacket, expected: &str) {
    println!("{} = {}", val_name, val.dump_str());
    assert_eq!(val.dump_str(), expected);
}

pub fn assert_dump_tc_packet(val_name: &str, val: &c_packet::TCpacket, expected: &str) {
    println!("{} = {}", val_name, val.dump_str());
    assert_eq!(val.dump_str(), expected);
}

fn test_cuc_time_set_error(
    acc: c_packet::CucTimeAccessor,
    cuc_time: cuc_time::Time) {
    let mut packet = c_packet::Packet::new_alloc(32);
    let error_message = packet.set_cuc_time_acc(acc, cuc_time).
        expect_err("expected error did not happen");
    println!("expected: {} for {}", error_message, cuc_time);
}

fn test_cuc_time_get_error(acc: c_packet::CucTimeAccessor) {
    let packet = c_packet::Packet::new_alloc(32);
    let error_message = packet.get_cuc_time_acc(acc).
        expect_err("expected error did not happen");
    println!("expected: {}", error_message);
}

fn test_cuc_time_set_get_ok(
    acc: c_packet::CucTimeAccessor,
    cuc_time: cuc_time::Time,
    expected_packet_str: &str) {
    let mut packet = c_packet::Packet::new_alloc(32);
    packet.set_cuc_time_acc(acc, cuc_time).unwrap();
    assert_dump_packet("packet", &packet, expected_packet_str);
    let cuc_time2 = packet.get_cuc_time_acc(acc).unwrap();
    cuc_time_tests::assert_dump_cuc_time("cuc_time2", cuc_time2, cuc_time);
}

pub fn test() {
    // some basic packet tests
    let packet = c_packet::Packet::new();
    assert_dump_packet("packet", &packet, "
0000 00 00 00 00 00 00 00                            .......");
    let tm_packet = c_packet::TMpacket::new();
    assert_dump_tm_packet("tm_packet", &tm_packet, "
0000 00 00 00 00 00 00 00                            .......");
    let tc_packet = c_packet::TCpacket::new();
    assert_dump_tc_packet("tc_packet", &tc_packet, "
0000 00 00 00 00 00 00 00                            .......");
    // test access of CUC time
    let mut packet = c_packet::Packet::new_alloc(32);
    assert_dump_packet("packet", &packet, "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................");
    let cuc_time = cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 5, 6, 7);
    cuc_time_tests::assert_dump_cuc_time("cuc_time", cuc_time,
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 5, 6, 7));
    packet.set_cuc_time_acc(CUC_TIME_ACC1, cuc_time).unwrap();
    assert_dump_packet("packet", &packet, "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 2f 01 02 03 04 05 06 07 00 00 00 00 00 00 00 00 /...............");
    let cuc_time = packet.get_cuc_time_acc(CUC_TIME_ACC1).unwrap();
    cuc_time_tests::assert_dump_cuc_time("cuc_time", cuc_time,
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 5, 6, 7));
    // test acces of all types of CUC time
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L1_4_0,
        cuc_time::Time::new_l1_time_4_0_init(1, 2, 3, 4), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 1c 01 02 03 04 00 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L1_4_1,
        cuc_time::Time::new_l1_time_4_1_init(1, 2, 3, 4, 5), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 1d 01 02 03 04 05 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L1_4_2,
        cuc_time::Time::new_l1_time_4_2_init(1, 2, 3, 4, 5, 6), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 1e 01 02 03 04 05 06 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L1_4_3,
        cuc_time::Time::new_l1_time_4_3_init(1, 2, 3, 4, 5, 6, 7), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 1f 01 02 03 04 05 06 07 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L2_4_0,
        cuc_time::Time::new_l2_time_4_0_init(1, 2, 3, 4), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 2c 01 02 03 04 00 00 00 00 00 00 00 00 00 00 00 ,...............");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L2_4_1,
        cuc_time::Time::new_l2_time_4_1_init(1, 2, 3, 4, 5), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 2d 01 02 03 04 05 00 00 00 00 00 00 00 00 00 00 -...............");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L2_4_2,
        cuc_time::Time::new_l2_time_4_2_init(1, 2, 3, 4, 5, 6), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 2e 01 02 03 04 05 06 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_L2_4_3,
        cuc_time::Time::new_l2_time_4_3_init(1, 2, 3, 4, 5, 6, 7), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 2f 01 02 03 04 05 06 07 00 00 00 00 00 00 00 00 /...............");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T1_4_0,
        cuc_time::Time::new_t1_time_4_0_init(1, 2, 3, 4), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 00 00 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T1_4_1,
        cuc_time::Time::new_t1_time_4_1_init(1, 2, 3, 4, 5), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 05 00 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T1_4_2,
        cuc_time::Time::new_t1_time_4_2_init(1, 2, 3, 4, 5, 6), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 05 06 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T1_4_3,
        cuc_time::Time::new_t1_time_4_3_init(1, 2, 3, 4, 5, 6, 7), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 05 06 07 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T2_4_0,
        cuc_time::Time::new_t2_time_4_0_init(1, 2, 3, 4), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 00 00 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T2_4_1,
        cuc_time::Time::new_t2_time_4_1_init(1, 2, 3, 4, 5), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 05 00 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T2_4_2,
        cuc_time::Time::new_t2_time_4_2_init(1, 2, 3, 4, 5, 6), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 05 06 00 00 00 00 00 00 00 00 00 00 ................");
    test_cuc_time_set_get_ok(
        CUC_TIME_ACC_T2_4_3,
        cuc_time::Time::new_t2_time_4_3_init(1, 2, 3, 4, 5, 6, 7), "
0000 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00 00 ................
0010 01 02 03 04 05 06 07 00 00 00 00 00 00 00 00 00 ................");
    // test CUC time access errors
    test_cuc_time_set_error(CUC_TIME_ACC_ERR1, cuc_time::Time::new_l2_time_4_3());
    test_cuc_time_set_error(CUC_TIME_ACC_ERR2, cuc_time::Time::new_l2_time_4_3());
    test_cuc_time_set_error(CUC_TIME_ACC_ERR3, cuc_time::Time::new_t2_time_4_3());
    test_cuc_time_get_error(CUC_TIME_ACC_ERR1);
    test_cuc_time_get_error(CUC_TIME_ACC_ERR2);
    test_cuc_time_get_error(CUC_TIME_ACC_ERR3);
    test_cuc_time_get_error(CUC_TIME_ACC_ERR4);
}
