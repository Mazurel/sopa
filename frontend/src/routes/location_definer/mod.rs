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

use crate::yew_components::{LocationView, SelectionSettings};
use libsopa::locations::Location;
use log::info;
use yew::prelude::*;

use crate::app::SharedAppState;
use crate::download::{download_binary_data, upload_binary_data};
use location_edit::LocationEdit;
use location_edit_manager::LocationEditManager;

mod location_edit;
mod location_edit_manager;
mod tags_selection;

#[derive(Properties, Clone, PartialEq)]
pub struct LocationDefinerProps {
    pub app_state: SharedAppState,
}

fn fetch_all_locations(db: &crate::locations::LocationsDatabase) -> Vec<Location> {
    let mut locs = Vec::new();
    db.use_locations(|locations| {
        locs.append(&mut locations.locations_in_random_order());
    });
    locs
}

#[function_component(LocationDefiner)]
pub fn location_definer(props: &LocationDefinerProps) -> Html {
    let location_definer_add_label = t!("location-definer-add-label");
    let location_definer_save_label = t!("location-definer-save-label");
    let location_definer_load_label = t!("location-definer-load-label");

    let locations_list = {
        let locations_db = props.app_state.locations_db.clone();
        use_state(|| fetch_all_locations(&locations_db))
    };
    let selected_location_index_state = use_state(|| 0);
    let selected_location_state = {
        let locations = locations_list.clone();
        let selected_location_index_state = selected_location_index_state.clone();
        use_state_eq(move || {
            locations
                .get(*selected_location_index_state)
                .unwrap_or_else(|| locations.iter().next().unwrap())
                .clone()
        })
    };

    {
        let locations_db = props.app_state.locations_db.clone();
        let locations_list = locations_list.clone();
        // This effect is responsible for reloading locations list from DB.
        // It is called once and registers DB callback.
        use_effect_with((), move |_| {
            let mut locations_db = (*locations_db).clone();
            let locations_list = locations_list.clone();
            let cb = {
                let locations_db = locations_db.clone();
                Callback::from(move |_| {
                    info!("Refetching locations ...");
                    let locations_list = locations_list.clone();
                    let locations_db = locations_db.clone();
                    locations_list.set(fetch_all_locations(&locations_db));
                })
            };
            locations_db.register_db_changed_callback(cb);
        });
    }

    {
        let locations_list = locations_list.clone();
        let selected_location_state = selected_location_state.clone();
        let selected_location_index_state = selected_location_index_state.clone();
        // This effect is responsible for updating list of locations
        use_effect_with(
            (selected_location_index_state, selected_location_state),
            move |(selected_location_index_state, selected_location_state)| {
                let mut temp_locations_list = (*locations_list).clone();
                let i: usize = **selected_location_index_state;
                temp_locations_list[i] = (**selected_location_state).clone();
                locations_list.set(temp_locations_list);
            },
        );
    }

    let on_current_location_removed = {
        let locations_db = props.app_state.locations_db.clone();
        let locations_list = locations_list.clone();
        let selected_location_state = selected_location_state.clone();
        let selected_location_index_state = selected_location_index_state.clone();
        Callback::from(move |_| {
            let locations_db = locations_db.clone();
            let mut locations = (*locations_db).clone();
            let locations_list = locations_list.clone();
            let new_idx = match *selected_location_index_state <= 0 {
                false => *selected_location_index_state + 1,
                true => 0,
            };
            let selected_location = (*selected_location_state).clone();
            selected_location_index_state.set(new_idx);
            selected_location_state.set(locations_list[new_idx].clone());
            locations.use_locations_mut(move |locations| {
                locations.remove(selected_location);
            });
            locations_db.set(locations);
        })
    };

    let location_edit_manager = LocationEditManager::init(
        props.app_state.clone(),
        selected_location_state.clone(),
        on_current_location_removed.clone(),
    );

    let on_new_location_request_cb = {
        let location_edit_manager = location_edit_manager.clone();
        let locations_list = locations_list.clone();
        let selected_location_state = selected_location_state.clone();
        let selected_location_index_state = selected_location_index_state.clone();
        Callback::from(move |_: MouseEvent| {
            let new_location = location_edit_manager.request_new_location();
            let mut temp_locations_list = (*locations_list).clone();
            let new_index = temp_locations_list.len();
            temp_locations_list.push(new_location.clone());
            selected_location_index_state.set(new_index);
            locations_list.set(temp_locations_list);
            selected_location_state.set(new_location);
            info!("Inserted new location");
        })
    };

    let on_db_save_request_cb = {
        let locations_db = props.app_state.locations_db.clone();
        let notifications = props.app_state.notifications.clone();
        Callback::from(move |_: MouseEvent| {
            let notifications = notifications.clone();
            let mut locations_db = locations_db.deref().clone();
            locations_db.use_locations_mut(|locations| {
                let binary_db = locations.to_bin_data();
                let binary_db_reference: &[u8] = &binary_db;
                match download_binary_data(binary_db_reference, "sopa.bson", "application/bson") {
                    Ok(()) => notifications.notify_info(t!("download-ok")),
                    Err(err_msg) => {
                        notifications.notify_error(format!("{}: {}", t!("download-error"), err_msg))
                    }
                }
            });
        })
    };

    let on_db_load_request_cb = {
        let locations_db = props.app_state.locations_db.clone();
        Callback::from(move |_: MouseEvent| {
            let locations_db = locations_db.clone();
            // TODO: Add notification when notification system will be ready
            upload_binary_data(
                "sopa.bson",
                "*.bson",
                Callback::from(move |binary_data| {
                    let locations_db_new = (*locations_db).clone();
                    locations_db_new.reload_database_from_bin(binary_data);
                    locations_db.set(locations_db_new);
                }),
            )
            .expect("Upload should succeed");
        })
    };

    let selected_location: Location = selected_location_state.deref().clone();
    info!("Editing location : {selected_location:?}");

    let all_locations_view: Vec<Html> = {
        let selected_location_state = selected_location_state.clone();
        let selected_location_index_state = selected_location_index_state.clone();
        locations_list
            .iter()
            .enumerate()
            .map(|(i, loc)| {
                let selected_location_state = selected_location_state.clone();
                let selected_location_index_state = selected_location_index_state.clone();
                let on_selected_cb = Callback::from(move |location: Location| {
                    selected_location_state.set(location);
                    selected_location_index_state.set(i);
                });
                let selection_settings = SelectionSettings {
                    selection_cb: on_selected_cb,
                    state: (*loc) == selected_location,
                };
                html!(<LocationView location={loc.clone()} {selection_settings}/>)
            })
            .collect()
    };

    html!(
        <>
            <div class="is-pinned-to-right-bot">
                <button class="button is-rounded is-info" onclick={on_new_location_request_cb}>
                    { location_definer_add_label }
                </button>
                <button class="button is-rounded is-primary ml-2" onclick={on_db_save_request_cb}>
                    { location_definer_save_label }
                </button>
                <button class="button is-rounded is-warning ml-2" onclick={on_db_load_request_cb}>
                    { location_definer_load_label }
                </button>
            </div>
            <div class="columns">
                <div style="height: 100vh; overflow: scroll" class="column is-one-third">
                    { all_locations_view }
                </div>
                <div style="height: min-content" class="column box is-two-thirds">
                    <LocationEdit initial_location_to_edit={selected_location} {location_edit_manager}/>
                </div>
            </div>
        </>
    )
}
