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

use super::day_hours::DayHoursEdit;
use libsopa::time::{Day, OpenedHours, TimeSpan};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct OpenedHoursEditProps {
    pub opened_hours: OpenedHours,
    pub on_opened_hours_changed: Callback<OpenedHours>,
}

#[function_component(OpenedHoursEdit)]
pub fn opened_hours_edit(props: &OpenedHoursEditProps) -> Html {
    let day_components: Vec<Html> = Day::get_all_days_in_week()
        .into_iter()
        .map(|day| {
            let day_clone = day.clone();
            let time_span = props
                .opened_hours
                .get_time_span_per_day()
                .get(&day)
                .cloned();
            let on_time_span_changed = {
                let on_opened_hours_changed = props.on_opened_hours_changed.clone();
                let opened_hours = props.opened_hours.clone();
                Callback::from(move |new_time_span: Option<TimeSpan>| {
                    let mut new_opened_hours = opened_hours.clone();
                    match new_time_span {
                        Some(span) => {
                            new_opened_hours.set_day_time_span(day_clone.clone(), span);
                        }
                        None => {
                            new_opened_hours.remove_day(&day_clone);
                        }
                    }
                    on_opened_hours_changed.emit(new_opened_hours);
                })
            };

            html! {
                <DayHoursEdit
                    day={day}
                    time_span={time_span}
                    on_time_span_changed={on_time_span_changed}
                />
            }
        })
        .collect();

    html! {
        <div class="block pb-2">
            <div class="block">
                <span class="is-size-5">{"Open Hours"}</span>
            </div>
            <div class="block pl-2 pr-6">
                { day_components }
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct OpenedHoursViewProps {
    pub opened_hours: OpenedHours,
}

#[function_component(OpenedHoursView)]
pub fn opened_hours_view(props: &OpenedHoursViewProps) -> Html {
    let day_components: Vec<Html> = Day::get_all_days_in_week()
        .into_iter()
        .filter_map(|day| {
            let is_opened = props.opened_hours.is_opened_on_day(&day);
            if is_opened {
                let time_span = props
                    .opened_hours
                    .get_time_span_per_day()
                    .get(&day)
                    .cloned();

                Some(html! {
                    <super::day_hours::DayHoursView
                        day={day}
                        time_span={time_span}
                    />
                })
            } else {
                None
            }
        })
        .collect();

    html! {
        <div class="block mt-5 pl-6 pr-6">
            { day_components }
        </div>
    }
}
