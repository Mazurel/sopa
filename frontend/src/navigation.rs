use log::info;
use std::borrow::Cow;

use crate::routes::{
    about::AboutPage, location_definer::LocationDefiner, location_finder::LocationFinder,
};

use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Route {
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

    fn into_html_view(&self) -> Html {
        match self {
            Route::LocationFinder => html!(<LocationFinder/>),
            Route::About => html!(<AboutPage/>),
            Route::LocationDefiner => html!(<LocationDefiner/>),
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

    if props.is_selected {
        html!(
            <a class="navbar-item has-text-weight-semibold" {onclick}>
                {route_name}
            </a>
        )
    } else {
        html!(
            <a class="navbar-item is-primary" {onclick}>
                {route_name}
            </a>
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct NavigationBarProps {
    pub on_view_content_update: Callback<Html>,
}

#[function_component(NavigationBar)]
pub fn navbar(props: &NavigationBarProps) -> Html {
    let selected_route = use_state_eq(|| Route::LocationFinder);
    let selected_route_clone = selected_route.clone();
    let on_route_selection_changed = Callback::from(move |route: Route| {
        info!("New navigation route: {route:?}");
        selected_route_clone.set(route);
    });

    let mut navbar_entries: Vec<Html> = Vec::new();
    navbar_entries.reserve(ALL_ROUTES.len());

    for entry in ALL_ROUTES {
        let is_selected: bool = (*selected_route).eq(&entry);
        let html_entry = html!(<NavigationEntry route={entry} on_selection={on_route_selection_changed.clone()} {is_selected}/>);
        navbar_entries.push(html_entry);
    }

    props
        .on_view_content_update
        .emit(selected_route.into_html_view());

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
