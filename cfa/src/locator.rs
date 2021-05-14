// use std::collections::HashMap;
use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::HeaderMap;
use std::error::Error;

// use crossbeam; // 0.8.0
use serde::{Serialize, Deserialize};

pub async fn locate_near(zip_code: String) -> Result<LocatorJson, Box<dyn Error>> {
    let params = [("q", zip_code.to_string()), ("per", "5".to_string())];
    let mut map = HeaderMap::new();
    map.insert(ACCEPT, "application/json".parse().unwrap());

    let client = reqwest::Client::new();
    let req = client.get("https://locator.chick-fil-a.com.yext-cdn.com/search")
        .query(&params)
        .headers(map);

    let res = req.send().await?;
    let data = res.json::<LocatorJson>().await?;
    
    Ok(data)
}

// Object defintions

#[derive(Serialize, Deserialize, Debug)]
pub struct LocatorJson {
    response: LocatorResponse,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocatorResponse {
    entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    profile: LocatorProfile
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