extern crate alloc;

use crate::{functions::has_tag_type::has_tag as has_tag_tags, records::property_type::Property};

pub fn has_tag_property_string(prop: &Property, tag_name: &str) -> bool {
  has_tag_tags(&prop.tags, tag_name)
}
