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

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Deserialize, Serialize)]
pub enum ContactType {
    PhoneNumber,
    Email,
    WebAddress,
}

impl ContactType {
    pub fn to_string(&self) -> String {
        let str_version = match &self {
            ContactType::PhoneNumber => t!("contact-type-phone-number"),
            ContactType::Email => t!("contact-type-email"),
            ContactType::WebAddress => t!("contact-type-web-address"),
        };
        str_version.to_string()
    }
}

impl Default for ContactType {
    fn default() -> Self {
        ContactType::Email
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Deserialize, Serialize)]
pub struct Contact {
    pub contact_type: ContactType,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, Eq, Deserialize, Serialize)]
pub struct ContactMethods {
    methods: Vec<Contact>,
}

impl Default for ContactMethods {
    fn default() -> Self {
        Self {
            methods: Vec::new(),
        }
    }
}

impl ContactMethods {
    pub fn all_contact_methods<'a>(&'a self) -> &'a Vec<Contact> {
        &self.methods
    }
    pub fn all_contact_methods_mut<'a>(&'a mut self) -> &'a mut Vec<Contact> {
        &mut self.methods
    }

    pub fn add_new_contact_method(&mut self, method: Contact) {
        self.methods.push(method);
    }

    pub fn len(&self) -> usize {
        self.methods.len()
    }
}
