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
// Executable configuration                                                   *
//*****************************************************************************
extern crate space_rust_library;

fn main() {
    space_rust_library::util_tests::asd_time_tests::test();
    space_rust_library::util_tests::config_tests::test();
    space_rust_library::util_tests::crc_tests::test();
    space_rust_library::util_tests::du_tests::test();
    space_rust_library::util_tests::exception_tests::test();
    space_rust_library::util_tests::tco_tests::test();
    space_rust_library::ccsds_tests::packet_tests::test();
}
