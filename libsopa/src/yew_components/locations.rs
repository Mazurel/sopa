use crate::locations::Location;
use crate::tags::Tags;
use crate::yew_components::{TagSelectionType, TagView};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct LocationViewProps {
    pub location: Location,
    #[prop_or(None)]
    pub global_selected_tags: Option<Tags>,
}

fn get_matching_tags(all_tags: &Tags, my_tags: &Tags) -> Vec<Html> {
    my_tags
        .get_all_tags()
        .iter()
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

    let tag_elements = match &props.global_selected_tags {
        None => None,
        Some(selected_tags) => Some(get_matching_tags(selected_tags, &props.location.tags)),
    };

    html!(
        <div class="component is-max-tablet p-5">
            <div class="card">
                <div class="card-header">
                    <div class="card-header-title has-background-info has-text-dark is-size-4 is-capitalized has-text-centered">
                        {location.name.clone()}
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
            </div>
        </div>
    )
}
