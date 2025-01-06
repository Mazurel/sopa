use libsopa::locations::{Location, Locations};
use libsopa::tags::Tags;
use libsopa::yew_components::{LocationView, TagPreferenceSelection};
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
struct LocationsViewProps {
    locations: Locations,
    selected_tags: Tags,
}

#[function_component(LocationsView)]
fn locations_view(props: &LocationsViewProps) -> Html {
    let all_locations = props.locations
        .all_locations()
        .iter()
        .map(|l| html!(<LocationView location={l.clone()} selected_tags={props.selected_tags.clone()}/>))
        .collect::<Vec<_>>();

    html!(
        <div class="container box is-max-desktop mt-2">
            {all_locations}
        </div>
    )
}

#[function_component(LocationFinder)]
pub fn location_finder() -> Html {
    let mut sample_locations = Locations::from(vec![
        Location {
            name: "OiK Gdańsk".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female", "age:adult"]),
        },
        Location {
            name: "OiK Gdańsk - Hostel".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female", "type:hostel", "age:adult"]),
        },
        Location {
            name: "OiK Gdynia".to_string(),
            tags: Tags::new_tags(["gender:male", "gender:female", "age:adult", "age:kid"]),
        },
    ]);

    let locations_state = use_state_eq(|| sample_locations.clone());
    let tag_preference_state = use_state_eq(|| Tags::new());

    let on_tag_preference_changed = {
        let locations_state = locations_state.clone();
        let tag_preference_state = tag_preference_state.clone();
        use_callback((), move |tag_preference: Tags, _| {
            let new_locations = locations_state
                .all_locations_in_order(&tag_preference)
                .into();
            locations_state.set(new_locations);
            tag_preference_state.set(tag_preference);
        })
    };

    html! {
        <div class="container">
            <div class="container">
                <TagPreferenceSelection tags={sample_locations.build_tags()} {on_tag_preference_changed}/>
            </div>
            <div class="container">
                <LocationsView locations={(*locations_state).clone()} selected_tags={(*tag_preference_state).clone()}/>
            </div>
        </div>
    }
}
