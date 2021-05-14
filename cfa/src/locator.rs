// use std::collections::HashMap;
use crossbeam_utils::thread;
use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::HeaderMap;
use std::error::Error;
use std::collections::HashMap;

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

pub async fn dispatcher(zip_codes: Vec<String>) {

    let zip_code_chunks = zip_codes.chunks(100);

    let locations: Result<Vec<_>, _> = thread::scope(|s| {
        let threads: Vec<_> = zip_code_chunks.map(|chunk| {
            s.spawn(move |_| {
                let mut map = CfaProfiles::new();
                for code in chunk {
                    let json_result = locate_near(code.to_string());
                    for entity in json_result.unwrap().response.entities {
                        map.insert(entity.profile.meta.id.to_string(), entity.profile);
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

    // let serialized = serde_json::to_string_pretty(&all_locations).unwrap();
    // println!("serialized = {}", serialized);

}

fn merge_locations(mut a: CfaProfiles, b: CfaProfiles) -> CfaProfiles {
    a.extend(b);
    a
}

type CfaProfiles = HashMap<String, LocatorProfile>;

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
struct Meta {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_snake_case)]
struct Coordinates {
    lat: f64,
    long: f64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct DayHourInterval {
    start: i32,
    end: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct DayHour {
    day: String,
    isClosed: bool,
    intervals: Vec<DayHourInterval>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct Hours {
    normalHours: Vec<DayHour>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct LocatorProfile {
    googlePlaceId: Option<String>,
    address: StreetAddress,
    yextRoutableCoordinate: Coordinates,
    c_conceptCode: String,
    c_status: String,
    c_locationSubtypeCode: String,
    hours: Hours,
    meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profiles {
    profiles: Vec<LocatorProfile>,
}