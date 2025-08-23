use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash, str::FromStr};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagGroup {
    Sex,
    Age,
    GeoLocation,
}

impl TagGroup {
    pub fn human_readable(&self) -> std::borrow::Cow<'static, str> {
        use TagGroup::*;

        match self {
            Sex => t!("tag-group-sex"),
            Age => t!("tag-group-age"),
            GeoLocation => t!("tag-group-geolocation"),
        }
    }
}

pub const ALL_TAG_GROUPS: &[TagGroup] = &[TagGroup::Sex, TagGroup::Age, TagGroup::GeoLocation];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn new(name: String) -> Tag {
        Tag { name }
    }
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
    pub tags: HashSet<Tag>,
}

impl From<Vec<Tag>> for Tags {
    fn from(value: Vec<Tag>) -> Self {
        Tags::new_tags(value)
    }
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
        let tag = Tag::new(tag.to_string());
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

    /// "Filter" all tags so that only tags in returned object
    /// are defined in the `other_tags` object.
    pub fn filter_by(&mut self, other_tags: &Tags) {
        let all_self_tags: Vec<Tag> = self.get_all_tags().iter().map(|t| (*t).clone()).collect();
        for self_tag in all_self_tags {
            if !other_tags.has_tag(&self_tag) {
                self.undefine_tag(self_tag);
            }
        }
    }

    /// Works like `filtered_by`, but clones and returns self.
    pub fn filtered_by(&self, other_tags: &Tags) -> Self {
        let mut tags = self.clone();
        tags.filter_by(other_tags);
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
