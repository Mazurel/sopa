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

use deli::{Database, Error, Model, Transaction};
use libsopa::locations::{Location, Locations};
use log::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const LOCATIONS_STORE_NAME: &str = "locations";

fn create_read_transaction(database: &Database) -> Result<Transaction, Error> {
    database.transaction().with_model::<Location>().build()
}

fn create_write_transaction(database: &Database) -> Result<Transaction, Error> {
    database
        .transaction()
        .writable()
        .with_model::<Location>()
        .build()
}

#[derive(Properties, Clone)]
pub struct LocationsDatabase {
    // Note: The ID is used for comparison, technically there may be multiple
    //       locations databases at the same time, in the future.
    //       Comparing database content here is just a waste of time.
    id: uuid::Uuid,
    locations: std::sync::Arc<std::sync::RwLock<Locations>>,
    yew_refresh_handle: UseForceUpdateHandle,
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
    pub fn new(yew_refresh_handle: UseForceUpdateHandle) -> Self {
        let new_self = LocationsDatabase {
            id: uuid::Uuid::new_v4(),
            locations: std::sync::Arc::new(std::sync::RwLock::new(Locations::new())),
            yew_refresh_handle,
        };

        {
            let new_self: LocationsDatabase = new_self.clone();
            spawn_local(async move {
                let mut new_self: LocationsDatabase = new_self.clone();
                new_self.fetch_locations_from_indexed_db_wrapped().await;
            });
        }

        new_self
    }

    async fn backup_locations_in_indexed_db(&self) -> Result<(), String> {
        // Open the database, creating it if needed
        let db = Database::builder(LOCATIONS_STORE_NAME)
            .version(1)
            .add_model::<Location>()
            .build()
            .await
            .map_err(|err| format!("Failed opening database: {err:?}"))?;

        let write_transaction = create_write_transaction(&db)
            .map_err(|err| format!("Failed creating write transaction: {err:?}"))?;
        let location_transaction = Location::with_transaction(&write_transaction)
            .map_err(|err| format!("Failed creating location transaction: {err:?}"))?;
        {
            let locations = {
                let locations = self.locations.read().unwrap();
                locations.locations_in_random_order()
            };
            for location in locations {
                info!("BACKUP");

                match location_transaction
                    .get(&location.get_id())
                    .await
                    .map_err(|err| format!("Failed getting location: {err:?}"))?
                {
                    Some(_) => {
                        // Update
                        location_transaction
                            .update(&location)
                            .await
                            .map_err(|err| {
                                format!("Failed changing location {location:?}: {err:?}")
                            })?;
                    }
                    None => {
                        // Add
                        location_transaction.add(&location).await.map_err(|err| {
                            format!("Failed adding location {location:?}: {err:?}")
                        })?;
                    }
                }
            }
        }

        Ok(())
    }

    async fn fetch_locations_from_indexed_db(&mut self) -> Result<(), String> {
        // Open the database, creating it if needed
        let db = Database::builder(LOCATIONS_STORE_NAME)
            .version(1)
            .add_model::<Location>()
            .build()
            .await
            .map_err(|err| format!("Failed opening database: {err:?}"))?;

        let read_transaction = create_read_transaction(&db)
            .map_err(|err| format!("Failed creating read transaction: {err:?}"))?;
        let location_transaction = Location::with_transaction(&read_transaction)
            .map_err(|err| format!("Failed creating location transaction: {err:?}"))?;

        let all_locations = location_transaction
            .get_all(.., None)
            .await
            .map_err(|err| format!("Failed reading all locations: {err:?}"))?;

        self.use_locations_mut_without_indexed_db(move |locations| {
            for location in all_locations {
                locations.push_update(location);
            }
        });

        Ok(())
    }

    async fn backup_locations_in_indexed_db_wrapped(&self) {
        match self.backup_locations_in_indexed_db().await {
            Ok(_) => (),
            Err(err) => warn!("Failed backing up location in IndexedDB: {err:?}"),
        }
    }

    async fn fetch_locations_from_indexed_db_wrapped(&mut self) {
        match self.fetch_locations_from_indexed_db().await {
            Ok(_) => {
                self.yew_refresh_handle.force_update();
                info!("Force update");
            }
            Err(err) => warn!("Failed fetching locations from IndexedDB: {err:?}"),
        }
    }

    pub fn reload_database_from_bin(&self, bin_data: Vec<u8>) {
        {
            let mut locations = self.locations.write().unwrap();
            *locations = Locations::from_bin_data(bin_data);
        }
    }

    pub fn load_default_database(yew_refresh_handle: UseForceUpdateHandle) -> Self {
        let database_raw: Vec<u8> = include_bytes!("initial_database.bson").to_vec();

        let mut database = Self::new(yew_refresh_handle);
        // We do not want to overwrite whatever is in indexed db
        database.use_locations_mut_without_indexed_db(move |locations| {
            *locations = Locations::from_bin_data(database_raw);
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
        self.use_locations_mut_without_indexed_db(use_fn);
        let database: LocationsDatabase = self.clone();
        spawn_local(async move {
            let database: LocationsDatabase = database.clone();
            database.backup_locations_in_indexed_db_wrapped().await;
        });
    }

    fn use_locations_mut_without_indexed_db<F>(&mut self, use_fn: F)
    where
        F: FnOnce(&mut Locations),
    {
        let mut locations = self.locations.write().unwrap();
        use_fn(&mut locations);
    }
}
