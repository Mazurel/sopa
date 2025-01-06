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
use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::{count_args, define_tags};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
}

define_tags!(
    "gender:male",
    "gender:female",
    "sexuality:lgbt",
    "type:hostel",
    "age:adult",
    "age:kid",
    "contact:phone"
);

pub fn get_all_supported_tags() -> Vec<Tag> {
    ALL_DEFINED_TAGS
        .iter()
        .map(|tag_name| Tag {
            name: tag_name.to_string(),
        })
        .collect()
}

impl FromStr for Tag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tag {
            name: String::from(s),
        })
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Hash for Tag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tags {
    tags: HashSet<Tag>,
}

impl Tags {
    pub fn new() -> Self {
        Tags {
            tags: HashSet::new(),
        }
    }

    pub fn new_tags<It: IntoIterator<Item = Str>, Str: ToString>(items: It) -> Self {
        let mut tags = Self::new();
        for item in items {
            tags.define_tag(item.to_string());
        }
        tags
    }

    pub fn get_all_tags(&self) -> Vec<&Tag> {
        self.tags.iter().collect()
    }

    pub fn define_tag<Str: ToString>(&mut self, tag: Str) -> Tag {
        let tag_id = tag.to_string();
        let tag = Tag {
            name: tag_id.to_string(),
        };
        self.tags.insert(tag.clone().into());
        tag
    }

    pub fn undefine_tag<Str: ToString>(&mut self, tag: Str) -> Tag {
        let tag_id = tag.to_string();
        let tag = Tag {
            name: tag_id.to_string(),
        };
        self.tags.remove(&tag);
        tag
    }

    pub fn with_tag<S: ToString>(&self, tag: S) -> Self {
        let mut tags = self.clone();
        tags.define_tag(tag);
        tags
    }

    pub fn without_tag<S: ToString>(&self, tag: S) -> Self {
        let mut tags = self.clone();
        tags.undefine_tag(tag);
        tags
    }

    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }

    pub fn overlap(&self, other: &Tags) -> f32 {
        let other_tags = other.get_all_tags();
        let other_tags_amount = other_tags.len();

        let overlap: f32 = other_tags
            .into_iter()
            .map(|t| match self.has_tag(t) {
                true => 1.0,
                false => 0.0,
            })
            .sum::<f32>()
            / other_tags_amount as f32;

        overlap.min(1.0).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Tags;

    #[test]
    fn creating_tags() {
        let mut tags = Tags::new();
        let tag = tags.define_tag("Test");
        assert!(tags.has_tag(&tag));

        for tag_name in (["Tag 1", "Tag 2", "Hello World"])
            .into_iter()
            .map(|t| t.to_string())
        {
            tags.define_tag(tag_name);
        }

        assert_eq!(tags.get_all_tags().len(), 4);
    }

    #[test]
    fn modyfing_tags() {
        let mut tags = Tags::new();
        let tag = tags.define_tag("Test");
        assert!(tags.has_tag(&tag.clone()));

        tags.undefine_tag("Test");
        assert!(!tags.has_tag(&tag.clone()));
    }
}
