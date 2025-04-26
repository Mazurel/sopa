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

#[derive(Debug, PartialEq, Clone, Eq, Deserialize, Serialize)]
pub enum ContactType {
    PhoneNumber(String),
    Email(String),
    WebAddress(String),
}

#[derive(Debug, PartialEq, Clone, Eq, Deserialize, Serialize)]
pub struct ContactMethods {
    methods: Vec<ContactType>,
}

impl Default for ContactMethods {
    fn default() -> Self {
        Self {
            methods: Vec::new(),
        }
    }
}

impl ContactMethods {
    pub fn all_contact_methods<'a>(&'a self) -> &'a Vec<ContactType> {
        &self.methods
    }

    pub fn add_new_contact_method(&mut self, method: ContactType) {
        self.methods.push(method);
    }
}
