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

#[macro_export]
macro_rules! count_args {
    // Match one or more arguments
    ($first:expr $(, $rest:expr)*) => {
        // Count the first argument and recursively count the rest
        1 + count_args!($($rest),*)
    };
    // Base case: no arguments
    () => {
        0
    };
}

#[macro_export]
macro_rules! define_tags {
    ($($tag:literal),*) => {
        use rust_i18n::t;

        static ALL_DEFINED_TAGS: [&'static str; count_args!($($tag),*)] = [
            $($tag),*
        ];

        impl Tag {
            pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
                match self.name.as_str() {
                    $($tag => t!($tag),)*
                    _ => std::borrow::Cow::Owned(self.name.clone()),
                }
            }
        }
    };
}
