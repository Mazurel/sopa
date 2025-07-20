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

use super::location_edit_manager::LocationEditManager;
use super::tags_selection::TagsSelectionEditForLocation;
use crate::yew_components::opened_hours::OpenedHoursEdit;
use crate::yew_components::ContactMethodsEdit;
use libsopa::contact::ContactMethods;
use libsopa::locations::Location;
use libsopa::time::OpenedHours;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

fn edit_control_buttons(
    save_on_click: Callback<MouseEvent>,
    clear_on_click: Callback<MouseEvent>,
    remove_on_click: Callback<MouseEvent>,
) -> Html {
    html!(
        <div class="field has-addons mt-5">
            <div class="control">
                <button class="button is-link is-light" onclick={save_on_click}>
                    {t!("location-definer-button-save")}
                </button>
            </div>
            <div class="control">
                <button class="button is-warning is-light" onclick={clear_on_click}>
                    {t!("location-definer-button-reset")}
                </button>
            </div>
            <div class="control">
                <button class="button is-danger is-light" onclick={remove_on_click}>
                    {t!("location-definer-button-remove")}
                </button>
            </div>
        </div>
    )
}

#[derive(Properties, Clone, PartialEq)]
pub struct LocationEditProps {
    #[prop_or(Location::default())]
    pub initial_location_to_edit: Location,
    pub location_edit_manager: LocationEditManager,
}

#[function_component(LocationEdit)]
pub fn location_edit(props: &LocationEditProps) -> Html {
    let change_title: Callback<_> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |event: Event| {
            let maybe_input_element = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = maybe_input_element {
                let mut location = location_edit_manager.get_location_under_edit();
                location.name = input.value();
                location_edit_manager.stage_location_changes(location);
            }
        })
    };

    let change_address: Callback<_> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |event: Event| {
            let maybe_input_element = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = maybe_input_element {
                let mut location = location_edit_manager.get_location_under_edit();
                location.address = input.value();
                location_edit_manager.stage_location_changes(location);
            }
        })
    };

    let change_description: Callback<_> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |event: InputEvent| {
            let maybe_text_area_element = event.target_dyn_into::<HtmlTextAreaElement>();
            if let Some(text_area) = maybe_text_area_element {
                let mut location = location_edit_manager.get_location_under_edit();
                location.description = text_area.value();
                location_edit_manager.stage_location_changes(location);
            }
        })
    };

    let change_contact_methods: Callback<ContactMethods> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |new_contact_methods| {
            let mut location = location_edit_manager.get_location_under_edit();
            location.contact_methods = new_contact_methods;
            location_edit_manager.stage_location_changes(location);
        })
    };

    let change_opened_hours: Callback<OpenedHours> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |new_opened_hours| {
            let mut location = location_edit_manager.get_location_under_edit();
            location.opened_hours = new_opened_hours;
            location_edit_manager.stage_location_changes(location);
        })
    };

    let button_save_on_click: Callback<MouseEvent> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |_| {
            location_edit_manager.commit_location_changes();
        })
    };

    let button_clear_on_click: Callback<MouseEvent> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |_| {
            location_edit_manager.clear_location_changes();
        })
    };

    let button_remove_on_click: Callback<MouseEvent> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |_| {
            location_edit_manager.remove_current_location();
        })
    };

    let location_to_edit = props.initial_location_to_edit.clone();
    let description = location_to_edit.description.clone();

    html!(
        <>
            <div class="content">
                <h2>{t!("location-definer-prompt")}</h2>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{t!("location-definer-title-label")}</div>
                <div class="control">
                    <input
                        class={"input"}
                        type={"text"}
                        value={location_to_edit.name.clone()}
                        placeholder={t!("location-definer-title-placeholder")}
                        onchange={change_title}
                        />
                </div>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{t!("location-definer-description-label")}</div>
                <div class="control">
                    <textarea
                        class="textarea"
                        placeholder={t!("location-definer-description-placeholder")}
                        value={description}
                        oninput={change_description}>
                        {description}
                    </textarea>
                </div>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{t!("location-definer-address-label")}</div>
                <div class="control">
                    <input
                        class={"input"}
                        type={"text"}
                        value={location_to_edit.address.clone()}
                        placeholder={t!("location-definer-address-placeholder")}
                        onchange={change_address}
                        />
                </div>
            </div>
            <OpenedHoursEdit opened_hours={location_to_edit.opened_hours.clone()} on_opened_hours_changed={change_opened_hours}/>
            <ContactMethodsEdit methods={location_to_edit.contact_methods.clone()} on_methods_changed={change_contact_methods}/>
            <div class="field">
                <div class="label">{t!("location-definer-tags-label")}</div>
                <div class="control">
                    <TagsSelectionEditForLocation location={location_to_edit.clone()} location_edit_manager={props.location_edit_manager.clone()}/>
                </div>
            </div>
            {edit_control_buttons(
                button_save_on_click,
                button_clear_on_click,
                button_remove_on_click,
            )}
        </>
    )
}
