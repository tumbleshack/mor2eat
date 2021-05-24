// use std::collections::HashMap;
use crossbeam_utils::thread;
// use std::fs;
use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::HeaderMap;
// use serde_json;
use std::error::Error;
use std::collections::HashMap;
use super::utils;

// use crossbeam; // 0.8.0
use serde::{Serialize, Deserialize};

#[tokio::main]
pub async fn locate_near(zip_code: String) -> Result<LocatorJson, Box<dyn Error>> {
    let params = [("q", zip_code.to_string()), ("per", "10".to_string())];
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

pub async fn dispatcher(zip_codes: Vec<String>) -> Result<Vec<LocatorProfile>, Box<dyn Error>> {
    let zip_code_chunks = zip_codes.chunks(100);

    let locations: Result<Vec<_>, _> = thread::scope(|s| {
        let threads: Vec<_> = zip_code_chunks.map(|chunk| {
            s.spawn(move |_| {
                let mut map = CfaProfiles::new();
                for code in chunk {
                    let json_result = locate_near(code.to_string());
                    match json_result {
                        Ok(data) => { 
                            println!("{}: Processing successful request", code);
                            for entity in data.response.entities {
                                map.insert(entity.profile.meta.id.to_string(), entity.profile);
                            }
                        }
                        Err(e) => println!("{}: ERROR! encountered when unwraping json data {:?}", code, e)
                    }
                }
                map
            })
        })
        .collect();
        threads.into_iter().map(|t| { t.join() } ).collect()
    }).unwrap();

    let locations = locations.unwrap();

    let locations = locations
        .into_iter()
        .fold(CfaProfiles::new(), merge_locations);

    let all_locations = locations.values().cloned().collect::<Vec<LocatorProfile>>();

    Ok(all_locations)
}

fn merge_locations(mut a: CfaProfiles, b: CfaProfiles) -> CfaProfiles {
    a.extend(b);
    a
}

pub fn output_cfa_data_to(path: String, data: Vec<LocatorProfile>) -> Result<(), Box<dyn Error>> {
    println!("");
    println!("Writing valid connection lists to file...");
    utils::output_to(path, &data)
}

pub fn intput_cfa_data_from(path: String) -> Result<CfaProfiles, Box<dyn Error>> {
    println!("Reading cfa data to memory...");
    // let str = fs::read_to_string(path)?;
    // let data: Vec<LocatorProfile> = serde_json::from_str(&str)?;
    let data: Vec<LocatorProfile> = utils::input_from(&path[..])?;
    let mut map = CfaProfiles::new();
    for profile in data {
        map.insert(profile.meta.id.to_string(), profile);
    }
    Ok(map)
}

pub type CfaProfiles = HashMap<String, LocatorProfile>;

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
    profile: LocatorProfile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct StreetAddress {
    pub city: Option<String>,
    pub countryCode: Option<String>,
    pub extraDescription: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub postalCode: Option<String>,
    pub region: Option<String>,
    pub sublocality: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Coordinates {
    pub lat: f64,
    pub long: f64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct DayHourInterval {
    pub start: i32,
    pub end: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct DayHour {
    pub day: String,
    pub isClosed: bool,
    pub intervals: Vec<DayHourInterval>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Hours {
    pub normalHours: Vec<DayHour>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainPhone {
    pub display: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct LocatorProfile {
    pub googlePlaceId: Option<String>,
    pub address: StreetAddress,
    pub yextRoutableCoordinate: Coordinates,
    pub c_conceptCode: String,
    pub c_status: String,
    pub c_locationSubtypeCode: String,
    pub c_carryout: bool,
    pub c_fullDineIn: bool,
    pub c_limitedDineIn: bool,
    pub c_locationName: String,
    pub hours: Hours,
    pub mainPhone: Option<MainPhone>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profiles {
    profiles: Vec<LocatorProfile>,
}