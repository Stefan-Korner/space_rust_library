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
// Utilities - Time Correlation                                               *
//                                                                            *
// Two separate time lines are supported:                                     *
// ERT...Earth Reception Time line                                            *
// OBT...Onboard Time line                                                    *
//                                                                            *
// The module is statically configured via entries in config.txt              *
//*****************************************************************************

use util::config;

///////////////
// constants //
///////////////
// TAI_MISSION_EPOCH = "1958.001.00.00.00.000"
static TAI_MISSION_EPOCH_DELTA: i32 = -378691200;
// UNIX_MISSION_EPOCH = "1970.001.00.00.00.000"
static UNIX_MISSION_EPOCH_DELTA: i32 = 0;
// GPS_MISSION_EPOCH = "1980.006.00.00.00.000"
static GPS_MISSION_EPOCH_DELTA: i32 = 315964800;
static GPS_LEAP_SECONDS_1980: i32 = 0;
static GPS_LEAP_SECONDS_2009: i32 = 15;
static GPS_LEAP_SECONDS_2012: i32 = 16;
static GPS_LEAP_SECONDS_2015: i32 = 17;
static GPS_LEAP_SECONDS_2017: i32 = 18;

lazy_static! {
    static ref ERT_LEAP_SECONDS_STR: String =
        config::get_config_value_as_string("TCO_ERT_LEAP_SECONDS_STR");
    static ref ERT_MISSION_EPOCH_STR: String =
        config::get_config_value_as_string("TCO_ERT_MISSION_EPOCH_STR");
    static ref ERT_MISSION_EPOCH_WITH_LEAP_SECONDS: i32 =
        get_mission_epoch_with_leap_seconds(
            &ERT_MISSION_EPOCH_STR,
            &ERT_LEAP_SECONDS_STR);
    static ref OBT_LEAP_SECONDS_STR: String =
        config::get_config_value_as_string("TCO_OBT_LEAP_SECONDS_STR");
    static ref OBT_MISSION_EPOCH_STR: String =
        config::get_config_value_as_string("TCO_OBT_MISSION_EPOCH_STR");
    static ref OBT_MISSION_EPOCH_WITH_LEAP_SECONDS: i32 =
        get_mission_epoch_with_leap_seconds(
            &OBT_MISSION_EPOCH_STR,
            &OBT_LEAP_SECONDS_STR);
}

///////////////
// functions //
///////////////

// converts the leap seconds string to seconds
fn get_leap_seconds(leap_seconds_str: &str) -> i32 {
    match leap_seconds_str {
        "GPS_LEAP_SECONDS_1980" => GPS_LEAP_SECONDS_1980,
        "GPS_LEAP_SECONDS_2009" => GPS_LEAP_SECONDS_2009,
        "GPS_LEAP_SECONDS_2012" => GPS_LEAP_SECONDS_2012,
        "GPS_LEAP_SECONDS_2015" => GPS_LEAP_SECONDS_2015,
        "GPS_LEAP_SECONDS_2017" => GPS_LEAP_SECONDS_2017,
        _ => {
            leap_seconds_str.parse::<i32>().expect(
                &format!("Leap seconds string {} has invalid symbolic format", leap_seconds_str))
        }
    }
}

// converts the mission epoch string to seconds
fn get_mission_epoch(mission_epoch_str: &str) -> i32 {
    match mission_epoch_str {
        "TAI_MISSION_EPOCH" => return TAI_MISSION_EPOCH_DELTA,
        "UNIX_MISSION_EPOCH" => return UNIX_MISSION_EPOCH_DELTA,
        "GPS_MISSION_EPOCH" => return GPS_MISSION_EPOCH_DELTA,
        _ => {}
    };
    1234
}

// converts mission epoch string and leap seconds string
// into mission epoch in seconds
fn get_mission_epoch_with_leap_seconds(
    mission_epoch_str: &str,
    leap_seconds_str: &str) -> i32 {
    get_mission_epoch(mission_epoch_str) - get_leap_seconds(leap_seconds_str)
}

///////////////
// accessors //
///////////////

// return the ERT mission epoch incl. leap seconds
pub fn get_ert_mission_epoch_with_leap_seconds() -> i32 {
    *ERT_MISSION_EPOCH_WITH_LEAP_SECONDS
}

// correlate the ERT mission epoch time to the local time
pub fn correlate_from_ert_mission_epoch(rs_epoch_time: u32) -> u32 {
    ((rs_epoch_time as i32) + *ERT_MISSION_EPOCH_WITH_LEAP_SECONDS) as u32
}

// correlate the local time to ERT mission epoch time
pub fn correlate_to_ert_mission_epoch(rs_utc_time: u32) -> u32 {
    ((rs_utc_time as i32) - *ERT_MISSION_EPOCH_WITH_LEAP_SECONDS) as u32
}

// return the OBT mission epoch incl. leap seconds
pub fn get_obt_mission_epoch_with_leap_seconds() -> i32 {
    *OBT_MISSION_EPOCH_WITH_LEAP_SECONDS
}

// correlate the OBT mission epoch time to the local time
pub fn correlate_from_obt_mission_epoch(rs_epoch_time: u32) -> u32 {
    ((rs_epoch_time as i32) + *OBT_MISSION_EPOCH_WITH_LEAP_SECONDS) as u32
}

// correlate the local time to OBT mission epoch time
pub fn correlate_to_obt_mission_epoch(rs_utc_time: u32) -> u32 {
    ((rs_utc_time as i32) - *OBT_MISSION_EPOCH_WITH_LEAP_SECONDS) as u32
}
