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

use log;
use std::collections::HashMap;

use gloo::utils::window;
use yew::prelude::*;

use crate::footer::Footer;
use crate::locations::LocationsDatabase;
use crate::notifications::NotificationManager;
use crate::yew_components::navigation::{NavigationBar, Route};

#[derive(Properties, Clone, PartialEq)]
pub struct SharedAppState {
    pub locations_db: UseStateHandle<LocationsDatabase>,
    pub notifications: NotificationManager,
}

#[function_component(App)]
pub fn app() -> Html {
    let selected_route: UseStateHandle<Option<Route>> = use_state(|| None);
    let locations_db = use_state(move || LocationsDatabase::load_default_database());

    let notifications = use_state(|| HashMap::new());

    let shared_app_state = {
        let notifications_counter = use_state(|| 0);
        let notifications = notifications.clone();
        use_state(move || {
            let notification_manager = NotificationManager {
                notifications,
                notifications_counter,
            };
            SharedAppState {
                locations_db,
                notifications: notification_manager,
            }
        })
    };

    let on_view_content_update = {
        let selected_route = selected_route.clone();
        Callback::from(move |route: Route| {
            selected_route.set(Some(route));
        })
    };

    let view_content = match *selected_route {
        None => html!(
            <div>
                {"⚒️ Loading page ⚒️"}
            </div>
        ),
        Some(route) => route.into_html_view((*shared_app_state).clone()),
    };

    html! {
        <div>
            <div class="is-primary">
                <NavigationBar {on_view_content_update} shared_app_state={(*shared_app_state).clone()}/>
            </div>
            <div class="container">
                {view_content}
            </div>
            <div id="notification-container" class="container">
                {
                    (*notifications)
                        .clone()
                        .into_iter()
                        .map(|(_, notification)| {
                            notification
                        }).collect::<Vec<_>>()
                }
            </div>
            <Footer/>
        </div>
    }
}

pub fn reload() {
    if let Err(err) = window().location().reload() {
        log::error!("Failed to reload page: {:?}", err);
    }
}
