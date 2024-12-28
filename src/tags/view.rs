use yew::prelude::*;

use super::data::{Tag, Tags};

#[derive(Clone, Copy, PartialEq, Eq)]
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
    selection_changed: Option<Callback<TagSelectionType>>,
}

#[function_component(SelectableTag)]
pub fn selectable_tag(props: &SelectableTagProps) -> Html {
    use TagSelectionType::*;

    let tag_selection_state = use_state(|| props.selection_type);

    let tag_selection_state_clone = tag_selection_state.clone();
    let onclick = Callback::from(move |_| {
        let next_selection_state = match *tag_selection_state_clone {
            Acceptable => NonAcceptable,
            NonAcceptable => Acceptable,
        };
        tag_selection_state_clone.set(next_selection_state.clone());
    });

    let mut classes = Vec::new();

    match *tag_selection_state.clone() {
        Acceptable => {
            classes.push("is-primary".to_string());
            classes.push("is-clickable".to_string());
            if let Some(selection_changed) = &props.selection_changed {
                selection_changed.emit(Acceptable);
            }
        }
        NonAcceptable => {
            classes.push("is-danger".to_string());
            classes.push("is-clickable".to_string());
            classes.push("has-background-danger-light".to_string());
            if let Some(selection_changed) = &props.selection_changed {
                selection_changed.emit(NonAcceptable);
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

    let text_classes = classes.join(" ").to_string();

    if props.interactive {
        html!(
            <div class={text_classes} {onclick}>
                {props.tag.human_readable()}
            </div>
        )
    } else {
        html!(
            <div class={text_classes}>
                {props.tag.human_readable()}
            </div>
        )
    }
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct TagSelectionProps {
    pub tags: Tags,
}

#[function_component(TagPreferenceSelection)]
pub fn tag_selection(props: &TagSelectionProps) -> Html {
    let tags_state = use_state(|| props.tags.clone());
    let gender_text_selection = t!("select-tags");

    html! {
        <div class="container card is-max-tablet mt-2 p-1 is-shadowless">
            <div class="block is-info is-size-3 p-1">
                {gender_text_selection}
            </div>
            <div class="is-flex is-flex-direction-row is-flex-wrap-wrap p-2">
                {
                    tags_state.get_all_tags().into_iter().map(|tag|
                        html!(
                            <SelectableTag
                                tag={tag.clone()}
                                selection_type={TagSelectionType::NonAcceptable}
                                selection_changed={Callback::from(|_| {})}
                            />
                        )
                    ).collect::<Vec<Html>>()
                }
            </div>
        </div>
    }
}
