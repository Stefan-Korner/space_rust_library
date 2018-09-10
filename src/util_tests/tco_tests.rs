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
use util::asd_time;
use util::tco;

///////////////
// functions //
///////////////

pub fn test() {
    println!("ERT delta = {}", tco::get_ert_delta().num_seconds());
    println!("OBT delta = {}", tco::get_obt_delta().num_seconds());
    let tai_zero = asd_time::parse_time(tco::TAI_MISSION_EPOCH).unwrap();
    let unix_zero = asd_time::parse_time(tco::UNIX_MISSION_EPOCH).unwrap();
    let gps_zero = asd_time::parse_time(tco::GPS_MISSION_EPOCH).unwrap();
    println!("TAI zero = {}", asd_time::get_time_str_with_nano(tai_zero));
    println!("UNIX zero = {}", asd_time::get_time_str_with_nano(unix_zero));
    println!("GPS zero = {}", asd_time::get_time_str_with_nano(gps_zero));
    let tai_ert_correlated = tco::correlate_to_ert_mission_epoch(tai_zero);
    let unix_ert_correlated = tco::correlate_to_ert_mission_epoch(unix_zero);
    let gps_ert_correlated = tco::correlate_to_ert_mission_epoch(gps_zero);
    let tai_obt_correlated = tco::correlate_to_obt_mission_epoch(tai_zero);
    let unix_obt_correlated = tco::correlate_to_obt_mission_epoch(unix_zero);
    let gps_obt_correlated = tco::correlate_to_obt_mission_epoch(gps_zero);
    println!("TAI ERT correlated to = {}", asd_time::get_time_str_with_nano(tai_ert_correlated));
    println!("UNIX ERT correlated to = {}", asd_time::get_time_str_with_nano(unix_ert_correlated));
    println!("GPS ERT correlated to = {}", asd_time::get_time_str_with_nano(gps_ert_correlated));
    println!("TAI OBT correlated to = {}", asd_time::get_time_str_with_nano(tai_obt_correlated));
    println!("UNIX OBT correlated to = {}", asd_time::get_time_str_with_nano(unix_obt_correlated));
    println!("GPS OBT correlated to = {}", asd_time::get_time_str_with_nano(gps_obt_correlated));
    let tai_ert_correlated = tco::correlate_from_ert_mission_epoch(tai_ert_correlated);
    let unix_ert_correlated = tco::correlate_from_ert_mission_epoch(unix_ert_correlated);
    let gps_ert_correlated = tco::correlate_from_ert_mission_epoch(gps_ert_correlated);
    let tai_obt_correlated = tco::correlate_from_obt_mission_epoch(tai_obt_correlated);
    let unix_obt_correlated = tco::correlate_from_obt_mission_epoch(unix_obt_correlated);
    let gps_obt_correlated = tco::correlate_from_obt_mission_epoch(gps_obt_correlated);
    println!("TAI ERT correlated from = {}", asd_time::get_time_str_with_nano(tai_ert_correlated));
    println!("UNIX ERT correlated from = {}", asd_time::get_time_str_with_nano(unix_ert_correlated));
    println!("GPS ERT correlated from = {}", asd_time::get_time_str_with_nano(gps_ert_correlated));
    println!("TAI OBT correlated from = {}", asd_time::get_time_str_with_nano(tai_obt_correlated));
    println!("UNIX OBT correlated from = {}", asd_time::get_time_str_with_nano(unix_obt_correlated));
    println!("GPS OBT correlated from = {}", asd_time::get_time_str_with_nano(gps_obt_correlated));
}
