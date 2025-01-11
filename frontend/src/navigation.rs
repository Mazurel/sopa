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
use log::info;
use std::borrow::Cow;

use crate::app::SharedAppState;
use crate::routes::{
    about::AboutPage, location_definer::LocationDefiner, location_finder::LocationFinder,
};

use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    LocationFinder,
    LocationDefiner,
    About,
}

impl Route {
    fn into_route_name(&self) -> Cow<'static, str> {
        match self {
            Route::About => t!("navbar:about"),
            Route::LocationDefiner => t!("navbar:location-definer"),
            Route::LocationFinder => t!("navbar:location-finder"),
        }
    }

    // NOTE: We force static lifetime here, to simplify lifetime management
    //       in the componenets of the routes - it is always statically allocated.
    fn into_html_view(&self, app_state: SharedAppState<'static>) -> Html {
        // TODO: Use shared app state in the routes
        match self {
            Route::LocationFinder => html!(<LocationFinder {app_state}/>),
            Route::About => html!(<AboutPage/>),
            Route::LocationDefiner => html!(<LocationDefiner {app_state}/>),
        }
    }
}

static ALL_ROUTES: [Route; 3] = [Route::LocationFinder, Route::LocationDefiner, Route::About];

#[derive(Properties, PartialEq)]
struct NavigationEntryProps {
    route: Route,
    on_selection: Callback<Route>,
    #[prop_or(false)]
    is_selected: bool,
}

#[function_component(NavigationEntry)]
fn naventry(props: &NavigationEntryProps) -> Html {
    let entry_clicked = use_state(|| false);

    let route_name = props.route.into_route_name();
    let entry_clicked_for_cb = entry_clicked.clone();
    let onclick = Callback::from(move |_| {
        entry_clicked_for_cb.set(true);
    });

    if *entry_clicked {
        props.on_selection.emit(props.route);
        entry_clicked.set(false);
    }

    html!(
        if props.is_selected {
            <a class="navbar-item has-text-weight-semibold" {onclick}>
                {route_name}
            </a>
        }
        else {
            <a class="navbar-item is-primary has-text-weight-normal" {onclick}>
                {route_name}
            </a>
        }
    )
}

#[derive(Properties, PartialEq)]
pub struct NavigationBarProps<'a> {
    pub on_view_content_update: Callback<Html>,
    pub shared_app_state: SharedAppState<'a>,
}

pub struct NavigationBar {
    route: Route,
}

impl Component for NavigationBar {
    type Message = Route;
    type Properties = NavigationBarProps<'static>;

    fn create(_ctx: &Context<Self>) -> Self {
        NavigationBar {
            route: Route::LocationFinder,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut navbar_entries: Vec<Html> = Vec::new();
        navbar_entries.reserve(ALL_ROUTES.len());

        for entry in ALL_ROUTES {
            let is_selected: bool = self.route.eq(&entry);
            let html_entry = html!(
                <NavigationEntry
                    route={entry}
                    on_selection={ctx.link().callback(|route| {
                        info!("New navigation route: {route:?}");
                        route
                    })}
                    {is_selected}
                />
            );
            navbar_entries.push(html_entry);
        }

        html!(
            <div class="">
                <nav class="navbar is-info mb-6" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">
                        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                            <span aria-hidden="false"></span>
                            <span aria-hidden="false"></span>
                            <span aria-hidden="false"></span>
                            <span aria-hidden="false"></span>
                        </a>
                    </div>
                    <div class="navbar-menu">
                        <div class="navbar-start">
                            <div class="navbar-item is-size-3 is-logo-font">
                                {"SOPa"}
                            </div>
                        </div>
                        <div class="navbar-end">
                            {navbar_entries}
                        </div>
                    </div>
                </nav>
            </div>
        )
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let props = ctx.props();
            props
                .on_view_content_update
                .emit(self.route.into_html_view(props.shared_app_state.clone()));
        }
    }

    fn update(&mut self, ctx: &Context<Self>, route: Route) -> bool {
        if self.route != route {
            self.route = route;
            let props = ctx.props();
            props
                .on_view_content_update
                .emit(self.route.into_html_view(props.shared_app_state.clone()));
            true
        } else {
            false
        }
    }
}
