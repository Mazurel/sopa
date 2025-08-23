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

mod tag_selection;

use crate::yew_components::LocationView;
use libsopa::locations::Location;
use libsopa::tags::Tags;
use tag_selection::TagPreferenceSelection;
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
        .map(|l| html!(<LocationView location={l.clone()} global_selected_tags={props.selected_tags.clone()} simplified_view={false}/>))
        .collect::<Vec<_>>();

    html!(
        <div class="container box mt-2">
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
    let locations_in_order_state = {
        let locations = fetch_all_locations(&props.app_state.locations_db);
        use_state_eq(move || locations)
    };
    let tag_preference_state = use_state_eq(|| Tags::new());

    {
        let locations_db = props.app_state.locations_db.clone();
        let locations_state = locations_in_order_state.clone();
        let tag_preference_state = tag_preference_state.clone();
        use_effect(move || {
            locations_db.use_locations(move |locations| {
                let new_locations: Vec<Location> = locations
                    .all_locations_in_order(&tag_preference_state)
                    .into();
                locations_state.set(new_locations);
            });
        });
    }

    let on_tag_preference_changed = {
        let tag_preference_state = tag_preference_state.clone();
        Callback::from(move |tag_preference: Tags| {
            tag_preference_state.set(tag_preference);
        })
    };

    html! {
        <div class="block">
            <TagPreferenceSelection {on_tag_preference_changed}/>
            <div class="container">
                <LocationsView locations={(*locations_in_order_state).clone()} selected_tags={(*tag_preference_state).clone()}/>
            </div>
        </div>
    }
}
