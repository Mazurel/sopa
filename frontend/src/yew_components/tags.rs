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

use libsopa::tags::Tag;

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
