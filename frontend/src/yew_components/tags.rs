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

use log::*;

use libsopa::tags::{Tag, Tags};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagSelectionType {
    Acceptable,
    NonAcceptable,
}

#[derive(Properties, PartialEq)]
pub struct TagViewProps {
    pub tag: Tag,
    #[prop_or(TagSelectionType::Acceptable)]
    pub selection_type: TagSelectionType,
    #[prop_or(true)]
    pub interactive: bool,
    #[prop_or(None)]
    pub selection_changed: Option<Callback<(Tag, TagSelectionType)>>,
}

fn tag_selection_into_html_style(selection_state: TagSelectionType) -> Classes {
    let mut classes: Vec<String> = vec![];

    match selection_state {
        TagSelectionType::Acceptable => {
            classes.push("is-primary".to_string());
        }
        TagSelectionType::NonAcceptable => {
            classes.push("is-danger".to_string());
            classes.push("has-background-danger-light".to_string());
        }
    }

    classes.append(&mut vec![
        "tag".to_string(),
        "is-size-6".to_string(),
        "has-text-weight-semibold".to_string(),
        "has-text-centered".to_string(),
        "is-hoverable".to_string(),
        "p-4".to_string(),
        "m-2".to_string(),
        "is-unselectable".to_string(),
    ]);

    classes!(classes)
}

#[function_component(TagView)]
pub fn selectable_tag(props: &TagViewProps) -> Html {
    use TagSelectionType::*;

    let tag_selection_state = use_state_eq(|| props.selection_type);

    {
        let tag_selection_state = tag_selection_state.clone();
        let selection_type = props.selection_type.clone();
        use_effect(move || {
            tag_selection_state.set(selection_type.clone());
        });
    }

    let onclick = {
        let tag_selection_state = tag_selection_state.clone();
        let selection_changed = props.selection_changed.clone();
        let tag = props.tag.clone();
        Callback::from(move |_| {
            let next_selection_state = match *tag_selection_state {
                Acceptable => NonAcceptable,
                NonAcceptable => Acceptable,
            };
            tag_selection_state.set(next_selection_state.clone());

            if let Some(selection_changed) = selection_changed.clone() {
                selection_changed.emit((tag.clone(), next_selection_state));
            }

            debug!("{:?} -> {:?}", tag.clone(), next_selection_state);
        })
    };

    let html_classes = tag_selection_into_html_style(*tag_selection_state);
    html!(
        if props.interactive {
            <div class={classes!(html_classes, "is-clickable")} {onclick}>
                {props.tag.human_readable()}
            </div>
        }
        else {
            <div class={classes!(html_classes)}>
                {props.tag.human_readable()}
            </div>
        }
    )
}

pub type TagPreference = Tags;

#[derive(Properties, Clone, PartialEq)]
pub struct TagSelectionProps {
    pub tags: Tags,
    pub on_tag_preference_changed: Callback<TagPreference>,
}

#[function_component(TagPreferenceSelection)]
pub fn tag_selection(props: &TagSelectionProps) -> Html {
    let selected_tags_state = use_state_eq(|| Tags::new());
    let tag_selection = t!("select-tags");

    let interactive_tags = props
        .tags
        .get_all_tags_in_order()
        .into_iter()
        .map(|tag| {
            let selected_tags_state = selected_tags_state.clone();
            let on_tag_preference_changed = props.on_tag_preference_changed.clone();
            let current_tag_selection_type = match selected_tags_state.has_tag(tag) {
                false => TagSelectionType::NonAcceptable,
                true => TagSelectionType::Acceptable,
            };
            let cb = move |(tag, state): (Tag, TagSelectionType)| {
                use TagSelectionType::*;
                match state {
                    Acceptable => {
                        let new_tags = selected_tags_state.with_tag(tag);
                        selected_tags_state.set(new_tags.clone());
                        on_tag_preference_changed.emit(new_tags);
                    }
                    NonAcceptable => {
                        let new_tags = selected_tags_state.without_tag(tag);
                        selected_tags_state.set(new_tags.clone());
                        on_tag_preference_changed.emit(new_tags);
                    }
                }
            };
            html!(
                <TagView
                    tag={tag.clone()}
                    selection_type={current_tag_selection_type}
                    selection_changed={cb}
                />
            )
        })
        .collect::<Vec<Html>>();

    html! {
        <div class="container card mt-2 p-1 is-shadowless">
            <div class="block is-info is-size-3 p-1">
                {tag_selection}
            </div>
            <div class="is-flex is-flex-direction-row is-flex-wrap-wrap p-2">
                {interactive_tags}
            </div>
        </div>
    }
}
