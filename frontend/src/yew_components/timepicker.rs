use libsopa::time::TimePoint;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TimepickerProps {
    pub on_time_changed: Callback<TimePoint>,
    pub disabled: bool,
    pub current_time: TimePoint,
}

#[function_component(Timepicker)]
pub fn timepicker(props: &TimepickerProps) -> Html {
    let TimepickerProps {
        disabled,
        on_time_changed,
        current_time,
    } = props;
    let hours_state = use_state_eq(|| current_time.hour);
    let minutes_state = use_state_eq(|| current_time.minute);

    {
        let hours_state = hours_state.clone();
        let minutes_state = minutes_state.clone();

        use_effect_with(current_time.clone(), move |current_time| {
            hours_state.set(current_time.hour);
            minutes_state.set(current_time.minute);
        });
    }

    {
        let on_time_changed = on_time_changed.clone();

        use_effect_with(
            (hours_state.clone(), minutes_state.clone()),
            move |(hours, minutes)| {
                let new_time = TimePoint {
                    hour: **hours,
                    minute: **minutes,
                };
                on_time_changed.emit(new_time);
            },
        );
    }

    let on_hours_changed = {
        let hours_state = hours_state.clone();

        Callback::from(move |event: Event| {
            if let Some(target) = event.target() {
                let target = target.unchecked_into::<HtmlInputElement>();
                if let Ok(hours) = target.value().parse::<u8>() {
                    hours_state.set(hours);
                }
            }
        })
    };

    let on_minutes_changed = {
        let minutes_state = minutes_state.clone();

        Callback::from(move |event: Event| {
            if let Some(target) = event.target() {
                let target = target.unchecked_into::<HtmlInputElement>();
                if let Ok(minutes) = target.value().parse::<u8>() {
                    minutes_state.set(minutes);
                }
            }
        })
    };

    let hours = format!("{:02}", *hours_state);
    let minutes = format!("{:02}", *minutes_state);

    html! {
          <div class="timepicker" dir="ltr">
            <input
                type="text"
                class="hours"
                disabled={*disabled}
                min="0" max="23"
                placeholder="hh"
                maxlength="2"
                value={hours}
                onchange={on_hours_changed} />
            <input
                type="text"
                class="minutes"
                disabled={*disabled}
                min="0" max="59"
                placeholder="mm"
                maxlength="2"
                value={minutes}
                onchange={on_minutes_changed} />
        </div>
    }
}
