use yew::prelude::*;

use rust_i18n::t;

struct CompatibleGender {
    male: bool,
    female: bool,
    other: bool,
}

impl Default for CompatibleGender {
    fn default() -> Self {
        CompatibleGender {
            male: false,
            female: false,
            other: false,
        }
    }
}

#[function_component(GenderSelection)]
fn gender_selection() -> Html {
    let gender_selection_state = use_state(|| CompatibleGender::default());

    let gender_text_selection = t!("gender-selection");
    let gender_text_male = t!("gender-male");
    let gender_text_female = t!("gender-female");
    let gender_text_other = t!("gender-other");

    html! {
        <div class="container card is-max-tablet mt-2 p-3 is-shadowless">
            <div class="block has-text-info is-size-3">
                {gender_text_selection}
            </div>
            <div class="columns">
                <div class="container column">
                    <label class="checkbox">
                        <input type="checkbox" class="mr-2" checked={gender_selection_state.male}/>
                        {gender_text_male}
                    </label>
                </div>
                <div class="container column">
                    <label class="checkbox">
                        <input type="checkbox" class="mr-2" checked={gender_selection_state.female}/>
                        {gender_text_female}
                    </label>
                </div>
                <div class="container column">
                    <label class="checkbox">
                        <input type="checkbox" class="mr-2" checked={gender_selection_state.other}/>
                        {gender_text_other}
                    </label>
                </div>
            </div>
        </div>
    }
}

#[function_component(PreferenceSelection)]
fn preference_selection() -> Html {
    html! {
        <div class="container box is-max-tablet mt-2">
            <GenderSelection/>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <PreferenceSelection></PreferenceSelection>
        </div>
    }
}
