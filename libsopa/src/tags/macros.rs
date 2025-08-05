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

#[macro_export]
macro_rules! define_tags {
    ($($tag:literal $(group: $group_type:expr)?),*) => {

        use rust_i18n::t;
        use lazy_static::lazy_static;
        use std::collections::HashMap;

        static ALL_DEFINED_TAGS: [&'static str; count_args!($($tag),*)] = [
            $($tag),*
        ];

        lazy_static! {
            pub static ref TAGS_BY_TAG_GROUP: HashMap<TagGroup, Vec<Tag>> = {
                let mut m = HashMap::new();

                for tag_group in ALL_TAG_GROUPS.iter() {
                    m.insert(*tag_group, vec![]);
                }

                $(
                    $(
                        m.get_mut(&$group_type).unwrap().push(Tag::new($tag.to_string()));
                    )?
                )*
                m
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
