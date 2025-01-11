use libsopa::{
    locations::{Location, Locations},
    tags::Tags,
};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct LocationsDatabase<'a> {
    locations: std::borrow::Cow<'a, Locations>,
}

impl<'a> LocationsDatabase<'a> {
    pub fn new() -> Self {
        LocationsDatabase {
            locations: std::borrow::Cow::Owned(Locations::new()),
        }
    }

    pub fn new_with_samples() -> Self {
        let mut db = Self::new();
        db.insert_location(Location {
            name: "OiK Gdańsk".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female", "age:adult"]),
            ..Default::default()
        });
        db.insert_location(Location {
            name: "OiK Gdańsk - Hostel".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female", "type:hostel", "age:adult"]),
            ..Default::default()
        });
        db.insert_location(Location {
            name: "OiK Gdynia".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female", "age:adult", "age:kid"]),
            ..Default::default()
        });
        db
    }

    pub fn insert_location(&mut self, location: Location) {
        self.locations.to_mut().push(location)
    }

    pub fn all_locations(&self) -> &Locations {
        &self.locations
    }
}
