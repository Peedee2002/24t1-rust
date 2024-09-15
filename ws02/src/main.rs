use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CSVEntry {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}


/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("trains.csv");

    let entries: Vec<CSVEntry> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    println!("Entries: {entries:?}");
    let vec= entries.iter().map(|entry|
        (entry.station.as_str(), entry.time_period.as_str(), entry.entries_total.unwrap_or(0) + entry.exits_total.unwrap_or(0))
    );
    let mut map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for (station, period, amount) in vec {
        let year = &period[period.find("2").unwrap()..];
        let year_map = match map.get_mut(year) {
            Some(year_map) => year_map,
            None => {
                let new_map = HashMap::new();
                map.insert(year.to_string(), new_map);
                map.get_mut(year).unwrap()
            }
        };
        year_map.insert(station.to_string(), amount + year_map.get(station).unwrap_or(&0));
    }
    for (year, values) in map {
        let max = values.iter().reduce(|(prev_station, prev_amount), (curr_station, curr_amount)| {
            if curr_amount > prev_amount { (curr_station, curr_amount) } else { (prev_station, prev_amount) }
        }).unwrap();
        let min = values.iter().reduce(|(prev_station, prev_amount), (curr_station, curr_amount)| {
            if curr_amount < prev_amount { (curr_station, curr_amount) } else { (prev_station, prev_amount) }
        }).unwrap();
        println!("{}", year);
        println!("Max station: {:?}", max);
        println!("Min station: {:?}", min);
    }
    Ok(())
}
