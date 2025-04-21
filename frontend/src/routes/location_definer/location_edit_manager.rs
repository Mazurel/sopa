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

use std::ops::Deref;

use crate::app::SharedAppState;
use libsopa::locations::Location;
use yew::prelude::*;

/// Is a helper object that can be passed using properties
/// and can be used to work with Location that is currently being edited.
/// It offers two types of location saving:
/// 1. Staging -> Saves as temporal changes, is not affecting DB.
/// 2. Commiting -> Saves changes into DB.
#[derive(Clone, PartialEq)]
pub struct LocationEditManager {
    stage_changes_cb: Callback<Location>,
    commit_changes_cb: Callback<()>,
    clear_changes_cb: Callback<()>,
    get_location_under_edit_cb: Callback<(), Location>,
    request_new_location_cb: Callback<(), Location>,
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

        let request_new_location_cb = {
            let locations_db: UseStateHandle<_> = app_state.locations_db.clone();
            Callback::from(move |_| {
                let mut locations_db = locations_db.deref().clone();
                let mut new_location: Option<Location> = None;
                let new_location_ref = &mut new_location;
                locations_db.use_locations_mut(move |locations| {
                    let new_location_name = t!("new-location-name");
                    let new_location_address = t!("new-location-address");

                    let new_location = new_location_ref;
                    *new_location = Some(locations.push_new(|location| {
                        location.name = new_location_name.to_string();
                        location.address = new_location_address.to_string();
                    }));
                });
                new_location.unwrap()
            })
        };

        let update_location_cb = {
            let selected_location_state = selected_location.clone();
            Callback::from(move |new_location: Location| {
                selected_location_state.set(new_location);
            })
        };

        let commit_changes_cb = {
            let selected_location = selected_location.clone();
            let locations_db: UseStateHandle<_> = app_state.locations_db.clone();
            Callback::from(move |_| {
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
                locations_db.use_locations(|locations| {
                    if let Some(clean_location) = locations.fetch_update(&selected_location) {
                        selected_location.set(clean_location);
                    }
                });
            })
        };

        LocationEditManager {
            stage_changes_cb: update_location_cb,
            commit_changes_cb,
            clear_changes_cb,
            get_location_under_edit_cb,
            request_new_location_cb,
        }
    }

    /// Commits changes that are currently being saved
    /// with `update_location_changes`.
    pub fn commit_location_changes(&self) {
        self.commit_changes_cb.emit(())
    }

    /// Update location changes, so that they are staged
    /// before being commited.
    pub fn stage_location_changes(&self, modified_location: Location) {
        self.stage_changes_cb.emit(modified_location)
    }

    /// Clear all staged location changes.
    pub fn clear_location_changes(&self) {
        self.clear_changes_cb.emit(())
    }

    pub fn get_location_under_edit(&self) -> Location {
        self.get_location_under_edit_cb.emit(())
    }

    pub fn request_new_location(&self) -> Location {
        self.request_new_location_cb.emit(())
    }
}
