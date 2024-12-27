use yew::prelude::*;

use super::data::{Tag, Tags};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TagSelectionType {
    Acceptable,
    NonAcceptable,
}

#[derive(Properties, PartialEq)]
struct SelectableTagProps {
    tag: Tag,
    selection_type: TagSelectionType,
    selection_changed: Callback<TagSelectionType>,
}

#[function_component(SelectableTag)]
fn selectable_tag(props: &SelectableTagProps) -> Html {
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
            classes.push("is-success".to_string());
            classes.push("has-text-grey-darker".to_string());
            classes.push("has-background-info-light".to_string());
            props.selection_changed.emit(Acceptable);
        }
        NonAcceptable => {
            classes.push("is-danger".to_string());
            classes.push("has-background-danger-light".to_string());
            props.selection_changed.emit(NonAcceptable);
        }
    }
    classes.append(&mut vec![
        "tag".to_string(),
        "is-size-8".to_string(),
        "has-text-weight-semibold".to_string(),
        "has-text-centered".to_string(),
        "is-hoverable".to_string(),
        "p-4".to_string(),
        "m-2".to_string(),
        "is-clickable".to_string(),
        "is-unselectable".to_string(),
    ]);

    let text_classes = classes.join(" ").to_string();
    html!(
        <div class={text_classes} {onclick}>
            {props.tag.human_readable()}
        </div>
    )
}

#[function_component(TagPreferenceSelection)]
pub fn tag_selection() -> Html {
    let tags_state = use_state(|| {
        let mut tags = Tags::new();
        tags.define_tag("gender:male");
        tags.define_tag("gender:female");
        tags.define_tag("sexuality:lgbt");
        tags
    });

    let gender_text_selection = t!("select-tags");

    html! {
        <div class="container card is-max-tablet mt-2 p-3 is-shadowless">
            <div class="block has-text-info is-size-3">
                {gender_text_selection}
            </div>
            <div class="is-flex is-flex-direction-row is-flex-wrap-wrap">
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
