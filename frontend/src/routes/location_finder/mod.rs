use crate::yew_components::{LocationView, TagPreferenceSelection};
use libsopa::locations::Location;
use libsopa::tags::Tags;
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
    pub app_state: SharedAppState,
}

fn fetch_all_locations(db: &crate::locations::LocationsDatabase) -> Vec<Location> {
    let mut locs = Vec::new();
    db.use_locations(|locations| {
        locs.append(&mut locations.locations_in_random_order());
    });
    locs
}

#[function_component(LocationFinder)]
pub fn location_finder(props: &LocationFinderProps) -> Html {
    let locations = fetch_all_locations(&props.app_state.locations_db);
    let locations_in_order_state = {
        let locations = locations.clone();
        use_state(move || locations)
    };

    let tag_preference_state = use_state_eq(|| Tags::new());

    let on_tag_preference_changed = {
        let locations_state = locations_in_order_state.clone();
        let tag_preference_state = tag_preference_state.clone();
        let locations_db = props.app_state.locations_db.clone();
        Callback::from(move |tag_preference: Tags| {
            locations_db.use_locations(|locations| {
                let new_locations: Vec<Location> =
                    locations.all_locations_in_order(&tag_preference).into();
                locations_state.set(new_locations);
                tag_preference_state.set(tag_preference);
            });
        })
    };

    let tags = use_memo((), |_| Tags::from(libsopa::tags::get_all_supported_tags()));

    html! {
        <div class="container">
            <div class="container">
                <TagPreferenceSelection tags={(*tags).clone()} {on_tag_preference_changed}/>
            </div>
            <div class="container">
                <LocationsView locations={(*locations_in_order_state).clone()} selected_tags={(*tag_preference_state).clone()}/>
            </div>
        </div>
    }
}
