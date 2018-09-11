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
// Calculation are base on time::Timespec (from Crate time)                   *
//*****************************************************************************
use time;
use util::asd_time;
use util::config;

///////////////
// constants //
///////////////
pub static TAI_MISSION_EPOCH: &str = "1958.001.00.00.00.000";
pub static TAI_MISSION_EPOCH_DELTA: i64 = -378691200;
pub static UNIX_MISSION_EPOCH: &str = "1970.001.00.00.00.000";
pub static UNIX_MISSION_EPOCH_DELTA: i64 = 0;
pub static GPS_MISSION_EPOCH: &str = "1980.006.00.00.00.000";
pub static GPS_MISSION_EPOCH_DELTA: i64 = 315964800;
pub static GPS_LEAP_SECONDS_1980: i64 = 0;
pub static GPS_LEAP_SECONDS_2009: i64 = 15;
pub static GPS_LEAP_SECONDS_2012: i64 = 16;
pub static GPS_LEAP_SECONDS_2015: i64 = 17;
pub static GPS_LEAP_SECONDS_2017: i64 = 18;

lazy_static! {
    static ref ERT_LEAP_SECONDS_STR: String =
        config::get_config_value_as_string("TCO_ERT_LEAP_SECONDS_STR");
    static ref ERT_MISSION_EPOCH_STR: String =
        config::get_config_value_as_string("TCO_ERT_MISSION_EPOCH_STR");
    static ref ERT_DELTA: time::Duration =
        get_mission_epoch_with_leap_seconds_delta(
            &ERT_MISSION_EPOCH_STR,
            &ERT_LEAP_SECONDS_STR);
    static ref OBT_LEAP_SECONDS_STR: String =
        config::get_config_value_as_string("TCO_OBT_LEAP_SECONDS_STR");
    static ref OBT_MISSION_EPOCH_STR: String =
        config::get_config_value_as_string("TCO_OBT_MISSION_EPOCH_STR");
    static ref OBT_DELTA: time::Duration =
        get_mission_epoch_with_leap_seconds_delta(
            &OBT_MISSION_EPOCH_STR,
            &OBT_LEAP_SECONDS_STR);
}

///////////////
// functions //
///////////////

// converts a leap seconds string (either symbolic or numeric) to seconds
fn get_leap_seconds(leap_seconds_str: &str) -> time::Duration {
    let leap_seconds = match leap_seconds_str {
        "GPS_LEAP_SECONDS_1980" => GPS_LEAP_SECONDS_1980,
        "GPS_LEAP_SECONDS_2009" => GPS_LEAP_SECONDS_2009,
        "GPS_LEAP_SECONDS_2012" => GPS_LEAP_SECONDS_2012,
        "GPS_LEAP_SECONDS_2015" => GPS_LEAP_SECONDS_2015,
        "GPS_LEAP_SECONDS_2017" => GPS_LEAP_SECONDS_2017,
        _ => {
            leap_seconds_str.parse::<i64>().expect(
                &format!("Leap seconds string {} has invalid symbolic format", leap_seconds_str))
        },
    };
    time::Duration::seconds(leap_seconds)
}

// converts a mission epoch string (either symbolic or ASD format) to seconds
fn get_mission_epoch(mission_epoch_str: &str) -> time::Duration {
    let timespec = match mission_epoch_str {
        "TAI_MISSION_EPOCH" => {
            time::Timespec::new(TAI_MISSION_EPOCH_DELTA, 0)
         },
        "UNIX_MISSION_EPOCH" => {
            time::Timespec::new(UNIX_MISSION_EPOCH_DELTA, 0)
         },
        "GPS_MISSION_EPOCH" => {
            time::Timespec::new(GPS_MISSION_EPOCH_DELTA, 0)
         },
        _ => {
            asd_time::parse_time(mission_epoch_str).expect(
                &format!("Mission epoch string {} has invalid symbolic format", mission_epoch_str))
        },
    };
    // convert Timespec to Duration
    timespec - time::Timespec::new(0, 0)
}

// converts mission epoch string and leap seconds string
// into mission epoch in seconds
fn get_mission_epoch_with_leap_seconds_delta (
    mission_epoch_str: &str,
    leap_seconds_str: &str) -> time::Duration {
    get_mission_epoch(mission_epoch_str) - get_leap_seconds(leap_seconds_str)
}

///////////////
// accessors //
///////////////

// return the ERT delta
pub fn get_ert_delta() -> time::Duration {
    *ERT_DELTA
}

// correlate the ERT mission epoch time to the local time
pub fn correlate_from_ert_mission_epoch(epoch_time: time::Timespec) ->
    time::Timespec {
    epoch_time + *ERT_DELTA
}

// correlate the local time to ERT mission epoch time
pub fn correlate_to_ert_mission_epoch(unix_time: time::Timespec) ->
    time::Timespec {
    unix_time - *ERT_DELTA
}

// return the OBT delta
pub fn get_obt_delta() -> time::Duration {
    *OBT_DELTA
}

// correlate the OBT mission epoch time to the local time
pub fn correlate_from_obt_mission_epoch(epoch_time: time::Timespec) ->
    time::Timespec {
    epoch_time + *OBT_DELTA
}

// correlate the local time to OBT mission epoch time
pub fn correlate_to_obt_mission_epoch(unix_time: time::Timespec) ->
    time::Timespec {
    unix_time - *OBT_DELTA
}
