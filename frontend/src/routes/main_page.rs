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

#[function_component(MainPage)]
pub fn main_page() -> Html {
    html!(
        <>
            <section class="hero is-small is-warning">
                <div class="hero-body">
                    <div class="notification is-warning has-text-centered">
                        <p class="block is-size-5 has-text-weight-bold">
                            {t!("main-page:alpha-warning-title")}
                        </p>
                        <p class="block is-size-6">
                            {t!("main-page:alpha-warning-message")}
                        </p>
                    </div>
                </div>
            </section>
            <section class="hero is-medium is-primary  is-logo-font">
                <div class="hero-body">
                    <h1 class="block is-size-1 has-text-centered has-text-weight-bold">
                        {t!("main-page:welcome-title")}
                    </h1>
                </div>
            </section>
            <section class="hero is-small is-info mt-2">
                <div class="hero-body">
                    <p class="block is-size-4 has-text-centered">
                        {t!("main-page:welcome-message")}
                    </p>
                    <p class="block is-size-4 has-text-centered">
                        {t!("main-page:help-message")}
                    </p>
                    <p class="block is-size-4 has-text-centered">
                        {t!("main-page:support-message")}
                    </p>
                </div>
            </section>
        </>
    )
}
