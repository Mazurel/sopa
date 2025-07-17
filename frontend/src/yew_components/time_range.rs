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

use log::*;
use yew::prelude::*;

use libsopa::locations::TimePoint;

#[derive(Properties, PartialEq, Clone)]
pub struct TimeSelectProps {
    pub on_time_update: Callback<TimePoint>,
}

#[function_component(TimeSelect)]
pub fn time_select(props: &TimeSelectProps) -> Html {
    let TimeSelectProps { on_time_update } = props;

    let oninput = {
        Callback::from(move |event: InputEvent| {
            info!("{event:?}");
        })
    };

    html! {
        <div>
            <input type="time" {oninput} />
        </div>
    }
}
