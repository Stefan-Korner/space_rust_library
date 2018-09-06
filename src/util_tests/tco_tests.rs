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
// Utilities - Time Correlation - Unit tests                                  *
//*****************************************************************************

use util::tco;

///////////////
// functions //
///////////////

pub fn test() {
    println!(
        "ert_mission_epoch_with_leap_seconds = {}",
        tco::get_ert_mission_epoch_with_leap_seconds());
    println!(
        "obt_mission_epoch_with_leap_seconds = {}",
        tco::get_obt_mission_epoch_with_leap_seconds());
}
