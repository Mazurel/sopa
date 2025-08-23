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

mod macros;
pub mod types;
pub use types::*;

// Note: These macros are defined in `macros.rs`.
use super::define_tags;

define_tags!(
    // Sexes
    "Male" group: TagGroup::Sex,
    "Female" group: TagGroup::Sex,
    // Age
    "Adult" group: TagGroup::Age,
    "Kid" group: TagGroup::Age,
    "Teenagers" group: TagGroup::Age,
    "Elderly" group: TagGroup::Age,
    // Geolocations
    "Trojmiasto" group: TagGroup::GeoLocation,
    "Pomorskie" group: TagGroup::GeoLocation,
    "Warszawa" group: TagGroup::GeoLocation,
    // Other
    "Hostel",
    "LGBT",
    "domestic abuse",
    "homelessness crisis",
    "disability",
    "suicidal crisis",
    "discrimination",
    "law",
    "social issues",
    "sexual abuse",
    "victims of crime",
    "parenting",
    "alcohol abuse",
    "substance abuse",
    "NFZ",
    "NGO", // Non-goverment
    "griving",
    "financial issues",
    "unemployment",
    "couples",
    "long-term therapy",
    "mental illness"
);

pub fn get_all_supported_tags() -> &'static Tags {
    &ALL_DEFINED_TAGS
}

pub fn get_all_supported_tags_without_group() -> &'static Tags {
    &ALL_DEFINED_TAGS_WITHOUT_GROUP
}

// TODO: This should be removed and sorting by order should be performed in the `Tags`
pub fn get_all_supported_tags_in_order() -> Vec<&'static Tag> {
    get_all_supported_tags().get_all_tags_in_order()
}

pub fn get_all_supported_tags_of_group(tag_group: &TagGroup) -> &Tags {
    TAGS_BY_TAG_GROUP.get(tag_group).unwrap() // We know all are supported
}

impl Tags {
    pub fn get_all_tags_in_order(&self) -> Vec<&Tag> {
        let mut tags = self.tags.iter().collect::<Vec<_>>();
        tags.sort_by_key(|tag| tag.human_readable().to_lowercase());
        tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check_for_tag_group() {
        for (sample_group, sample_entry) in [
            // Few known examples, to ensure everything is OK
            (TagGroup::Age, Tag::new("Kid".to_string())),
            (TagGroup::GeoLocation, Tag::new("Trojmiasto".to_string())),
            (TagGroup::Sex, Tag::new("Male".to_string())),
        ] {
            let tags = get_all_supported_tags_of_group(&sample_group);
            assert!(tags.get_all_tags().len() > 0);
            assert!(tags.has_tag(&sample_entry));
        }
    }

    #[test]
    fn sanity_check_for_tags_with_and_without_groups() {
        let all_tags = get_all_supported_tags().get_all_tags();
        let all_tags_without_groups = get_all_supported_tags_without_group().get_all_tags();
        let all_tags_of_some_group = get_all_supported_tags_of_group(&TagGroup::Age).get_all_tags();

        assert!(all_tags.len() > all_tags_without_groups.len());
        assert!(all_tags_without_groups.len() > all_tags_of_some_group.len());
    }
}
