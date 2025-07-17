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

use web_sys::HtmlInputElement;
use yew::prelude::*;

use libsopa::contact::{Contact, ContactMethods, ContactType};

fn get_contact_fontawesome_icon(contact: ContactType) -> &'static str {
    match contact {
        ContactType::Email => "fa-envelope",
        ContactType::PhoneNumber => "fa-phone",
        ContactType::WebAddress => "fa-link",
    }
}

#[derive(Properties, PartialEq)]
pub struct ContactViewProps {
    pub contact: Contact,
}

#[function_component(ContactView)]
pub fn contact_view(props: &ContactViewProps) -> Html {
    let ContactViewProps { contact } = props;
    let contact_view_fontawesome_icon = get_contact_fontawesome_icon(props.contact.contact_type);

    let contact_view_actual = {
        match contact.contact_type {
            ContactType::WebAddress => {
                let href = format!("{}", contact.value.clone());
                html!(
                    <a {href} target="blank">{contact.value.clone()}</a>
                )
            }
            ContactType::Email => {
                let href = format!("mailto:{}", contact.value.clone());
                html!(
                    <a {href}>{contact.value.clone()}</a>
                )
            }
            _ => html!(
                <span>{contact.value.clone()}</span>
            ),
        }
    };

    let icon_class = classes!(["fas", contact_view_fontawesome_icon,]);
    html!(
        <div class="icon-text ml-6 mt-2 mb-1">
          <span class="icon has-text-info">
            <i class={icon_class}></i>
          </span>
          {contact_view_actual}
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct ContactEditProps {
    pub contact: Contact,
    pub update_contact_cb: Callback<Option<Contact>>,
}

#[function_component(ContactEdit)]
pub fn contact_edit(props: &ContactEditProps) -> Html {
    const ALL_CONTACT_TYPES: [ContactType; 3] = [
        ContactType::Email,
        ContactType::PhoneNumber,
        ContactType::WebAddress,
    ];
    let contacts_edit_value_label = t!("contacts-edit-value-label");

    let contact_view_fontawesome_icon =
        get_contact_fontawesome_icon(props.contact.contact_type.clone());
    let icon_class = classes!(["fas", contact_view_fontawesome_icon,]);

    let type_of_contact = {
        let contact_type_as_str = props.contact.contact_type.to_string();
        let all_contact_types: Html = ALL_CONTACT_TYPES
            .iter()
            .map(|contact_type| {
                let selected_contact = props.contact.clone();
                let update_contact_cb = props.update_contact_cb.clone();
                let fontawesome_icon = get_contact_fontawesome_icon(contact_type.clone());
                let contact_type_name = contact_type.to_string();
                let mut class = classes!(["fas", fontawesome_icon,]);
                let onclick = Callback::from(move |_| {
                    update_contact_cb.emit(Some(Contact {
                        contact_type: contact_type.clone(),
                        value: selected_contact.value.clone(),
                    }))
                });

                if selected_contact.contact_type == *contact_type {
                    class.push("is-active");
                }

                html!(
                <a class="dropdown-item" {onclick}>
                    <span class="icon is-small">
                        <i {class} aria-hidden="true"></i>
                    </span>
                    <span class="pl-1">{contact_type_name}</span>
                </a>)
            })
            .collect();

        html!(
            <>
                <div class="dropdown is-hoverable">
                <div class="dropdown-trigger">
                    <button class="button" aria-haspopup="true" aria-controls="dropdown-menu">
                    <span class="icon is-small">
                        <i class={icon_class} aria-hidden="true"></i>
                    </span>
                    <span class="pr-1 pl-1">{contact_type_as_str}</span>
                    <span class="icon is-small">
                        <i class="fas fa-angle-down" aria-hidden="true"></i>
                    </span>
                    </button>
                </div>
                <div class="dropdown-menu" id="dropdown-menu" role="menu">
                    <div class="dropdown-content">
                        {all_contact_types}
                    </div>
                </div>
                </div>
            </>
        )
    };

    let onchange: Callback<Event> = {
        let contact = props.contact.clone();
        let update_contact_cb = props.update_contact_cb.clone();
        Callback::from(move |event: Event| {
            let maybe_input_element = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input_element) = maybe_input_element {
                let value = input_element.value();
                let mut new_contact = contact.clone();
                new_contact.value = value;
                update_contact_cb.emit(Some(new_contact));
            }
        })
    };

    let remove_self_cb: Callback<_> = {
        let update_contact_cb = props.update_contact_cb.clone();
        Callback::from(move |_| {
            update_contact_cb.emit(None);
        })
    };

    html!(
            <div class="field container is-max-tablet mt-1">
                <div class="control">
                    <div class="columns">
                        <div class="column icon-text is-half is-info has-text-info">
                            {type_of_contact}
                        </div>
                        <div class="column">
                            <input
                                class={"input"}
                                type={"text"}
                                value={props.contact.value.clone()}
                                {onchange}
                                />
                        </div>
                        <div>
                            <button class="icon m-3" onclick={remove_self_cb}>
                                <i class="fas fa-lg fa-minus has-text-danger" style="transform: translate(0%,50%);"></i>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct ContactsViewProps {
    pub methods: ContactMethods,
}

#[function_component(ContactMethodsView)]
pub fn contacts_view(props: &ContactsViewProps) -> Html {
    props
        .methods
        .all_contact_methods()
        .into_iter()
        .map(|contact| html!(<ContactView contact={contact.clone()}/>))
        .collect()
}

#[derive(Properties, PartialEq, Clone)]
pub struct ContactsEditProps {
    pub methods: ContactMethods,
    pub on_methods_changed: Callback<ContactMethods>,
}

#[function_component(ContactMethodsEdit)]
pub fn contacts_view(props: &ContactsEditProps) -> Html {
    let contacts_edit_menu_title = t!("contacts-edit-menu-title");
    let contacts_edit_add_contact = t!("contacts-edit-add-contact");

    let selected_contact_method_idx = use_state(|| 0);
    let selected_contact_methods_state = use_state(|| props.methods.clone());
    let selected_contact_method_state = {
        let selected_contact_method_idx = selected_contact_method_idx.clone();
        let selected_contact_methods_state = selected_contact_methods_state.clone();
        use_state(move || {
            (*selected_contact_methods_state)
                .all_contact_methods()
                .get(*selected_contact_method_idx)
                .cloned()
        })
    };

    {
        // Restart editing when upstream methods change
        let selected_contact_method_idx = selected_contact_method_idx.clone();
        let selected_contact_method_state = selected_contact_method_state.clone();
        let selected_contact_methods_state = selected_contact_methods_state.clone();
        use_effect_with(props.methods.clone(), move |new_methods| {
            selected_contact_method_idx.set(0);
            selected_contact_method_state.set(None);
            selected_contact_methods_state.set(new_methods.clone());
        });
    }

    {
        // Update currently selected to be editied contact option
        let selected_contact_method_idx = selected_contact_method_idx.clone();
        let selected_contact_methods_state = selected_contact_methods_state.clone();
        let selected_contact_method_state = selected_contact_method_state.clone();
        let is_editing_new_method =
            selected_contact_methods_state.len() <= *selected_contact_method_idx;

        if is_editing_new_method {
            let new_method = if selected_contact_method_state.is_none() {
                let new_method = Contact {
                    contact_type: ContactType::default(),
                    value: "".to_string(),
                };
                selected_contact_method_state.set(Some(new_method.clone()));
                new_method
            } else {
                (*selected_contact_method_state).clone().unwrap()
            };
            let mut selected_contact_methods = (*selected_contact_methods_state).clone();
            selected_contact_methods.add_new_contact_method(new_method);
            selected_contact_methods_state.set(selected_contact_methods);
        }
    }

    let add_new_contact_method: Callback<yew::MouseEvent> = {
        let selected_contact_method_idx = selected_contact_method_idx.clone();
        Callback::from(move |_| {
            let next_i = *selected_contact_method_idx + 1;
            selected_contact_method_idx.set(next_i);
        })
    };

    let contact_methods_html: Html = selected_contact_methods_state
        .all_contact_methods()
        .into_iter()
        .enumerate()
        .map(|(i, contact_method)| {
            let selected_contact_method_idx = selected_contact_method_idx.clone();
            let selected_contact_methods_state = selected_contact_methods_state.clone();
            let on_methods_changed = props.on_methods_changed.clone();
            let update_contact_cb = Callback::from(move |contact| match contact {
                Some(contact) => {
                    let mut new_contact_methods = (*selected_contact_methods_state).clone();
                    *new_contact_methods
                        .all_contact_methods_mut()
                        .get_mut(i)
                        .unwrap() = contact;
                    selected_contact_method_idx.set(i);
                    selected_contact_methods_state.set(new_contact_methods.clone());
                    on_methods_changed.emit(new_contact_methods);
                }
                None => {
                    let mut new_contact_methods = (*selected_contact_methods_state).clone();
                    new_contact_methods.all_contact_methods_mut().remove(i);
                    if i < *selected_contact_method_idx {
                        selected_contact_method_idx.set(*selected_contact_method_idx - 1);
                    }
                    selected_contact_methods_state.set(new_contact_methods.clone());
                    on_methods_changed.emit(new_contact_methods);
                }
            });
            html!(<ContactEdit contact={contact_method.clone()} {update_contact_cb}/>)
        })
        .collect();

    html!(
        <div class="block pb-2">
            <div class="block">
                <span class="is-size-5">{contacts_edit_menu_title}</span>
            </div>
            <div class="block">
                {contact_methods_html}
            </div>
            <div class="block has-text-right">
                <button class="icon mr-1" onclick={add_new_contact_method}>
                    <i class="fas fa-lg fa-plus has-text-primary" style="transform: translate(0%,20%);"></i>
                </button>
                <span>{contacts_edit_add_contact}</span>
            </div>
        </div>
    )
}
