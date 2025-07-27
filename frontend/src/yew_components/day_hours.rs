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

use super::timepicker::Timepicker;
use libsopa::time::{Day, TimePoint, TimeSpan};
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn init_default_time_span() -> TimeSpan {
    TimeSpan {
        #[rustfmt::skip]
        from: TimePoint {
            hour: 9,
            minute: 0
        },
        #[rustfmt::skip]
        to: TimePoint {
            hour: 17,
            minute: 0,
        },
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct DayHoursEditProps {
    pub day: Day,
    pub time_span: Option<TimeSpan>,
    pub on_time_span_changed: Callback<Option<TimeSpan>>,
}

#[function_component(DayHoursEdit)]
pub fn day_hours_edit(props: &DayHoursEditProps) -> Html {
    let is_open = props.time_span.is_some();
    let current_time_span = props
        .time_span
        .clone()
        .unwrap_or_else(init_default_time_span);

    let toggle_day_enabled = {
        let on_time_span_changed = props.on_time_span_changed.clone();
        let current_time_span = current_time_span.clone();
        Callback::from(move |event: Event| {
            if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
                if input.checked() {
                    on_time_span_changed.emit(Some(current_time_span.clone()));
                } else {
                    on_time_span_changed.emit(None);
                }
            }
        })
    };

    let change_opening_time = {
        let on_time_span_changed = props.on_time_span_changed.clone();
        let current_time_span = current_time_span.clone();
        Callback::from(move |new_time: TimePoint| {
            let new_time_span = TimeSpan {
                from: new_time,
                to: current_time_span.to.clone(),
            };
            on_time_span_changed.emit(Some(new_time_span));
        })
    };

    let change_closing_time = {
        let on_time_span_changed = props.on_time_span_changed.clone();
        let current_time_span = current_time_span.clone();
        Callback::from(move |new_time: TimePoint| {
            let new_time_span = TimeSpan {
                from: current_time_span.from.clone(),
                to: new_time,
            };
            on_time_span_changed.emit(Some(new_time_span));
        })
    };

    html! {
        <div class="field is-grouped is-max-tablet">
            <div style="width: 1em" class="block"/>
            <div class="control is-centered-vertically-in-parent">
                <input
                    style="vertical-align: middle;"
                    class="checkbox"
                    type="checkbox"
                    checked={is_open}
                    onchange={toggle_day_enabled}
                    />
            </div>
            <div class="label is-centered-vertically-in-parent">{props.day.to_display_name()}</div>
            // NOTE: This is expanded control, to align everything to the right
            <div class="control is-expanded"/>
            <div class="label is-centered-vertically-in-parent">{t!("day-open")}</div>
            <Timepicker
                on_time_changed={change_opening_time}
                disabled={!is_open}
                current_time={current_time_span.from}
                />
            <div class="label is-centered-vertically-in-parent">{t!("day-closed")}</div>
            <Timepicker
                on_time_changed={change_closing_time}
                disabled={!is_open}
                current_time={current_time_span.to}
                />
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct DayHoursViewProps {
    pub day: Day,
    pub time_span: Option<TimeSpan>,
}

#[function_component(DayHoursView)]
pub fn day_hours_view(props: &DayHoursViewProps) -> Html {
    let time_span = &props.time_span;
    let day_name = props.day.to_display_name();

    match time_span {
        None => html!(<> </>),
        Some(time_span) => html! {
            <div class="field is-grouped is-max-tablet">
                <div class="control"/>
                <div class="control">
                    <span class="tag is-medium">{time_span.from.to_time_string()}</span>
                </div>
                <div class="label is-centered-vertically-in-parent">{" : "}</div>
                <div class="control">
                    <span class="tag is-medium">{time_span.to.to_time_string()}</span>
                </div>
                <div class="label is-centered-vertically-in-parent">
                    <span>{day_name}</span>
                </div>
            </div>
        },
    }
}
