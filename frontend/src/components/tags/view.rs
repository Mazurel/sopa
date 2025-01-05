use yew::prelude::*;

use super::data::{Tag, Tags};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagSelectionType {
    Acceptable,
    NonAcceptable,
}

#[derive(Properties, PartialEq)]
pub struct SelectableTagProps {
    pub tag: Tag,
    pub selection_type: TagSelectionType,
    #[prop_or(true)]
    pub interactive: bool,
    pub selection_changed: Option<Callback<(Tag, TagSelectionType)>>,
}

#[function_component(TagView)]
pub fn selectable_tag(props: &SelectableTagProps) -> Html {
    use TagSelectionType::*;

    let tag_selection_state = use_state_eq(|| props.selection_type);

    let tag_selection_state_clone = tag_selection_state.clone();
    let onclick = Callback::from(move |_| {
        let next_selection_state = match *tag_selection_state_clone {
            Acceptable => NonAcceptable,
            NonAcceptable => Acceptable,
        };
        tag_selection_state_clone.set(next_selection_state.clone());
    });

    let mut classes = Vec::new();

    // When interactive mode is not use, always use property value!
    let tag_selection = {
        if props.interactive {
            *tag_selection_state.clone()
        } else {
            props.selection_type.clone()
        }
    };

    match tag_selection {
        Acceptable => {
            classes.push("is-primary".to_string());
            if props.interactive {
                classes.push("is-clickable".to_string());
                if let Some(selection_changed) = &props.selection_changed {
                    selection_changed.emit((props.tag.clone(), Acceptable));
                }
            }
        }
        NonAcceptable => {
            classes.push("is-danger".to_string());
            classes.push("has-background-danger-light".to_string());
            if props.interactive {
                classes.push("is-clickable".to_string());
                if let Some(selection_changed) = &props.selection_changed {
                    selection_changed.emit((props.tag.clone(), NonAcceptable));
                }
            }
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

    html!(
        if props.interactive {
            <div class={classes!(classes)} {onclick}>
                {props.tag.human_readable()}
            </div>
        }
        else {
            <div class={classes!(classes)}>
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
        .get_all_tags()
        .into_iter()
        .map(|tag| {
            let selected_tags_state = selected_tags_state.clone();
            let cb = move |(tag, state): (Tag, TagSelectionType)| {
                use TagSelectionType::*;
                match state {
                    Acceptable => {
                        let new_tags = selected_tags_state.with_tag(tag.to_string());
                        selected_tags_state.set(new_tags);
                    }
                    NonAcceptable => {
                        let new_tags = selected_tags_state.without_tag(tag.to_string());
                        selected_tags_state.set(new_tags);
                    }
                }
            };
            html!(
                <TagView
                    tag={tag.clone()}
                    selection_type={TagSelectionType::NonAcceptable}
                    selection_changed={cb}
                />
            )
        })
        .collect::<Vec<Html>>();

    props
        .on_tag_preference_changed
        .emit((*selected_tags_state).clone());

    html! {
        <div class="container card is-max-tablet mt-2 p-1 is-shadowless">
            <div class="block is-info is-size-3 p-1">
                {tag_selection}
            </div>
            <div class="is-flex is-flex-direction-row is-flex-wrap-wrap p-2">
                {interactive_tags}
            </div>
        </div>
    }
}
