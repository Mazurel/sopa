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
    let location_definer_add_label = t!("location-definer-add-label");

    let locations_list = {
        let locations_db = &props.app_state.locations_db;
        use_state(|| fetch_all_locations(locations_db))
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
                info!("Updated list!");
            },
        );
    }

    let location_edit_manager =
        LocationEditManager::init(props.app_state.clone(), selected_location_state.clone());

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
            <div class="box is-rounded is-pinned-to-right-bot" onclick={on_new_location_request_cb}>
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
