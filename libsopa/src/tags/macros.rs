#[macro_export(local_inner_macros)]
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

#[macro_export(local_inner_macros)]
macro_rules! push_tag_without_group_or_do_nothing {
    ($vec: ident, $tag:literal group: $group_type:expr) => {
        // Nothing happens here :)
        {}
    };
    ($vec: ident, $tag:literal) => {
        $vec.push(Tag::new($tag.to_string()));
    };
}

#[macro_export]
macro_rules! define_tags {
    ($($tag:literal $(group: $group_type:expr)?),*) => {
        use rust_i18n::t;
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        static ALL_DEFINED_TAGS: [&'static str; super::count_args!($($tag),*)] = [
            $($tag),*
        ];

        lazy_static! {
            pub static ref TAGS_BY_TAG_GROUP: HashMap<TagGroup, Tags> = {
                let mut m = HashMap::new();

                for tag_group in ALL_TAG_GROUPS.iter() {
                    m.insert(*tag_group, Tags::new());
                }

                $(
                    $(
                        m.get_mut(&$group_type).unwrap().define_tag($tag);
                    )?
                )*
                m
            };

            pub static ref ALL_DEFINED_TAGS_WITHOUT_GROUP: Vec<Tag> = {
                let mut result = vec![];
                $(
                    super::push_tag_without_group_or_do_nothing!(result, $tag $(group: $group_type)?);
                )*
                result
            };
        }

        impl Tag {
            pub fn human_readable(&self) -> std::borrow::Cow<'_, str> {
                match self.name.as_str() {
                    $($tag => t!($tag),)*
                    _ => std::borrow::Cow::Owned(self.name.clone()),
                }
            }

            pub fn get_associated_group(&self) -> Option<TagGroup> {
                match self.name.as_str() {
                    $(
                        $(
                            $tag => Some($group_type),
                        )?
                    )*
                    _ => None
                }
            }
        }
    };
}
