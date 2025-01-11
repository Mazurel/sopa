use libsopa::locations::Location;
use libsopa::tags::Tags;
use libsopa::yew_components::{LocationView, TagPreferenceSelection};
use yew::prelude::*;

use crate::app::SharedAppState;

#[derive(Properties, PartialEq, Eq)]
struct LocationsViewProps {
    locations: Vec<Location>,
    selected_tags: Tags,
}

#[function_component(LocationsView)]
fn locations_view(props: &LocationsViewProps) -> Html {
    let all_locations = props.locations
        .iter()
        .map(|l| html!(<LocationView location={l.clone()} global_selected_tags={props.selected_tags.clone()}/>))
        .collect::<Vec<_>>();

    html!(
        <div class="container box is-max-desktop mt-2">
            {all_locations}
        </div>
    )
}

#[derive(Properties, Clone, PartialEq)]
pub struct LocationFinderProps {
    pub app_state: SharedAppState<'static>,
}

#[function_component(LocationFinder)]
pub fn location_finder(props: &LocationFinderProps) -> Html {
    let locations = props.app_state.locations_db.all_locations().clone();
    let locations_in_order_state = use_state(|| locations.locations_in_random_order().clone());

    let tag_preference_state = use_state_eq(|| Tags::new());

    let on_tag_preference_changed = {
        let locations_state = locations_in_order_state.clone();
        let tag_preference_state = tag_preference_state.clone();
        let locations = locations.clone();
        use_callback((), move |tag_preference: Tags, _| {
            let new_locations: Vec<Location> =
                locations.all_locations_in_order(&tag_preference).into();
            locations_state.set(new_locations);
            tag_preference_state.set(tag_preference);
        })
    };

    html! {
        <div class="container">
            <div class="container">
                <TagPreferenceSelection tags={locations.build_tags()} {on_tag_preference_changed}/>
            </div>
            <div class="container">
                <LocationsView locations={(*locations_in_order_state).clone()} selected_tags={(*tag_preference_state).clone()}/>
            </div>
        </div>
    }
}
