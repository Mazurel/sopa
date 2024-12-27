use crate::tags::{TagPreferenceSelection, Tags};
use yew::prelude::*;

struct Location {
    name: String,
    tags: Tags,
}

#[function_component(PreferenceSelection)]
fn preference_selection() -> Html {
    html! {
        <div class="container box is-max-tablet mt-2">
            <TagPreferenceSelection/>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let sample_locations = vec![Location {
        name: "OiK Gdańsk".to_string(),
        tags: Tags::new_tags(["gender:male", "gender:female"]),
    }];

    html! {
        <div class="container">
            <PreferenceSelection></PreferenceSelection>
        </div>
    }
}
