use std::ops::Deref;

use crate::yew_components::{LocationView, SelectionSettings};
use libsopa::locations::Location;
use log::info;
use yew::prelude::*;

use crate::app::SharedAppState;
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
    let locations_db = &props.app_state.locations_db;

    let locations_list = fetch_all_locations(locations_db);
    let location_definer_add_label = t!("location-definer-add-label");

    let selected_location_state = {
        let locations = locations_list.clone();
        use_state_eq(move || {
            locations
                .into_iter()
                .next()
                .expect("There exists at least one location")
        })
    };

    let on_selected_cb = {
        let selected_location_state = selected_location_state.clone();
        Callback::from(move |location: Location| {
            selected_location_state.set(location);
        })
    };

    let selected_location: Location = selected_location_state.deref().clone();
    info!("Editing location : {selected_location:?}");

    let all_locations_view: Vec<Html> = locations_list
        .iter()
        // TODO: Pass Location selection callback and handle it properly
        .map(|loc| {
            let on_selected_cb = on_selected_cb.clone();
            let selection_settings = SelectionSettings {
                selection_cb: on_selected_cb,
                state: (*loc) == selected_location,
            };
            html!(<LocationView location={loc.clone()} {selection_settings}/>)
        })
        .collect();

    let location_edit_manager =
        LocationEditManager::init(props.app_state.clone(), selected_location_state.clone());

    html!(
        <>
            <div class="box is-rounded is-pinned-to-right-bot">
                <button class="button is-rounded is-info">
                    { location_definer_add_label }
                </button>
            </div>
            <div class="columns">
                <div class="column is-one-third">
                    { all_locations_view }
                </div>
                <div class="column box is-two-thirds">
                    <LocationEdit initial_location_to_edit={selected_location} {location_edit_manager}/>
                </div>
            </div>
        </>
    )
}
