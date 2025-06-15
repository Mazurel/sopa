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
use crate::language;
use crate::routes::{
    about::AboutPage, location_definer::LocationDefiner, location_finder::LocationFinder,
    main_page::MainPage,
};

use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    MainPage,
    LocationFinder,
    LocationDefiner,
    About,
}

impl Route {
    pub fn into_route_name(&self) -> Cow<'static, str> {
        match self {
            Route::About => t!("navbar:about"),
            Route::LocationDefiner => t!("navbar:location-definer"),
            Route::LocationFinder => t!("navbar:location-finder"),
            Route::MainPage => t!("navbar:main-page"),
        }
    }

    // NOTE: We force static lifetime here, to simplify lifetime management
    //       in the componenets of the routes - it is always statically allocated.
    pub fn into_html_view(&self, app_state: SharedAppState) -> Html {
        // TODO: Use shared app state in the routes
        match self {
            Route::LocationFinder => html!(<LocationFinder {app_state}/>),
            Route::About => html!(<AboutPage/>),
            Route::LocationDefiner => html!(<LocationDefiner {app_state}/>),
            Route::MainPage => html!(<MainPage/>),
        }
    }
}

static ALL_ROUTES: [Route; 4] = [
    Route::MainPage,
    Route::LocationFinder,
    Route::LocationDefiner,
    Route::About,
];

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
pub struct LanguageSelectionNavigationEntryProps {
    pub reload_ui_cb: Callback<()>,
}

#[function_component(LanguageSelectionNavigationEntry)]
pub fn language_selection_navigation_entry(props: &LanguageSelectionNavigationEntryProps) -> Html {
    let supported_languages = language::list_supported_languages();
    let reload_ui_cb = props.reload_ui_cb.clone();
    let supported_languages_elements = supported_languages
        .iter()
        .map(|language| {
            let reload_ui_cb = reload_ui_cb.clone();
            let language = language.clone();
            let lang_string = language.to_uppercase();
            let emoji = language::get_emoji_for_language(language.as_str());
            let onclick = Callback::from(move |_| {
                let _ = language::set_language(language.clone());
                reload_ui_cb.emit(());
            });

            html!(
                <a class="navbar-item is-hoverable" {onclick}>
                    {format!("{} {}", emoji, lang_string)}
                </a>
            )
        })
        .collect::<Vec<_>>();

    html!(
        <a class="navbar-item is-info has-text-weight-normal is-hoverable has-dropdown">
            <a class="navbar-link">
                {t!("navigation-bar-language-selection")}
            </a>
            <div class="navbar-dropdown">
                {supported_languages_elements}
            </div>
        </a>
    )
}

#[derive(Properties, PartialEq)]
pub struct NavigationBarProps {
    pub on_view_content_update: Callback<Route>,
    pub shared_app_state: SharedAppState,
}

pub struct NavigationBar {
    route: Route,
}

pub enum NavigationMessage {
    ChangeRoute(Route),
    ReloadUI,
}

impl Component for NavigationBar {
    type Message = NavigationMessage;
    type Properties = NavigationBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        NavigationBar {
            route: Route::MainPage,
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
                    on_selection={ctx.link().callback(|route| { NavigationMessage::ChangeRoute(route) })}
                    {is_selected}
                />
            );
            navbar_entries.push(html_entry);
        }

        // Add last entry representing selection of language
        let reload_ui_cb = ctx.link().callback(move |_| NavigationMessage::ReloadUI);
        navbar_entries.push(html!(<LanguageSelectionNavigationEntry {reload_ui_cb} />));

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
                            <div class="navbar-item is-size-1 is-logo-font is-unselectable">
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
            props.on_view_content_update.emit(self.route.clone());
        }
    }

    fn update(&mut self, ctx: &Context<Self>, message: NavigationMessage) -> bool {
        match message {
            NavigationMessage::ReloadUI => {
                info!("Reloading UI");
                let props = ctx.props();
                props.on_view_content_update.emit(self.route.clone());
                true
            }
            NavigationMessage::ChangeRoute(route) => {
                if self.route != route {
                    self.route = route;
                    let props = ctx.props();
                    props.on_view_content_update.emit(self.route.clone());
                    true
                } else {
                    false
                }
            }
        }
    }
}
