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
pub mod util;
pub mod util_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn test_util_du() {
        ::util_tests::du_tests::test();
    }

    #[test]
    fn test_util_exception() {
        ::util_tests::exception_tests::test();
    }
}