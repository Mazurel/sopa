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

#[derive(Properties, PartialEq)]
pub struct ContactViewProps {
    pub contact: ContactType,
}

#[function_component(ContactView)]
pub fn contact_view(props: &ContactViewProps) -> Html {
    let contact_view_fontawesome_icon = match props.contact {
        ContactType::Email(_) => "fa-envelope",
        ContactType::PhoneNumber(_) => "fa-phone-volume",
        ContactType::WebAddress(_) => "fa-link",
    };

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
