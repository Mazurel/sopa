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

use crate::yew_components::{TagSelectionType, TagView};
use libsopa::{
    locations::Location,
    tags::{get_all_supported_tags, get_all_supported_tags_in_order, Tag, Tags},
};
use yew::prelude::*;

use super::location_edit_manager::LocationEditManager;

#[derive(Properties, PartialEq)]
pub struct TagsSelectionEditForLocationProps {
    pub location: Location,
    pub location_edit_manager: LocationEditManager,
}

#[function_component(TagsSelectionEditForLocation)]
pub fn tags_selection_edit_for_location(props: &TagsSelectionEditForLocationProps) -> Html {
    let TagsSelectionEditForLocationProps {
        location,
        location_edit_manager,
    } = props;

    let selection_changed: Callback<(Tag, TagSelectionType)> = {
        let location_edit_manager = location_edit_manager.clone();
        Callback::from(move |(tag, new_state)| match new_state {
            TagSelectionType::Acceptable => {
                let mut location = location_edit_manager.get_location_under_edit();
                if !location.tags.has_tag(&tag) {
                    location.tags = location.tags.with_tag(tag.clone());
                    assert!(location.tags.has_tag(&tag));
                    location_edit_manager.stage_location_changes(location);
                }
            }
            TagSelectionType::NonAcceptable => {
                let mut location = location_edit_manager.get_location_under_edit();
                if location.tags.has_tag(&tag) {
                    location.tags = location.tags.without_tag(tag.clone());
                    assert!(!location.tags.has_tag(&tag));
                    location_edit_manager.stage_location_changes(location);
                }
            }
        })
    };

    let location_tags = location.tags.clone();
    let all_tags = use_memo((), |_| Tags::from(get_all_supported_tags_in_order()));

    (*all_tags)
        .get_all_tags_in_order()
        .into_iter()
        .map(|tag| {
            let tag_state = match location_tags.has_tag(tag) {
                false => TagSelectionType::NonAcceptable,
                true => TagSelectionType::Acceptable,
            };
            html!(<TagView
                tag={tag.clone()}
                interactive={true}
                selection_changed={Some(selection_changed.clone())}
                selection_type={tag_state}
            />)
        })
        .collect::<Html>()
}
