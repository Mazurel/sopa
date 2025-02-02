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
pub struct SharedAppState {
    pub locations_db: UseStateHandle<LocationsDatabase>,
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

    let shared_app_state = use_state(|| SharedAppState { locations_db });

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
