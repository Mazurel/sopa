use crate::components::tags::{get_all_supported_tags, TagView};
use yew::prelude::*;

#[function_component(LocationDefiner)]
pub fn location_definer() -> Html {
    let location_definer_prompt = t!("location-definer-prompt");
    let all_tags_view: Vec<Html> = get_all_supported_tags()
        .into_iter()
        .map(|tag| html!(<TagView {tag} interactive={false}/>))
        .collect();

    html!(
        <>
            <div>{location_definer_prompt}</div>
            <div>{all_tags_view}</div>
        </>
    )
}
