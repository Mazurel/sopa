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
use yew::prelude::*;

use crate::locations::LocationsDatabase;
use crate::navigation::NavigationBar;

#[derive(Properties, Clone, PartialEq)]
pub struct SharedAppState<'a> {
    pub locations_db: UseStateHandle<LocationsDatabase<'a>>,
    pub locations_db_update_request: Callback<LocationsDatabase<'a>>,
}

impl<'a> SharedAppState<'a> {
    pub fn map_locations_db_state<F>(&'a self, mapping_function: F)
    where
        F: FnOnce(LocationsDatabase) -> LocationsDatabase,
    {
        let current_locations_db = (*self.locations_db).clone();
        let locations_db = mapping_function(current_locations_db.clone());
        if locations_db != current_locations_db {
            self.locations_db_update_request.emit(locations_db);
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let view_content = use_state_eq(|| {
        html!(
            <div>
                {"⚒️ Loading page ⚒️"}
            </div>
        )
    });
    let view_content_clone = view_content.clone();
    let on_view_content_update =
        use_callback((), move |html: Html, _| view_content_clone.set(html));

    let locations_db = use_state(|| LocationsDatabase::new_with_samples());
    let locations_db_update_request = {
        let locations_db = locations_db.clone();
        use_callback((), move |new_locations_db, _| {
            locations_db.set(new_locations_db);
        })
    };

    let shared_app_state = use_state(|| SharedAppState {
        locations_db,
        locations_db_update_request,
    });

    html! {
        <div>
            <div class="is-primary">
                <NavigationBar {on_view_content_update} shared_app_state={(*shared_app_state).clone()}/>
            </div>
            <div class="container">
                {(*view_content).clone()}
            </div>
        </div>
    }
}
