use super::data::*;
use crate::routes::location_finder::tags::{SelectableTag, TagSelectionType, Tags};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct LocationViewProps {
    pub location: Location,
    pub selected_tags: Tags,
}

#[function_component(LocationView)]
pub fn location_view(props: &LocationViewProps) -> Html {
    let location = &props.location;

    let tag_elements = location
        .tags
        .get_all_tags()
        .iter()
        .map(|t| {
            let selection_type = match props.selected_tags.has_tag(*t) {
                false => TagSelectionType::NonAcceptable,
                true => TagSelectionType::Acceptable,
            };

            html!(<SelectableTag
                tag={(*t).clone()}
                interactive={false}
                {selection_type}
            />)
        })
        .collect::<Vec<_>>();

    html!(
        <div class="component is-max-tablet p-5">
            <div class="card">
                <div class="card-header">
                    <div class="card-header-title has-background-info has-text-dark is-size-4 is-capitalized has-text-centered">
                        {location.name.clone()}
                    </div>
                </div>
                <div class="card-content">
                    <div class="component">
                        {t!("location-tags-info").to_string()}
                    </div>
                    <div class="component">
                        {tag_elements}
                    </div>
                </div>
            </div>
        </div>
    )
}
