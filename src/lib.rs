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
// Library configuration                                                      *
//*****************************************************************************
#[macro_use]
extern crate lazy_static;
extern crate time;
pub mod test_helpers;
#[macro_use]
pub mod util;
pub mod util_tests;
pub mod ccsds;
pub mod ccsds_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn test_util_asd_time() {
        ::util_tests::asd_time_tests::test();
    }

    #[test]
    fn test_util_config() {
        ::util_tests::config_tests::test();
    }

    #[test]
    fn test_util_crc() {
        ::util_tests::crc_tests::test();
    }

    #[test]
    fn test_util_du() {
        ::util_tests::du_tests::test();
    }

    #[test]
    fn test_util_exception() {
        ::util_tests::exception_tests::test();
    }

    #[test]
    fn test_util_tco() {
        ::util_tests::tco_tests::test();
    }

    #[test]
    fn test_ccsds_packet() {
        ::ccsds_tests::packet_tests::test();
    }
}
