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
// Utilities - Exception type                                                 *
//                                                                            *
// Compatible with std::error::Error                                          *
//*****************************************************************************
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Exception {
    description: String
}

impl error::Error for Exception {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

pub fn raise(description: &str) -> Exception {
    Exception {
        description: String::from(description)
    }
}
