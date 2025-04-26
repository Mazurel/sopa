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

use yew::prelude::*;

use log::*;

use libsopa::contact::{ContactMethods, ContactType};

fn get_contact_fontawesome_icon(contact: ContactType) -> &'static str {
    match contact {
        ContactType::Email(_) => "fa-envelope",
        ContactType::PhoneNumber(_) => "fa-phone-volume",
        ContactType::WebAddress(_) => "fa-link",
    }
}

#[derive(Properties, PartialEq)]
pub struct ContactViewProps {
    pub contact: ContactType,
}

#[function_component(ContactView)]
pub fn contact_view(props: &ContactViewProps) -> Html {
    let contact_view_fontawesome_icon = get_contact_fontawesome_icon(props.contact.clone());

    let contact_view_content = match props.contact.clone() {
        ContactType::Email(email) => email,
        ContactType::PhoneNumber(phone) => phone,
        ContactType::WebAddress(address) => address,
    };

    let icon_class = classes!(["fas", contact_view_fontawesome_icon,]);
    html!(
        <div class="icon-text ml-2 mt-2 mb-1">
          <span class="icon has-text-info">
            <i class={icon_class}></i>
          </span>
          <span>{contact_view_content}</span>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct ContactEditProps {
    pub contact: ContactType,
}

#[function_component(ContactEdit)]
pub fn contact_edit(props: &ContactViewProps) -> Html {
    let contact_view_fontawesome_icon = get_contact_fontawesome_icon(props.contact.clone());

    let contact_view_content = match props.contact.clone() {
        ContactType::Email(email) => email,
        ContactType::PhoneNumber(phone) => phone,
        ContactType::WebAddress(address) => address,
    };

    let icon_class = classes!(["fas", contact_view_fontawesome_icon,]);
    html!(
        <div class="icon-text ml-2 mt-2 mb-1">
          <span class="icon has-text-info">
            <i class={icon_class}></i>
          </span>
          <span>{contact_view_content}</span>
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

#[derive(Properties, PartialEq)]
pub struct ContactsEditProps {
    pub methods: ContactMethods,
}

#[function_component(ContactMethodsEdit)]
pub fn contacts_view(props: &ContactsEditProps) -> Html {
    let contacts_edit_menu_title = t!("contacts-edit-menu-title");

    html!(
        <div class="block">
            <span class="is-size-5">{contacts_edit_menu_title}</span>
            // TODO: Put here list of editable contacts
            <button class="icon m-3">
                <i class="fas fa-lg fa-plus"></i>
            </button>
        </div>
    )
}
