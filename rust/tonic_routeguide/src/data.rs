use serde::Deserialize;
use std::fs::File;
use std::hash::{Hash, Hasher};

pub mod routeguide {
    tonic::include_proto!("routeguide");
}

impl Hash for crate::routeguide::Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
} 

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

#[allow(dead_code)]
pub fn load() -> Vec<crate::routeguide::Feature> {
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "data"]);
    let file = File::open(data_dir.join("route_guide_db.json")).expect("failed to open data file");

    let decoded: Vec<Feature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");

    decoded
        .into_iter()
        .map(|feature| crate::routeguide::Feature {
            name: feature.name,
            location: Some(crate::routeguide::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect()
}
