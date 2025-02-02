use libsopa::{
    locations::{Location, Locations},
    tags::Tags,
};
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct LocationsDatabase {
    // Note: The ID is used for comparison, technically there may be multiple
    //       locations databases at the same time, in the future.
    //       Comparing database content here is just a waste of time.
    id: uuid::Uuid,
    locations: std::sync::Arc<std::sync::RwLock<Locations>>,
}

impl PartialEq for LocationsDatabase {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}
impl Eq for LocationsDatabase {}

impl LocationsDatabase {
    pub fn new() -> Self {
        LocationsDatabase {
            id: uuid::Uuid::new_v4(),
            locations: std::sync::Arc::new(std::sync::RwLock::new(Locations::new())),
        }
    }

    pub fn new_with_samples() -> Self {
        let mut database = Self::new();
        database.use_locations_mut(|locations| {
            locations.push_new(|loc| {
                loc.name = "OiK Gdańsk".to_string();
                loc.tags = Tags::new_tags(["gender:male", "gender:female", "age:adult"]);
            });
            locations.push_new(|loc| {
                loc.name = "OiK Gdańsk - Hostel".to_string();
                loc.tags =
                    Tags::new_tags(["gender:male", "gender:female", "type:hostel", "age:adult"]);
            });
            locations.push_new(|loc| {
                loc.name = "OiK Gdynia".to_string();
                loc.tags = Tags::new_tags(["gender:male", "gender:female", "age:adult", "age:kid"]);
            });
        });
        database
    }

    pub fn use_locations<F>(&self, use_fn: F)
    where
        F: FnOnce(&Locations),
    {
        let locations = self.locations.read().unwrap();
        use_fn(&locations);
    }

    pub fn use_locations_mut<F>(&mut self, use_fn: F)
    where
        F: FnOnce(&mut Locations),
    {
        let mut locations = self.locations.write().unwrap();
        use_fn(&mut locations);
    }
}
