use libsopa::tags::get_all_supported_tags;
use libsopa::yew_components::TagView;
use yew::prelude::*;

#[function_component(LocationDefiner)]
pub fn location_definer() -> Html {
    let location_definer_prompt = t!("location-definer-prompt");
    let location_definer_title_label = t!("location-definer-title-label");
    let location_definer_title_placeholder = t!("location-definer-title-placeholder");
    let location_definer_address_label = t!("location-definer-address-label");
    let location_definer_address_placeholder = t!("location-definer-address-placeholder");
    let location_definer_tags_label = t!("location-definer-tags-label");
    let all_tags_view: Vec<Html> = get_all_supported_tags()
        .into_iter()
        .map(|tag| html!(<TagView {tag} interactive={false}/>))
        .collect();

    html!(
        <div class="columns">
            <div class="column is-one-third">
                {"List of places will be here !"}
            </div>
            <div class="column box is-two-thirds">
                <div class="content">
                    <h2>{location_definer_prompt}</h2>
                </div>
                <div class="section">
                    <div class="field container is-max-tablet">
                        <div class="label">{location_definer_title_label}</div>
                        <div class="control">
                            <input
                                class={"input"}
                                type={"text"}
                                placeholder={location_definer_title_placeholder}/>
                        </div>
                    </div>
                    <div class="field container is-max-tablet">
                        <div class="label">{location_definer_address_label}</div>
                        <div class="control">
                            <input
                                class={"input"}
                                type={"text"}
                                placeholder={location_definer_address_placeholder}/>
                        </div>
                    </div>
                </div>
                <div class="field">
                    <div class="label">{location_definer_tags_label}</div>
                    <div class="control">
                        <div>{all_tags_view}</div>
                    </div>
                </div>
            </div>
        </div>
    )
}
