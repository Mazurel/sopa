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

use super::opened_hours::OpenedHoursView;
use crate::yew_components::{ContactMethodsView, TagSelectionType, TagView};
use libsopa::locations::Location;
use libsopa::tags::{get_all_supported_tags, Tags};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct LocationDescriptionViewProps {
    description: String,
}

#[function_component(LocationDescriptionView)]
pub fn location_description_view(props: &LocationDescriptionViewProps) -> Html {
    let description = props.description.trim_end();
    let is_description_empty = description.is_empty();
    let style = "white-space: pre-wrap; text-indent: 0 each-line;";

    html!(
        if !is_description_empty {
            <div class="block ml-6 mt-2">
                <span class="p-5" {style}>
                    <p>
                        { description }
                    </p>
                </span>
            </div>
        }
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct LocationAddressViewProps {
    address: String,
}

#[function_component(LocationAddressView)]
pub fn location_address_view(props: &LocationAddressViewProps) -> Html {
    let address = props.address.trim();
    let is_address_empty = address.is_empty();

    html!(
        if !is_address_empty {
            <div class="icon-text ml-6 mt-2">
              <span class="icon has-text-info">
                <i class="fas fa-home"></i>
              </span>
              <span>{address}</span>
            </div>
        }
    )
}

#[derive(Clone, PartialEq)]
pub struct SelectionSettings {
    pub selection_cb: Callback<Location>,
    pub state: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LocationViewProps {
    pub location: Location,
    #[prop_or(true)]
    pub simplified_view: bool,
    #[prop_or(None)]
    pub global_selected_tags: Option<Tags>,
    #[prop_or(None)]
    pub selection_settings: Option<SelectionSettings>,
}

fn get_matching_tags(all_tags: &Tags, my_tags: &Tags) -> Vec<Html> {
    my_tags
        .get_all_tags_in_order()
        .iter()
        .filter(|t| {
            // We want to filter tags that are not supported
            get_all_supported_tags().has_tag(t)
        })
        .map(|t| {
            let selection_type = match all_tags.has_tag(*t) {
                false => TagSelectionType::NonAcceptable,
                true => TagSelectionType::Acceptable,
            };

            html!(<TagView
                tag={(*t).clone()}
                interactive={false}
                {selection_type}
            />)
        })
        .collect::<Vec<_>>()
}

#[function_component(LocationView)]
pub fn location_view(props: &LocationViewProps) -> Html {
    let location = &props.location;
    let description = location.description.clone();
    let is_selectable = props.selection_settings.is_some();

    let tag_elements = match &props.global_selected_tags {
        None => None,
        Some(selected_tags) => Some(get_matching_tags(selected_tags, &props.location.tags)),
    };

    let onclick = {
        let selection_settings = props.selection_settings.clone();
        let location = location.clone();
        Callback::from(move |_| {
            if is_selectable {
                let selection_settings = selection_settings.clone().unwrap();
                let next_state = !selection_settings.state;
                if next_state == true {
                    selection_settings.selection_cb.emit(location.clone());
                }
            }
        })
    };

    let mut wrapper_classes = classes!("component", "is-max-tablet", "p-5");
    if is_selectable {
        let selected_state = props.selection_settings.clone().unwrap().state;
        if selected_state {
            wrapper_classes.push("selected");
        } else {
            wrapper_classes.push("selectable");
        }
    }

    html!(
        <div class={wrapper_classes} {onclick}>
            <div class="card">
                <div class="card-header">
                    <div class="card-header-title has-background-info has-text-dark is-size-4 is-capitalized">
                        {location.name.clone()}
                    </div>
                </div>
                if !props.simplified_view {
                    <LocationDescriptionView {description} />
                    <div class="columns">
                        <div class="column">
                            <LocationAddressView address={location.address.clone()} />
                            <ContactMethodsView methods={location.contact_methods.clone()} />
                        </div>
                        <div class="column">
                            <OpenedHoursView opened_hours={location.opened_hours.clone()} />
                        </div>
                    </div>
                    if let Some(tag_elements) = tag_elements {
                        <div class="card-content">
                            <div class="component">
                                {t!("location-tags-info").to_string()}
                            </div>
                            <div class="component">
                                {tag_elements}
                            </div>
                        </div>
                    }
                }
            </div>
        </div>
    )
}
