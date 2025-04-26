/*
Copyright (C) 2025 Mateusz Mazur (Mazurel) <mateusz.mazur@e.email>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, see
<https://www.gnu.org/licenses/>.
*/

use crate::{contact::ContactMethods, tags::Tags};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Location {
    id: Uuid,
    pub name: String,
    pub tags: Tags,
    pub address: String,
    pub description: String,
    pub contact_methods: ContactMethods,
}

impl Location {
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            id: Uuid::new_v4(),
            name: String::from(""),
            tags: Tags::new(),
            address: String::from(""),
            description: String::from(""),
            contact_methods: ContactMethods::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Locations {
    pub locations: HashMap<Uuid, Location>,
}

impl From<Vec<Location>> for Locations {
    fn from(value: Vec<Location>) -> Self {
        Locations {
            locations: value.into_iter().map(|loc| (loc.id, loc)).collect(),
        }
    }
}

impl Locations {
    /// Initializes new locations DB
    ///
    /// ```rust
    /// let locations = libsopa::locations::Locations::new();
    /// ```
    pub fn new() -> Self {
        Self::from(vec![])
    }

    pub fn build_tags(&self) -> Tags {
        let mut tags = Tags::new();
        for loc in self.locations_in_random_order().into_iter() {
            for tag in loc.tags.get_all_tags() {
                tags.define_tag(tag.name.clone());
            }
        }
        tags
    }

    pub fn locations_in_random_order(&self) -> Vec<Location> {
        self.locations
            .values()
            .into_iter()
            .map(|l| l.clone())
            .collect()
    }

    pub fn all_locations_in_order(&self, tags_preference: &Tags) -> Vec<Location> {
        let mut locations = self.locations_in_random_order();
        let mut locations_overlaps: Vec<_> = locations
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

    /// Fetch update for `old_location`.
    /// This function returns new instance of Location with updated content,
    /// it is assummed the function will be used in the following fashion:
    ///
    /// ```rust
    /// # use libsopa::locations::*;
    /// # let mut locations = Locations::new();
    /// # let mut old_location = locations.push_new(|loc| {
    /// #    loc.name = "Example".to_string();
    /// # });
    /// if let Some(new_location) = locations.fetch_update(&old_location) {
    ///     // Most likely you will want to update old location now
    ///     old_location = new_location;
    /// }
    /// ```
    ///
    /// **SAFETY:**
    /// Current implementation doesn't distinguish missing location in the DB
    /// with case when location wasn't changed.
    /// That is why, for now we panic in case of location missing in the DB,
    /// in the future we will need to return custom result type.
    pub fn fetch_update(&self, old_location: &Location) -> Option<Location> {
        if let Some(location) = self.locations.get(&old_location.get_id()) {
            if old_location != location {
                Some(location.clone())
            } else {
                None
            }
        } else {
            panic!("Location is not in the DB at all, this API doesn't support this case for now!");
        }
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.locations.contains_key(&location.id)
    }

    /// Push state of `new_location` to the database, modyfing permanently
    /// content of the `Location` in the database, based on location ID.
    ///
    /// **WARNING**:
    /// Pushing update may result in data loss if the same location
    /// was modified "in the meantime". Please try to fetch current
    /// location version from via `fetch_update` before modyfing location.
    pub fn push_update(&mut self, new_location: Location) {
        self.locations.insert(new_location.get_id(), new_location);
    }

    /// Creates new location instance in the database,
    /// then consumes `modify_loc_fn` to change this location initial content.
    /// Afterwards commits the changes.
    pub fn push_new<F>(&mut self, modify_loc_fn: F) -> Location
    where
        F: FnOnce(&mut Location),
    {
        let mut location = Location::default();
        modify_loc_fn(&mut location);
        self.push_update(location.clone());
        location
    }

    /// Creates new location instance in the database,
    /// Afterwards commits the changes.
    pub fn push_new_nomodify(&mut self) -> Location {
        self.push_new(|_| {})
    }

    pub fn to_bin_data(&self) -> Vec<u8> {
        bson::to_vec(self).unwrap()
    }

    pub fn from_bin_data(bin_data: Vec<u8>) -> Self {
        bson::from_slice(&bin_data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_locations_and_working_with_one_location() {
        let mut locations = Locations::new();
        let location = locations.push_new(|loc| {
            loc.name = "Example".to_string();
        });
        assert_eq!(location.name, "Example".to_string());

        {
            let mut modified_location = location.clone();
            modified_location.name = "Not an Example".to_string();
            locations.push_update(modified_location);
        }

        // The original object should still contain old value
        assert_eq!(location.name, "Example".to_string());
        // But after fetching update, it should have a new value
        let new_location = locations
            .fetch_update(&location)
            .expect("There should be an update to the location");
        assert_ne!(location, new_location);
        assert_eq!(new_location.name, "Not an Example".to_string());
    }
}
