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

use app::App;

mod app;
mod color_scheme;
mod cookies;
mod download;
mod footer;
mod language;
mod locations;
mod notifications;
mod routes;
mod window_location;
mod yew_components;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
// Init rust-i18n, refer to lib readme
i18n!("../locales");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    language::init_language_settings();
    color_scheme::init_color_scheme_system();
    yew::Renderer::<App>::new().render();
}
