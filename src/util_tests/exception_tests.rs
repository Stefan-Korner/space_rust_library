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
// Utilities - Exception type - Unit tests                                    *
//*****************************************************************************
use util::exception;

fn function_ok() -> Result<(), exception::Exception> {
    Ok(())
}

fn function_with_exception() -> Result<(), exception::Exception> {
    Err(exception::raise("some exception"))
}

pub fn test() {
    function_ok().unwrap();
    let description = function_with_exception().unwrap_err();
    println!("--> description = {}", description);
}
