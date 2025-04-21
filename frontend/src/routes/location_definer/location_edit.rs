use super::location_edit_manager::LocationEditManager;
use super::tags_selection::TagsSelectionEditForLocation;
use libsopa::locations::Location;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LocationEditProps {
    #[prop_or(Location::default())]
    pub initial_location_to_edit: Location,
    pub location_edit_manager: LocationEditManager,
}

#[function_component(LocationEdit)]
pub fn location_edit(props: &LocationEditProps) -> Html {
    let location_definer_prompt = t!("location-definer-prompt");
    let location_definer_title_label = t!("location-definer-title-label");
    let location_definer_title_placeholder = t!("location-definer-title-placeholder");
    let location_definer_address_label = t!("location-definer-address-label");
    let location_definer_address_placeholder = t!("location-definer-address-placeholder");
    let location_definer_tags_label = t!("location-definer-tags-label");
    let location_definer_button_save = t!("location-definer-button-save");
    let location_definer_button_reset = t!("location-definer-button-reset");
    // let location_definer_button_exit = t!("location-definer-button-exit");

    let change_title: Callback<_> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |event: Event| {
            let maybe_input_element = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = maybe_input_element {
                let mut location = location_edit_manager.get_location_under_edit();
                location.name = input.value();
                location_edit_manager.update_location_cb.emit(location);
            }
        })
    };

    let change_address: Callback<_> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |event: Event| {
            let maybe_input_element = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = maybe_input_element {
                let mut location = location_edit_manager.get_location_under_edit();
                location.address = input.value();
                location_edit_manager.update_location_cb.emit(location);
            }
        })
    };

    let button_save_on_click: Callback<MouseEvent> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |_| {
            location_edit_manager.commit_changes_cb.emit(());
        })
    };

    let button_clear_on_click: Callback<MouseEvent> = {
        let location_edit_manager = props.location_edit_manager.clone();
        Callback::from(move |_| {
            location_edit_manager.clear_changes_cb.emit(());
        })
    };

    let location_to_edit = props.initial_location_to_edit.clone();

    html!(
        <>
            <div class="content">
                <h2>{location_definer_prompt}</h2>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{location_definer_title_label}</div>
                <div class="control">
                    <input
                        class={"input"}
                        type={"text"}
                        value={location_to_edit.name.clone()}
                        placeholder={location_definer_title_placeholder}
                        onchange={change_title}
                        />
                </div>
            </div>
            <div class="field container is-max-tablet">
                <div class="label">{location_definer_address_label}</div>
                <div class="control">
                    <input
                        class={"input"}
                        type={"text"}
                        value={location_to_edit.address.clone()}
                        placeholder={location_definer_address_placeholder}
                        onchange={change_address}
                        />
                </div>
            </div>
            <div class="field">
                <div class="label">{location_definer_tags_label}</div>
                <div class="control">
                    <TagsSelectionEditForLocation location={location_to_edit.clone()} location_edit_manager={props.location_edit_manager.clone()}/>
                </div>
            </div>
            <div class="field has-addons mt-5">
                <div class="control">
                    <button class="button is-link is-light" onclick={button_save_on_click}>
                        {location_definer_button_save}
                    </button>
                </div>
                <div class="control">
                    <button class="button is-warning is-light" onclick={button_clear_on_click}>
                        {location_definer_button_reset}
                    </button>
                </div>
                /* TODO: When new location is registered, only then this button is to be used */
                /*
                <div class="control">
                    <button class="button is-danger is-light">
                        {location_definer_button_exit}
                    </button>
                </div>
                */
            </div>
        </>
    )
}
