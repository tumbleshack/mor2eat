use crossbeam_utils::thread;
use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::HeaderMap;
use serde::{Serialize, Deserialize};
use std::env;
use std::error::Error;
use std::collections::HashMap;
use super::locator;
use super::utils;



pub fn decide_connections_from(data: locator::CfaProfiles) -> Result<ConnectionLists, Box<dyn Error>> {
    let mut map = ConnectionLists::new();
    let mut conn_count: i64 = 0;
    for profile in data.values() {
        for node in data.values() {
            if 
                !profile.meta.id.eq(&node.meta.id) &&
                hypotenuse(&profile.yextRoutableCoordinate, &node.yextRoutableCoordinate) < 0.25 
            {
                if !map.contains_key(&profile.meta.id) {
                    map.insert(profile.meta.id.clone(), Vec::new());
                }
                let vec = map.get_mut(&profile.meta.id).unwrap();
                vec.push(node.meta.id.clone());
                conn_count += 1;
            }
        }
    }
    println!("Connection count: {}", conn_count);
    Ok(map)
}

pub fn build_edges(profiles: &locator::CfaProfiles, valid_connections: &ConnectionLists) -> Result<Edges, Box<dyn Error>> {
    let sites: Vec<String> = valid_connections.keys().map(|s| (&**s).to_string()).collect::<Vec<_>>();
    let site_chunks = sites.chunks(500);

    let edge_chunks: Result<Vec<_>, _> = thread::scope(|s| {
        let threads: Vec<_> = site_chunks.map(|chunk| {
            s.spawn(move |_| {
                edges_for(chunk.to_vec(), profiles, valid_connections)
            })
        }).collect();
        threads.into_iter().map(|t| { t.join() }).collect()
    }).unwrap();

    let edge_chunks: Vec<Edges> = edge_chunks.unwrap();

    let edges = edge_chunks
        .into_iter()
        .fold(Edges::new(), merge_edges);

    Ok(edges)
}

fn merge_edges(mut a: Edges, b: Edges) -> Edges {
    a.extend(b);
    a
}

pub fn edges_for(sites: Vec<String>, profiles: &locator::CfaProfiles, connections: &ConnectionLists) -> Edges {
    let mut map = Edges::new();
    
    for site in sites {
        let mut destinations: Vec<String> = Vec::new();
        let conns = connections.get(&site).unwrap();
        for dest in conns {
            let dest_loc_str = location_str_from(dest, profiles);
            destinations.push(dest_loc_str.unwrap());
        }
        if destinations.len() < 1 {
            println!("{}: NONE destinations", &site);
            continue;
        }
        let chunk_size = 5;
        let dest_chunks = destinations.chunks(chunk_size);
        // println!("dest chunks = {:?}", dest_chunks);
        let mut chunk_idx = 0;
        for dest_chunk in dest_chunks {
            let dest_str = dest_chunk.join("|");
            let origin_str = location_str_from(&site, profiles).unwrap();
            // println!("origin = {}", origin_str);
            // println!("desinations = {}", dest_str);
            match call_distance_matrix(&origin_str, &dest_str) {
                Ok(distance_data) => {
                    // println!("Distance data ={:?}", &distance_data.destination_addresses);
                    if distance_data.rows.len() > 0 {
                        let mut distances: Vec<Edge> = Vec::new();
                        for (index, element) in distance_data.rows.get(0).unwrap().elements.iter().enumerate() {
                            let id_temp = conns.get(chunk_idx * chunk_size + index).unwrap().to_string();
                            // println!("{}: mapping from {:?}", id_temp, element);
                            let edge = Edge {
                                node: id_temp,
                                distance: element.duration.value
                            };
                            distances.push(edge);
                        }
                        if map.contains_key(&site) {
                            for distance in distances {
                                map.get_mut(&site).unwrap().push(distance);
                            }
                        } else {
                            map.insert(site.clone(), distances.clone());
                        }
                    } else {
                        println!("{}: ROWS LESS THAN 1", site);
                        println!("{}: Distance data = {:?}", site, distance_data);
                        println!("origin = {}", origin_str);
                        println!("desinations = {}", dest_str);
                    }
                },
                Err(err) => {
                    println!("Problem processing google maps data for site {}, {}", site, err);
                }
            }
            chunk_idx += 1;
        }
    }
    map
}

#[tokio::main]
pub async fn call_distance_matrix(origin_str: &str, dest_str: &str) -> Result<DistanceResponse, Box<dyn Error>> {
    let params = [
        ("origins", origin_str), 
        ("destinations", dest_str),
        // ("departure_time", "now"),
        ("key", &env::var("GMAPS_API_KEY")?)
    ];
    let mut map = HeaderMap::new();
    map.insert(ACCEPT, "application/json".parse().unwrap());

    let client = reqwest::Client::new();
    let req = client.get("https://maps.googleapis.com/maps/api/distancematrix/json")
        .query(&params)
        .headers(map);

    let res = req.send().await?;

    let data = res.json::<DistanceResponse>().await?;
    
    Ok(data)
}

pub fn location_str_from(site_id: &str, profiles: &locator::CfaProfiles) -> Result<String, Box<dyn Error>> {
    if let Some(place_id) = &profiles.get(site_id).unwrap().googlePlaceId {
        let mut label: String = "place_id:".to_owned();
        label.push_str(&place_id);
        Ok(label)
    } else {
        let mut label: String = profiles.get(site_id).unwrap().yextRoutableCoordinate.lat.to_string().to_owned();
        label.push(',');
        label.push_str(&(profiles.get(site_id).unwrap().yextRoutableCoordinate.long.to_string()[..]));
        Ok(label)
    }
}

pub fn output_valid_connections_from(path: String, lists: ConnectionLists) -> Result<(), Box<dyn Error>> {
    println!("");
    println!("Writing valid connection lists to file...");
    utils::output_to(path, &lists)
}

pub fn input_valid_conntions_from(path: String) -> Result<ConnectionLists, Box<dyn Error>> {
    println!("Reading valid connection lists to memory...");
    utils::input_from(&path[..])
}

pub fn hypotenuse(a: &locator::Coordinates, b: &locator::Coordinates) -> f64 {
    ((a.lat - b.lat).powf(2.0) + (a.long - b.long).powf(2.0)).sqrt()
}

type ConnectionLists = HashMap<String, Vec<String>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge { pub node: String, pub distance: i64 }

pub type Edges = HashMap<String, Vec<Edge>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DistanceResponse {
    pub status: String,
    pub destination_addresses: Vec<String>,
    pub rows: Vec<RowObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RowObject {
    pub elements: Vec<ElementObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElementObject {
    pub status: String,
    pub duration: Duration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Duration {
    pub value: i64,
}