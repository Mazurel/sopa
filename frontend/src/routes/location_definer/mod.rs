use libsopa::locations::Location;
use libsopa::tags::get_all_supported_tags;
use libsopa::yew_components::LocationView;
use libsopa::yew_components::TagView;
use yew::prelude::*;

use crate::app::SharedAppState;

#[derive(Properties, Clone, PartialEq)]
struct LocationEditProps {
    #[prop_or(None)]
    location_to_edit: Option<Location>,
    #[prop_or(None)]
    update_location: Option<Callback<Location>>,
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
                        placeholder={location_definer_title_placeholder}/>
                </div>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{location_definer_address_label}</div>
                <div class="control">
                    <input
                        class={"input"}
                        type={"text"}
                        placeholder={location_definer_address_placeholder}/>
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
                    <button class="button is-link is-light">
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
    pub app_state: SharedAppState<'static>,
}

#[function_component(LocationDefiner)]
pub fn location_definer(props: &LocationDefinerProps) -> Html {
    let location_definer_add_label = t!("location-definer-add-label");

    let all_locations_view: Vec<Html> = props
        .app_state
        .locations_db
        .all_locations()
        .locations_in_random_order()
        .into_iter()
        .map(|loc| html!(<LocationView location={(*loc).clone()}/>))
        .collect();

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
                    <LocationEdit/>
                </div>
            </div>
        </>
    )
}
