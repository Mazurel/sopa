use std::ops::Deref;

use libsopa::locations::Location;
use libsopa::tags::get_all_supported_tags;
use libsopa::yew_components::LocationView;
use libsopa::yew_components::TagView;
use yew::prelude::*;

use crate::app::SharedAppState;

#[derive(Properties, Clone, PartialEq)]
struct LocationEditProps {
    #[prop_or(Location::default())]
    location_to_edit: Location,
    update_location_cb: Callback<Location>,
}

#[function_component(LocationEdit)]
fn location_edit(props: &LocationEditProps) -> Html {
    let location_definer_prompt = t!("location-definer-prompt");
    let location_definer_title_label = t!("location-definer-title-label");
    let location_definer_title_placeholder = t!("location-definer-title-placeholder");
    let location_definer_address_label = t!("location-definer-address-label");
    let location_definer_address_placeholder = t!("location-definer-address-placeholder");
    let location_definer_tags_label = t!("location-definer-tags-label");
    let location_definer_button_save = t!("location-definer-button-save");
    let location_definer_button_reset = t!("location-definer-button-reset");
    let location_definer_button_exit = t!("location-definer-button-exit");

    let location_state = {
        let location_to_edit = props.location_to_edit.clone();
        use_state(move || location_to_edit.clone())
    };

    let change_title: Callback<_> = {
        let location_state = location_state.clone();
        Callback::from(move |event: Event| {
            if let Some(val) = event.as_string() {
                let mut location = location_state.deref().clone();
                location.name = val;
                location_state.set(location);
            }
        })
    };

    let change_address: Callback<_> = {
        let location_state = location_state.clone();
        Callback::from(move |event: Event| {
            if let Some(val) = event.as_string() {
                let mut location = location_state.deref().clone();
                location.address = val;
                location_state.set(location);
            }
        })
    };

    let button_save_on_click: Callback<MouseEvent> = {
        let update_location = props.update_location_cb.clone();
        let location_state = location_state.clone();
        Callback::from(move |_| update_location.emit((*location_state).clone()))
    };

    let all_tags_view: Vec<Html> = get_all_supported_tags()
        .into_iter()
        .map(|tag| html!(<TagView {tag} interactive={false}/>))
        .collect();

    html!(
        <>
            <div class="content">
                <h2>{location_definer_prompt}</h2>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{location_definer_title_label}</div>
                <div class="control">
                    <input
                        class={"input"}
                        type={"text"}
                        placeholder={location_definer_title_placeholder}
                        onchange={change_title}
                        />
                </div>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{location_definer_address_label}</div>
                <div class="control">
                    <input
                        class={"input"}
                        type={"text"}
                        placeholder={location_definer_address_placeholder}
                        onchange={change_address}
                        />
                </div>
            </div>
            <div class="field">
                <div class="label">{location_definer_tags_label}</div>
                <div class="control">
                    <div>{all_tags_view}</div>
                </div>
            </div>
            <div class="field has-addons mt-5">
                <div class="control">
                    <button class="button is-link is-light" onclick={button_save_on_click}>
                        {location_definer_button_save}
                    </button>
                </div>
                <div class="control">
                    <button class="button is-warning is-light">
                        {location_definer_button_reset}
                    </button>
                </div>
                <div class="control">
                    <button class="button is-danger is-light">
                        {location_definer_button_exit}
                    </button>
                </div>
            </div>
        </>
    )
}

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

    {
        // In this block we re-fetch data and change state if needed.
        let selected_location_state = selected_location_state.clone();
        locations_db.use_locations(move |locations_db| {
            match locations_db.fetch_update(&selected_location_state) {
                Some(new_location) => {
                    selected_location_state.set(new_location);
                }
                None => {}
            }
        });
    }
    let selected_location: Location = selected_location_state.deref().clone();

    let all_locations_view: Vec<Html> = locations_list
        .iter()
        .map(|loc| html!(<LocationView location={loc.clone()}/>))
        .collect();

    let update_location_cb = {
        let selected_location_state = selected_location_state.clone();
        use_callback((), move |new_location, _| {
            selected_location_state.set(new_location);
        })
    };

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
                    <LocationEdit location_to_edit={selected_location} {update_location_cb}/>
                </div>
            </div>
        </>
    )
}
