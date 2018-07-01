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

pub fn test() {
    let txt1 = "0123456789 hihihi huhuhu";
    let mut du1 = du::DU::new_owner(String::from(txt1).into_bytes());
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 30 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 0123456789 hihih
0010 69 20 68 75 68 75 68 75                         i huhuhu");
    println!("du1[0] = {}", du1[0]);
    assert_eq!(du1[0], 48);
    du1[0] = 64;
    println!("du1[0] = {}", du1[0]);
    assert_eq!(du1[0], 64);
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 40 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 @123456789 hihih
0010 69 20 68 75 68 75 68 75                         i huhuhu");
    du1.resize(0x20);
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 40 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 @123456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    let mut val = du1.get_unsigned_acc(UINT_ACC1).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 64);
    val = du1.get_unsigned_acc(UINT_ACC2).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 16433);
    val = du1.get_unsigned_acc(UINT_ACC3).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 4206898);
    val = du1.get_unsigned_acc(UINT_ACC4).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 1076965939);

    du1.set_unsigned_acc(UINT_ACC1, 1).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 01 31 32 33 34 35 36 37 38 39 20 68 69 68 69 68 .123456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_unsigned_acc(UINT_ACC2, 513).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 02 01 32 33 34 35 36 37 38 39 20 68 69 68 69 68 ..23456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_unsigned_acc(UINT_ACC3, 197121).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 03 02 01 33 34 35 36 37 38 39 20 68 69 68 69 68 ...3456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_unsigned_acc(UINT_ACC4, 67305985).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 04 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    let mut bval = du1.get_big_unsigned_acc(BUINT_ACC1).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 4);
    bval = du1.get_big_unsigned_acc(BUINT_ACC2).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 1027);
    bval = du1.get_big_unsigned_acc(BUINT_ACC3).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 262914);
    bval = du1.get_big_unsigned_acc(BUINT_ACC4).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 67305985);
    bval = du1.get_big_unsigned_acc(BUINT_ACC5).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 17230332212);
    bval = du1.get_big_unsigned_acc(BUINT_ACC6).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 4410965046325);
    bval = du1.get_big_unsigned_acc(BUINT_ACC7).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 1129207051859254);
    bval = du1.get_big_unsigned_acc(BUINT_ACC8).unwrap();
    println!("bval = {}", bval);
    assert_eq!(bval, 289077005275969079);

    du1.set_big_unsigned_acc(BUINT_ACC1, 1).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 01 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_big_unsigned_acc(BUINT_ACC2, 513).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 02 01 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_big_unsigned_acc(BUINT_ACC3, 197121).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 03 02 01 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_big_unsigned_acc(BUINT_ACC4, 67305985).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 04 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_big_unsigned_acc(BUINT_ACC5, 21542142465).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 05 04 03 02 01 35 36 37 38 39 20 68 69 68 69 68 .....56789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_big_unsigned_acc(BUINT_ACC6, 6618611909121).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 06 05 04 03 02 01 36 37 38 39 20 68 69 68 69 68 ......6789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_big_unsigned_acc(BUINT_ACC7, 1976943448883713).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 07 06 05 04 03 02 01 37 38 39 20 68 69 68 69 68 .......789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_big_unsigned_acc(BUINT_ACC8, 578437695752307201).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 08 07 06 05 04 03 02 01 38 39 20 68 69 68 69 68 ........89 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    // temporary force a proper contents in the buffer
    du1[0] = 0x04;
    du1[1] = 0x03;
    du1[2] = 0x02;
    du1[3] = 0x01;
    du1[4] = 0x34;
    du1[5] = 0x35;
    du1[6] = 0x36;
    du1[7] = 0x37;

    val = du1.get_bits_acc(BIT_ACC1).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 0);
    val = du1.get_bits_acc(BIT_ACC2).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 4);
    val = du1.get_bits_acc(BIT_ACC3).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 48);
    val = du1.get_bits_acc(BIT_ACC4).unwrap();
    println!("val = {}", val);
    assert_eq!(val, 8211);

    du1.set_bits_acc(BIT_ACC1, 1).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 0c 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_bits_acc(BIT_ACC2, 3).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 0b 03 02 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_bits_acc(BIT_ACC3, 171).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 0b 0a b2 01 34 35 36 37 38 39 20 68 69 68 69 68 ....456789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
    du1.set_bits_acc(BIT_ACC4, 39030).unwrap();
    du1.dump("du1");
    assert_eq!(du1.dump_str(), "
0000 0b 0a b9 87 64 35 36 37 38 39 20 68 69 68 69 68 ....d56789 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");

    let bytes = du1.get_bytes_acc(BYTE_ACC1).unwrap().to_vec();
    let du3 = du::DU::new_read_only(&bytes);
    du3.dump("du3");
    assert_eq!(du3.dump_str(), "
0000 35 36 37 38                                     5678");

    du1.set_bytes_acc(BYTE_ACC1, "ABCD".as_bytes()).unwrap();
    du1.dump("du2");
    assert_eq!(du1.dump_str(), "
0000 0b 0a b9 87 64 41 42 43 44 39 20 68 69 68 69 68 ....dABCD9 hihih
0010 69 20 68 75 68 75 68 75 00 00 00 00 00 00 00 00 i huhuhu........
0020 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ................
0030 00 00 00 00 00 00 00 00                         ........");
}
