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

use crate::{app::SharedAppState, yew_components::navigation::Route};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainPageProps {
    pub app_state: SharedAppState,
}

#[function_component]
pub fn MainPage(props: &MainPageProps) -> Html {
    let current_route_state = props.app_state.current_route_state.clone();

    html!(
        <div class="main-page">
            // Alpha Warning Banner
            <section class="section pb-3">
                <div class="container">
                    <div class="warning-banner notification is-warning has-text-centered mb-6">
                        <p class="is-size-5 has-text-weight-bold mb-2">
                            <span class="icon-text">
                                <span>{t!("main-page:alpha-warning-title")}</span>
                            </span>
                        </p>
                        <p class="is-size-6">
                            {t!("main-page:alpha-warning-message")}
                        </p>
                    </div>
                </div>
            </section>

            // Hero Section with Title
            <section class="hero is-medium">
                <div class="hero-body hero-gradient">
                    <div class="container has-text-centered">
                        <h1 class="title is-1 is-spaced welcome-title is-logo-font">
                            {t!("main-page:welcome-title")}
                        </h1>
                        <div class="subtitle is-4 has-text-white-ter mt-4">
                            {"🤝 "}
                            {t!("main-page:support-message")}
                        </div>
                    </div>
                </div>
            </section>

            // Main Content Section
            <section class="section py-6">
                <div class="container">
                    <div class="content-card p-6 mb-6">
                        <div class="columns is-vcentered">
                            <div class="column is-8">
                                <div class="content">
                                    <p class="is-size-4 mb-4">
                                        {t!("main-page:welcome-message")}
                                    </p>
                                    <p class="is-size-5 mb-0">
                                        {t!("main-page:help-message")}
                                    </p>
                                </div>
                            </div>
                            <div class="column is-4 has-text-centered">
                                <div class="feature-icon support-icon">
                                    {"🏥"}
                                </div>
                            </div>
                        </div>
                    </div>

                    // Feature Cards
                    <div class="columns is-multiline">
                        <div class="column is-4">
                            <div class="box feature-card has-text-centered">
                                <div class="feature-icon help-icon">
                                    {"🔍"}
                                </div>
                                <h3 class="title is-5 mb-3">{t!("main-page:feature-location-finder-title")}</h3>
                                <p>
                                    {t!("main-page:feature-location-finder-description")}
                                </p>
                            </div>
                        </div>
                        <div class="column is-4">
                            <div class="box feature-card has-text-centered">
                                <div class="feature-icon community-icon">
                                    {"🏷️"}
                                </div>
                                <h3 class="title is-5 mb-3">{t!("main-page:feature-smart-filtering-title")}</h3>
                                <p>
                                    {t!("main-page:feature-smart-filtering-description")}
                                </p>
                            </div>
                        </div>
                        <div class="column is-4">
                            <div class="box feature-card has-text-centered">
                                <div class="feature-icon support-icon">
                                    {"💾"}
                                </div>
                                <h3 class="title is-5 mb-3">{t!("main-page:feature-data-management-title")}</h3>
                                <p>
                                    {t!("main-page:feature-data-management-description")}
                                </p>
                            </div>
                        </div>
                    </div>

                    // Call to Action
                    <div class="cta-section p-6 has-text-centered mt-6">
                        <h2 class="title is-3 mb-4">
                            {t!("main-page:cta-title")}
                        </h2>
                        <p class="subtitle is-5 mb-5">
                            {t!("main-page:cta-subtitle")}
                        </p>
                        <div class="buttons is-centered">
                            <button
                                class="cta-button"
                                onclick={Callback::from(move |_| {
                                    current_route_state.set(Some(Route::LocationFinder));
                                })}
                            >
                                <span class="icon-text">
                                    <span class="icon">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <span>{t!("main-page:cta-button")}</span>
                                </span>
                            </button>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    )
}
