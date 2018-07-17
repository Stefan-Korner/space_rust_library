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
// Utilities - Data Unit - Unit tests                                         *
//*****************************************************************************
use util::du;
use util::du::DUintf;

def_unsigned_accessor!(UINT_ACC1, 0, 1);
def_unsigned_accessor!(UINT_ACC2, 0, 2);
def_unsigned_accessor!(UINT_ACC3, 0, 3);
def_unsigned_accessor!(UINT_ACC4, 0, 4);

def_big_unsigned_accessor!(BUINT_ACC1, 0, 1);
def_big_unsigned_accessor!(BUINT_ACC2, 0, 2);
def_big_unsigned_accessor!(BUINT_ACC3, 0, 3);
def_big_unsigned_accessor!(BUINT_ACC4, 0, 4);
def_big_unsigned_accessor!(BUINT_ACC5, 0, 5);
def_big_unsigned_accessor!(BUINT_ACC6, 0, 6);
def_big_unsigned_accessor!(BUINT_ACC7, 0, 7);
def_big_unsigned_accessor!(BUINT_ACC8, 0, 8);

def_bit_accessor!(BIT_ACC1,  4,  1);
def_bit_accessor!(BIT_ACC2,  5,  3);
def_bit_accessor!(BIT_ACC3, 12,  8);
def_bit_accessor!(BIT_ACC4, 20, 16);

def_byte_accessor!(BYTE_ACC1, 5, 4);

fn assert_dump_u8(val_name: &str, val: u8, expected: u8) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

fn assert_dump_u32(val_name: &str, val: u32, expected: u32) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

fn assert_dump_u64(val_name: &str, val: u64, expected: u64) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

fn assert_dump_uszie(val_name: &str, val: usize, expected: usize) {
    println!("{} = {}", val_name, val);
    assert_eq!(val, expected);
}

fn assert_dump_du(val_name: &str, val: &du::DU, expected: &str) {
    println!("{} = {}", val_name, val.dump_str());
    assert_eq!(val.dump_str(), expected);
}

pub fn test() {
    let txt = "0123456789 hihihi huhuhu";

    let mut du1 = du::DU::new();
    assert_dump_du("du1", &du1, "EMPTY");
    du1.resize(10);
    assert_dump_du("du1", &du1, "
0000 00 00 00 00 00 00 00 00 00 00                   ..........");
    du1[0] = 0x30;
    du1[1] = 0x31;
    du1[2] = 0x32;
    du1[3] = 0x33;
    du1[4] = 0x34;
    du1[5] = 0x35;
    du1[6] = 0x36;
    du1[7] = 0x37;
    du1[8] = 0x38;
    du1[9] = 0x39;
    assert_dump_du("du1", &du1, "
0000 30 31 32 33 34 35 36 37 38 39                   0123456789");
    assert_dump_u8("du1[0]", du1[0], 0x30);
    assert_dump_u8("du1[1]", du1[1], 0x31);
    assert_dump_u8("du1[2]", du1[2], 0x32);
    assert_dump_u8("du1[3]", du1[3], 0x33);
    assert_dump_u8("du1[4]", du1[4], 0x34);
    assert_dump_u8("du1[5]", du1[5], 0x35);
    assert_dump_u8("du1[6]", du1[6], 0x36);
    assert_dump_u8("du1[7]", du1[7], 0x37);
    assert_dump_u8("du1[8]", du1[8], 0x38);
    assert_dump_u8("du1[9]", du1[9], 0x39);

    let vec2 = du1.buffer_read_only().to_vec();
    let mut du2 = du::DU::new_clone(&vec2);
    assert_dump_du("du2", &du2, "
0000 30 31 32 33 34 35 36 37 38 39                   0123456789");
    du2[0] = 0x41;
    du2[1] = 0x42;
    du2[2] = 0x43;
    du2[3] = 0x44;
    du2[4] = 0x45;
    du2[5] = 0x46;
    du2[6] = 0x47;
    du2[7] = 0x48;
    du2[8] = 0x49;
    du2[9] = 0x4a;
    assert_dump_du("du2", &du2, "
0000 41 42 43 44 45 46 47 48 49 4a                   ABCDEFGHIJ");
    assert_dump_uszie("vec2.len()", vec2.len(), 10);
    assert_dump_u8("vec2[0]", vec2[0], 0x30);
    assert_dump_u8("vec2[1]", vec2[1], 0x31);
    assert_dump_u8("vec2[2]", vec2[2], 0x32);
    assert_dump_u8("vec2[3]", vec2[3], 0x33);
    assert_dump_u8("vec2[4]", vec2[4], 0x34);
    assert_dump_u8("vec2[5]", vec2[5], 0x35);
    assert_dump_u8("vec2[6]", vec2[6], 0x36);
    assert_dump_u8("vec2[7]", vec2[7], 0x37);
    assert_dump_u8("vec2[8]", vec2[8], 0x38);
    assert_dump_u8("vec2[9]", vec2[9], 0x39);

    let du3 = du::DU::new_alloc(0);
    assert_dump_du("du3", &du3, "EMPTY");
    let mut du3 = du::DU::new_alloc(10);
    assert_dump_du("du3", &du3, "
0000 00 00 00 00 00 00 00 00 00 00                   ..........");
    du3[0] = 0x30;
    du3[1] = 0x31;
    du3[2] = 0x32;
    du3[3] = 0x33;
    du3[4] = 0x34;
    du3[5] = 0x35;
    du3[6] = 0x36;
    du3[7] = 0x37;
    du3[8] = 0x38;
    du3[9] = 0x39;
    assert_dump_du("du3", &du3, "
0000 30 31 32 33 34 35 36 37 38 39                   0123456789");
    assert_dump_u8("du3[0]", du3[0], 0x30);
    assert_dump_u8("du3[1]", du3[1], 0x31);
    assert_dump_u8("du3[2]", du3[2], 0x32);
    assert_dump_u8("du3[3]", du3[3], 0x33);
    assert_dump_u8("du3[4]", du3[4], 0x34);
    assert_dump_u8("du3[5]", du3[5], 0x35);
    assert_dump_u8("du3[6]", du3[6], 0x36);
    assert_dump_u8("du3[7]", du3[7], 0x37);
    assert_dump_u8("du3[8]", du3[8], 0x38);
    assert_dump_u8("du3[9]", du3[9], 0x39);

    let vec4 = vec2;
    let mut du4 = du::DU::new_owner(vec4);
    assert_dump_du("du4", &du4, "
0000 30 31 32 33 34 35 36 37 38 39                   0123456789");
    du4[0] = 0x41;
    du4[1] = 0x42;
    du4[2] = 0x43;
    du4[3] = 0x44;
    du4[4] = 0x45;
    du4[5] = 0x46;
    du4[6] = 0x47;
    du4[7] = 0x48;
    du4[8] = 0x49;
    du4[9] = 0x4a;
    assert_dump_du("du4", &du4, "
0000 41 42 43 44 45 46 47 48 49 4a                   ABCDEFGHIJ");

    let du5 = du::DU::new_read_only(txt.as_bytes());
    assert_dump_du("du5", &du5, "
0000 30 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 0123456789 hihih
0010 69 20 68 75 68 75 68 75                         i huhuhu");
    assert_dump_u8("du5[0]", du5[0], 0x30);
    assert_dump_u8("du5[1]", du5[1], 0x31);
    assert_dump_u8("du5[2]", du5[2], 0x32);
    assert_dump_u8("du5[3]", du5[3], 0x33);
    assert_dump_u8("du5[4]", du5[4], 0x34);
    assert_dump_u8("du5[5]", du5[5], 0x35);
    assert_dump_u8("du5[6]", du5[6], 0x36);
    assert_dump_u8("du5[7]", du5[7], 0x37);
    assert_dump_u8("du5[8]", du5[8], 0x38);
    assert_dump_u8("du5[9]", du5[9], 0x39);

    let mut array6: [u8; 10] = [0; 10];
    // sub-scope to avoid life cycle conflict (read-only + read-write)
    {
        let mut du6 = du::DU::new_read_write(&mut array6);
        assert_dump_du("du6", &du6, "
0000 00 00 00 00 00 00 00 00 00 00                   ..........");
        du6[0] = 0x41;
        du6[1] = 0x42;
        du6[2] = 0x43;
        du6[3] = 0x44;
        du6[4] = 0x45;
        du6[5] = 0x46;
        du6[6] = 0x47;
        du6[7] = 0x48;
        du6[8] = 0x49;
        du6[9] = 0x4a;
        assert_dump_du("du6", &du6, "
0000 41 42 43 44 45 46 47 48 49 4a                   ABCDEFGHIJ");
    }
    assert_dump_u8("array6[0]", array6[0], 0x41);
    assert_dump_u8("array6[1]", array6[1], 0x42);
    assert_dump_u8("array6[2]", array6[2], 0x43);
    assert_dump_u8("array6[3]", array6[3], 0x44);
    assert_dump_u8("array6[4]", array6[4], 0x45);
    assert_dump_u8("array6[5]", array6[5], 0x46);
    assert_dump_u8("array6[6]", array6[6], 0x47);
    assert_dump_u8("array6[7]", array6[7], 0x48);
    assert_dump_u8("array6[8]", array6[8], 0x49);
    assert_dump_u8("array6[9]", array6[9], 0x4a);

    let mut du9 = du::DU::new_owner(String::from(txt).into_bytes());
    assert_dump_du("du9", &du9, "
0000 30 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 0123456789 hihih
0010 69 20 68 75 68 75 68 75                         i huhuhu");
    assert_dump_u8("du9[0]", du9[0], 48);
    du9[0] = 64;
    assert_dump_u8("du9[0]", du9[0], 64);
    assert_dump_du("du9", &du9, "
0000 40 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 @123456789 hihih
0010 69 20 68 75 68 75 68 75                         i huhuhu");
    du9.resize(0x20);
    assert_dump_du("du9", &du9, "
0000 40 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 @123456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    let mut val = du9.get_unsigned_acc(UINT_ACC1).unwrap();
    assert_dump_u32("val", val, 64);
    val = du9.get_unsigned_acc(UINT_ACC2).unwrap();
    assert_dump_u32("val", val, 16433);
    val = du9.get_unsigned_acc(UINT_ACC3).unwrap();
    assert_dump_u32("val", val, 4206898);
    val = du9.get_unsigned_acc(UINT_ACC4).unwrap();
    assert_dump_u32("val", val, 1076965939);

    du9.set_unsigned_acc(UINT_ACC1, 1).unwrap();
    assert_dump_du("du9", &du9, "
0000 01 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 .123456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_unsigned_acc(UINT_ACC2, 513).unwrap();
    assert_dump_du("du9", &du9, "
0000 02 01 32 33 34 35 36 37 38 39 20 68 69 68 69 68 ..23456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_unsigned_acc(UINT_ACC3, 197121).unwrap();
    assert_dump_du("du9", &du9, "
0000 03 02 01 33 34 35 36 37 38 39 20 68 69 68 69 68 ...3456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_unsigned_acc(UINT_ACC4, 67305985).unwrap();
    assert_dump_du("du9", &du9, "
0000 04 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    let mut bval = du9.get_big_unsigned_acc(BUINT_ACC1).unwrap();
    assert_dump_u64("bval", bval, 4);
    bval = du9.get_big_unsigned_acc(BUINT_ACC2).unwrap();
    assert_dump_u64("bval", bval, 1027);
    bval = du9.get_big_unsigned_acc(BUINT_ACC3).unwrap();
    assert_dump_u64("bval", bval, 262914);
    bval = du9.get_big_unsigned_acc(BUINT_ACC4).unwrap();
    assert_dump_u64("bval", bval, 67305985);
    bval = du9.get_big_unsigned_acc(BUINT_ACC5).unwrap();
    assert_dump_u64("bval", bval, 17230332212);
    bval = du9.get_big_unsigned_acc(BUINT_ACC6).unwrap();
    assert_dump_u64("bval", bval, 4410965046325);
    bval = du9.get_big_unsigned_acc(BUINT_ACC7).unwrap();
    assert_dump_u64("bval", bval, 1129207051859254);
    bval = du9.get_big_unsigned_acc(BUINT_ACC8).unwrap();
    assert_dump_u64("bval", bval, 289077005275969079);

    du9.set_big_unsigned_acc(BUINT_ACC1, 1).unwrap();
    assert_dump_du("du9", &du9, "
0000 01 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_big_unsigned_acc(BUINT_ACC2, 513).unwrap();
    assert_dump_du("du9", &du9, "
0000 02 01 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_big_unsigned_acc(BUINT_ACC3, 197121).unwrap();
    assert_dump_du("du9", &du9, "
0000 03 02 01 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_big_unsigned_acc(BUINT_ACC4, 67305985).unwrap();
    assert_dump_du("du9", &du9, "
0000 04 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_big_unsigned_acc(BUINT_ACC5, 21542142465).unwrap();
    assert_dump_du("du9", &du9, "
0000 05 04 03 02 01 35 36 37 38 39 20 68 69 68 69 68 .....56789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_big_unsigned_acc(BUINT_ACC6, 6618611909121).unwrap();
    assert_dump_du("du9", &du9, "
0000 06 05 04 03 02 01 36 37 38 39 20 68 69 68 69 68 ......6789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_big_unsigned_acc(BUINT_ACC7, 1976943448883713).unwrap();
    assert_dump_du("du9", &du9, "
0000 07 06 05 04 03 02 01 37 38 39 20 68 69 68 69 68 .......789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_big_unsigned_acc(BUINT_ACC8, 578437695752307201).unwrap();
    assert_dump_du("du9", &du9, "
0000 08 07 06 05 04 03 02 01 38 39 20 68 69 68 69 68 ........89 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    // temporary force a proper contents in the buffer
    du9[0] = 0x04;
    du9[1] = 0x03;
    du9[2] = 0x02;
    du9[3] = 0x01;
    du9[4] = 0x34;
    du9[5] = 0x35;
    du9[6] = 0x36;
    du9[7] = 0x37;

    val = du9.get_bits_acc(BIT_ACC1).unwrap();
    assert_dump_u32("val", val, 0);
    val = du9.get_bits_acc(BIT_ACC2).unwrap();
    assert_dump_u32("val", val, 4);
    val = du9.get_bits_acc(BIT_ACC3).unwrap();
    assert_dump_u32("val", val, 48);
    val = du9.get_bits_acc(BIT_ACC4).unwrap();
    assert_dump_u32("val", val, 8211);

    du9.set_bits_acc(BIT_ACC1, 1).unwrap();
    assert_dump_du("du9", &du9, "
0000 0c 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_bits_acc(BIT_ACC2, 3).unwrap();
    assert_dump_du("du9", &du9, "
0000 0b 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_bits_acc(BIT_ACC3, 171).unwrap();
    assert_dump_du("du9", &du9, "
0000 0b 0a b2 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du9.set_bits_acc(BIT_ACC4, 39030).unwrap();
    assert_dump_du("du9", &du9, "
0000 0b 0a b9 87 64 35 36 37 38 39 20 68 69 68 69 68 ....d56789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    // sub-scope to avoid life cycle conflict (read-only + read-write)
    {
        let bytes = du9.get_bytes_acc(BYTE_ACC1).unwrap();
        let du8 = du::DU::new_read_only(&bytes);
        assert_dump_du("du8", &du8, "
0000 35 36 37 38                                     5678");
    }

    du9.set_bytes_acc(BYTE_ACC1, "ABCD".as_bytes()).unwrap();
    assert_dump_du("du9", &du9, "
0000 0b 0a b9 87 64 41 42 43 44 39 20 68 69 68 69 68 ....dABCD9 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
}
