use crate::yew_components::{TagSelectionType, TagView};
use libsopa::tags::{
    get_all_supported_tags_of_group, get_all_supported_tags_without_group, Tag, TagGroup, Tags,
};
use yew::prelude::*;

pub type TagPreference = Tags;

fn togglable_tag(selected_tags_state: UseStateHandle<Tags>, tag: Tag) -> Html {
    let current_tag_selection_type = match selected_tags_state.has_tag(&tag) {
        false => TagSelectionType::NonAcceptable,
        true => TagSelectionType::Acceptable,
    };
    let cb = move |(tag, state): (Tag, TagSelectionType)| {
        use TagSelectionType::*;
        match state {
            Acceptable => {
                let new_tags = selected_tags_state.with_tag(tag);
                selected_tags_state.set(new_tags.clone());
            }
            NonAcceptable => {
                let new_tags = selected_tags_state.without_tag(tag);
                selected_tags_state.set(new_tags.clone());
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
}

#[derive(Properties, Clone, PartialEq)]
pub struct TagSelectionProps {
    pub on_tag_preference_changed: Callback<TagPreference>,
}

#[function_component(TagPreferenceSelection)]
pub fn tag_selection(props: &TagSelectionProps) -> Html {
    let selected_tags_state = use_state_eq(|| Tags::new());
    let tag_selection = t!("select-tags");

    {
        let on_tag_preference_changed = props.on_tag_preference_changed.clone();
        let selected_tags_state = selected_tags_state.clone();
        use_effect_with(selected_tags_state, move |new_tags| {
            on_tag_preference_changed.emit((**new_tags).clone());
        });
    }

    let ungrouped_interactive_tags = get_all_supported_tags_without_group()
        .get_all_tags_in_order()
        .into_iter()
        .map(|tag| {
            let selected_tags_state = selected_tags_state.clone();
            togglable_tag(selected_tags_state, tag.clone())
        })
        .collect::<Vec<Html>>();

    let mut grouped_tags_elements = vec![];
    for tag_group in [TagGroup::Age, TagGroup::GeoLocation, TagGroup::Sex] {
        let tags_in_group = get_all_supported_tags_of_group(&tag_group);
        let tag_group_name = tag_group.human_readable();

        let grouped_interactive_tags = tags_in_group
            .get_all_tags_in_order()
            .into_iter()
            .map(|tag| {
                let selected_tags_state = selected_tags_state.clone();
                togglable_tag(selected_tags_state, tag.clone())
            })
            .collect::<Vec<Html>>();

        grouped_tags_elements.push(html!(
            <div class="block">
                <h4 style="margin-bottom: 0.1em;" class="subtitle is-4">{tag_group_name}</h4>
                <div class="is-flex is-flex-direction-row is-flex-wrap-wrap tag-flex">
                    {grouped_interactive_tags}
                </div>
            </div>
        ));
    }

    html! {
        <>
            <section class="hero">
              <div class="hero-body ml-auto mr-auto">
                <p class="title">{tag_selection}</p>
              </div>
            </section>
            <div class="columns ml-6 mr-6">
                <div class="column">
                    <div class="block is-info is-size-3 p-1">
                        {grouped_tags_elements}
                    </div>
                </div>
                <div class="column card mt-2 p-1 is-shadowless">
                    <div class="is-flex is-flex-direction-row is-flex-wrap-wrap p-2 tag-flex">
                        {ungrouped_interactive_tags}
                    </div>
                </div>
            </div>
        </>
    }
}
