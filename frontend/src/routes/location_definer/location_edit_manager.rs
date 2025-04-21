use std::ops::Deref;

use crate::app::SharedAppState;
use libsopa::locations::Location;
use log::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LocationEditManager {
    pub update_location_cb: Callback<Location>,
    pub commit_changes_cb: Callback<()>,
    pub clear_changes_cb: Callback<()>,
    get_location_under_edit_cb: Callback<(), Location>,
}

impl LocationEditManager {
    pub fn init(
        app_state: SharedAppState,
        selected_location: UseStateHandle<Location>,
    ) -> LocationEditManager {
        let get_location_under_edit_cb = {
            let selected_location_state = selected_location.clone();
            Callback::from(move |_| (*selected_location_state).clone())
        };

        let update_location_cb = {
            let selected_location_state = selected_location.clone();
            Callback::from(move |new_location: Location| {
                info!("Location updated: {new_location:?}");
                selected_location_state.set(new_location);
            })
        };

        let commit_changes_cb = {
            let selected_location = selected_location.clone();
            let locations_db: UseStateHandle<_> = app_state.locations_db.clone();
            Callback::from(move |_| {
                info!("Location commited: {selected_location:?}");
                let mut locations_db = locations_db.deref().clone();
                let selected_location = selected_location.clone().deref().clone();
                locations_db.use_locations_mut(move |locations| {
                    locations.push_update(selected_location);
                });
            })
        };

        let clear_changes_cb = {
            let selected_location = selected_location.clone();
            let locations_db: UseStateHandle<_> = app_state.locations_db.clone();
            Callback::from(move |_| {
                info!("Location cleared: {selected_location:?}");
                locations_db.use_locations(|locations| {
                    if let Some(clean_location) = locations.fetch_update(&selected_location) {
                        selected_location.set(clean_location);
                    }
                });
            })
        };

        LocationEditManager {
            update_location_cb,
            commit_changes_cb,
            clear_changes_cb,
            get_location_under_edit_cb,
        }
    }

    pub fn get_location_under_edit(&self) -> Location {
        self.get_location_under_edit_cb.emit(())
    }
}
