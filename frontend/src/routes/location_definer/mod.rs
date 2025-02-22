use std::ops::Deref;

use libsopa::locations::Location;
use libsopa::tags::get_all_supported_tags;
use libsopa::yew_components::LocationView;
use libsopa::yew_components::SelectionSettings;
use libsopa::yew_components::TagView;
use log::info;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
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

    // We always want to re-fetch props
    let location_to_edit = props.location_to_edit.clone();

    let change_title: Callback<_> = {
        let location_to_edit = location_to_edit.clone();
        let update_location_cb = props.update_location_cb.clone();
        Callback::from(move |event: Event| {
            let maybe_input_element = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = maybe_input_element {
                let update_location_cb = update_location_cb.clone();
                let mut location = location_to_edit.clone();
                location.name = input.value();
                update_location_cb.emit(location);
            }
        })
    };

    let change_address: Callback<_> = {
        let location_to_edit = location_to_edit.clone();
        let update_location_cb = props.update_location_cb.clone();
        Callback::from(move |event: Event| {
            let maybe_input_element = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = maybe_input_element {
                let update_location_cb = update_location_cb.clone();
                let mut location = location_to_edit.clone();
                location.address = input.value();
                update_location_cb.emit(location);
            }
        })
    };

    let button_save_on_click: Callback<MouseEvent> = {
        // TODO: Commit changes
        Callback::from(move |_| {})
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
                        value={location_to_edit.name.clone()}
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

    let on_selected_cb = {
        let selected_location_state = selected_location_state.clone();
        Callback::from(move |location: Location| {
            selected_location_state.set(location);
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

    let update_location_cb = {
        let selected_location_state = selected_location_state.clone();
        let locations_db: UseStateHandle<_> = locations_db.clone();
        Callback::from(move |new_location: Location| {
            info!("New Location: {new_location:?}");
            {
                // Here we first need to indicate to the DB that location was changed,
                // then we can update state of the component.
                let mut locations_db = locations_db.deref().clone();
                let new_location = new_location.clone();
                locations_db.use_locations_mut(move |locations| {
                    locations.push_update(new_location);
                });
            }
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
