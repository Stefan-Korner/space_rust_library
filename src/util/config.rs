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
// Utilities - Configuration File Handling                                    *
//                                                                            *
// Configuration items are define a configuration file as name/value pairs.   *
// The values in the configuration file can be overwritten via environment    *
// variables. The file name of the configuration file is                      *
//                                                                            *
//     config.txt                                                             *
//                                                                            *
// but it can be defined explizitly via command line switch                   *
//                                                                            *
//     config_file <file_name>                                                *
//                                                                            *
// Note: A white space is needed as separator between the config_file switch  *
//       and the <file_name>.                                                 *
// Note: The file is searched from the local directory, when no absolute path *
//       is used in the <file_name>.                                          *
//*****************************************************************************
use std::collections::HashMap;
use std::env;
use std::fs;

///////////////
// constants //
///////////////

// default config file name
static DEFAULT_CONFIG_FILE_NAME: &'static str = "config.txt";
// config file name - command line option
static DEFAULT_CONFIG_FILE_OPTION: &'static str = "config_file";

lazy_static! {
    static ref CONFIG_NAME_VALUE_PAIRS: HashMap<String, String> =
        read_config_name_value_pairs();
}

///////////////
// functions //
///////////////

// returns the name of the config file
fn get_config_file_name() -> String {
    // search command line options for the proper switch
    let mut switch_found = false;
    for argument in env::args() {
        if switch_found {
            return argument
        }
        if argument == DEFAULT_CONFIG_FILE_OPTION.to_string() {
            switch_found = true;
        }
    }
    DEFAULT_CONFIG_FILE_NAME.to_string()
}

// reads name/value pairs from file,
// returns a hash map (--> unsorted!) with name/value pairs,
// supports comments with #
fn read_name_value_pair_file(file_name: &str) -> HashMap<String, String> {
    let text = fs::read_to_string(file_name).expect(
        &format!("Unable to read file {}", file_name));
    let mut name_value_pairs = HashMap::new();
    for (line_nr, mut line) in text.lines().enumerate() {
        // remove comments
        match line.find("#") {
            Some(comment_pos) => {
                line = &line[..comment_pos];
            },
            None => {}
        }
        // remove trailing and ending white spaces
        line = line.trim();
        // skip empty lines
        if line.len() == 0 {
            continue;
        }
        let mut split_line = line.split('=');
        let name = split_line.next().expect(
            &format!("Cannot read name from file {}, line {}", file_name, line_nr));
        let value = split_line.next().expect(
            &format!("Cannot read value from file {}, line {}", file_name, line_nr));
        name_value_pairs.insert(name.trim().to_string(), value.trim().to_string());
    }
    name_value_pairs
}

// read name/value pairs from config file
// and overwrite values from environment variables
fn read_config_name_value_pairs() -> HashMap<String, String> {
    let config_file_name = get_config_file_name();
    let mut name_value_pairs = read_name_value_pair_file(&config_file_name);
    for (name, mut value) in &mut name_value_pairs {
        match env::var(name) {
            Ok(env_value) => {
                // in-place value change
                value.clear();
                value.push_str(&env_value);
            }
            Err(_) => {}
        }
    }
    name_value_pairs
}

///////////////
// accessors //
///////////////

// forces immediate read of configuration entries
// panics when there is no configuration file
pub fn force_config_items_read() {
    CONFIG_NAME_VALUE_PAIRS.len();
}

// returns all configuration entries
pub fn get_config_name_value_pairs() -> &'static HashMap<String, String> {
    &CONFIG_NAME_VALUE_PAIRS
}

// returns a configuration entry as string
pub fn get_config_value_as_string(value_name: &str) -> String {
    match CONFIG_NAME_VALUE_PAIRS.get(value_name) {
        Some(value) => {
            value.to_string()
        },
        None => {
            panic!(format!("Config item {} not found", value_name));
        }
    }
}

// returns a configuration entry as u8 value
pub fn get_config_value_as_u8(value_name: &str) -> u8 {
    get_config_value_as_string(value_name).parse::<u8>().expect(
        &format!("Config item {} has no unsigned int 8 format", value_name))
}

// returns a configuration entry as u16 value
pub fn get_config_value_as_u16(value_name: &str) -> u16 {
    get_config_value_as_string(value_name).parse::<u16>().expect(
        &format!("Config item {} has no unsigned int 16 format", value_name))
}

// returns a configuration entry as u32 value
pub fn get_config_value_as_u32(value_name: &str) -> u32 {
    get_config_value_as_string(value_name).parse::<u32>().expect(
        &format!("Config item {} has no unsigned int 32 format", value_name))
}

// returns a configuration entry as u64 value
pub fn get_config_value_as_u64(value_name: &str) -> u64 {
    get_config_value_as_string(value_name).parse::<u64>().expect(
        &format!("Config item {} has no unsigned int 64 format", value_name))
}

// returns a configuration entry as usize value
pub fn get_config_value_as_usize(value_name: &str) -> usize {
    get_config_value_as_string(value_name).parse::<usize>().expect(
        &format!("Config item {} has no unsigned size format", value_name))
}

// returns a configuration entry as i8 value
pub fn get_config_value_as_i8(value_name: &str) -> i8 {
    get_config_value_as_string(value_name).parse::<i8>().expect(
        &format!("Config item {} has no signed int 8 format", value_name))
}

// returns a configuration entry as i16 value
pub fn get_config_value_as_i16(value_name: &str) -> i16 {
    get_config_value_as_string(value_name).parse::<i16>().expect(
        &format!("Config item {} has no signed int 16 format", value_name))
}

// returns a configuration entry as i32 value
pub fn get_config_value_as_i32(value_name: &str) -> i32 {
    get_config_value_as_string(value_name).parse::<i32>().expect(
        &format!("Config item {} has no signed int 32 format", value_name))
}

// returns a configuration entry as i64 value
pub fn get_config_value_as_i64(value_name: &str) -> i64 {
    get_config_value_as_string(value_name).parse::<i64>().expect(
        &format!("Config item {} has no signed int 64 format", value_name))
}

// returns a configuration entry as isize value
pub fn get_config_value_as_isize(value_name: &str) -> isize {
    get_config_value_as_string(value_name).parse::<isize>().expect(
        &format!("Config item {} has no signed size format", value_name))
}

// returns a configuration entry as f32 value
pub fn get_config_value_as_f32(value_name: &str) -> f32 {
    get_config_value_as_string(value_name).parse::<f32>().expect(
        &format!("Config item {} has no float 32 format", value_name))
}

// returns a configuration entry as f64 value
pub fn get_config_value_as_f64(value_name: &str) -> f64 {
    get_config_value_as_string(value_name).parse::<f64>().expect(
        &format!("Config item {} has no float 64 format", value_name))
}
