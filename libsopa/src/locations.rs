use crate::tags::Tags;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub name: String,
    pub tags: Tags,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Locations {
    pub locations: Vec<Location>,
}

impl From<Vec<Location>> for Locations {
    fn from(value: Vec<Location>) -> Self {
        Locations { locations: value }
    }
}

impl Into<Vec<Location>> for Locations {
    fn into(self) -> Vec<Location> {
        self.locations
    }
}

impl Locations {
    pub fn new() -> Self {
        Self::from(vec![])
    }

    pub fn build_tags(&self) -> Tags {
        let mut tags = Tags::new();
        for loc in self.locations.iter() {
            for tag in loc.tags.get_all_tags() {
                tags.define_tag(tag.name.clone());
            }
        }
        tags
    }

    pub fn locations_in_random_order<'a>(&'a self) -> &'a Vec<Location> {
        &self.locations
    }

    pub fn all_locations_in_order(&self, tags_preference: &Tags) -> Vec<Location> {
        let mut locations = self.locations.clone();
        let mut locations_overlaps: Vec<_> = self
            .locations
            .clone()
            .into_iter()
            .map(|loc| loc.tags.overlap(tags_preference))
            .collect();

        let mut locations_result = Vec::new();

        while locations.len() > 0 {
            let i = locations_overlaps
                .iter()
                .enumerate()
                .min_by(|(_, o1), (_, o2)| o2.total_cmp(o1))
                .expect("Locations should not be empty due to loop condition.")
                .0;

            locations_overlaps.remove(i);
            locations_result.push(locations.remove(i));
        }

        locations_result
    }

    pub fn to_bin_data(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    pub fn push(&mut self, location: Location) {
        self.locations.push(location);
    }
}
