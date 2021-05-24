use std::fs;
use std::error::Error;
use serde::{Serialize, Deserialize};
use super::locator;

pub fn output_to<T: Serialize>(path: String, object: &T) -> Result<(), Box<dyn Error>> {
    let data_str = serde_json::to_string_pretty(object).unwrap();
    fs::write(path, data_str)?;
    Ok(())
}

// https://www.reddit.com/r/rust/comments/dewoz5/need_help_understanding_serde_and_rust_lifetimes/
pub fn input_from<T: for <'a> Deserialize<'a>>(path: &str) -> Result<T, Box<dyn Error>> {
    let str = fs::read_to_string(path).unwrap().clone();
    let data: T = serde_json::from_str(&str)?;
    Ok(data)
}

pub fn filter_profiles(profiles: &locator::CfaProfiles) -> locator::CfaProfiles {
    let mut map = locator::CfaProfiles::new();
    for profile in profiles.values() {
        if
            profile.c_status.eq("OPEN") &&
            profile.c_locationSubtypeCode.eq("FSU") || profile.c_locationSubtypeCode.eq("DHG") || profile.c_locationSubtypeCode.eq("DTO") &&
            profile.c_carryout
        {
            map.insert(profile.meta.id.clone(), profile.clone());
        }
    }
    map
}