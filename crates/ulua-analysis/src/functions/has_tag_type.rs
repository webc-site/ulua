extern crate alloc;

use crate::type_aliases::tags::Tags;

pub fn has_tag(tags: &Tags, tag_name: &str) -> bool {
  tags.iter().any(|t| t == tag_name)
}
