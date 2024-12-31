use rust_i18n::t;
use std::collections::HashMap;

pub type TagId = String;

#[derive(Clone, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
    pub description: String,
}

impl Tag {
    pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
        match self.name.as_str() {
            "gender:male" => t!("gender:male"),
            "gender:female" => t!("gender:female"),
            "sexuality:lgbt" => t!("sexuality:lgbt"),
            "type:hostel" => t!("type:hostel"),
            _ => std::borrow::Cow::Owned(self.name.clone()),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Tags {
    tags: HashMap<TagId, Tag>,
}

impl Tags {
    pub fn new() -> Self {
        Tags {
            tags: HashMap::new(),
        }
    }

    pub fn new_tags<It: IntoIterator<Item = Str>, Str: ToString>(items: It) -> Self {
        let mut tags = Self::new();
        for item in items {
            tags.define_tag(item);
        }
        tags
    }

    pub fn get_all_tags(&self) -> Vec<&Tag> {
        self.tags.values().collect()
    }

    pub fn get_tag(&self, tag_id: TagId) -> Option<&Tag> {
        self.tags.get(&tag_id)
    }

    pub fn define_tag<Str: ToString>(&mut self, tag_name: Str) -> Option<TagId> {
        let tag_id = tag_name.to_string();
        let tag = Tag {
            name: tag_name.to_string(),
            description: String::from(""),
        };
        let insert_ok = self.tags.insert(tag_id.clone(), tag).is_none();

        match insert_ok {
            true => Some(tag_id),
            false => None,
        }
    }

    pub fn has_tag(&self, tag_id: &TagId) -> bool {
        self.tags.contains_key(tag_id)
    }
}

#[cfg(test)]
mod tests {
    use super::Tags;

    #[test]
    fn creating_tags() {
        let mut tags = Tags::new();
        let tag_id = tags
            .define_tag("Test".to_string())
            .expect("Tag insertion should be succesfull");

        let tag = tags
            .get_tag(tag_id)
            .expect("This tag should have been just inserted");
        assert_eq!(tag.name, "Test");

        for tag_name in (["Tag 1", "Tag 2", "Hello World"])
            .into_iter()
            .map(|t| t.to_string())
        {
            tags.define_tag(tag_name)
                .expect("Tag {tag_name} should have been sucesffuly inserted");
        }

        assert_eq!(tags.get_all_tags().len(), 4);
    }
}
