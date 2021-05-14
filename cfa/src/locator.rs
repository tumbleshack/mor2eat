// use std::collections::HashMap;
use std::error::Error;
use std::fs;

// use crossbeam; // 0.8.0
use serde::{Serialize, Deserialize};

pub fn parse_locator_profile_from(file: String) -> Result<LocatorProfile, Box<dyn Error>> {
    let contents = fs::read_to_string(file)?;

    let deserialized: LocatorProfile = serde_json::from_str(&contents).unwrap();
    Ok(deserialized)
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct StreetAddress {
    city: Option<String>,
    countryCode: Option<String>,
    extraDescription: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    line3: Option<String>,
    postalCode: Option<String>,
    region: Option<String>,
    sublocality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Coordinates {
    lat: f64,
    long: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct DayHourInterval {
    start: i32,
    end: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct DayHour {
    day: String,
    isClosed: bool,
    intervals: Vec<DayHourInterval>
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Hours {
    normalHours: Vec<DayHour>
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LocatorProfile {
    googlePlaceId: String,
    address: StreetAddress,
    yextRoutableCoordinate: Coordinates,
    c_conceptCode: String,
    c_status: String,
    c_locationSubtypeCode: String,
    hours: Hours,
}