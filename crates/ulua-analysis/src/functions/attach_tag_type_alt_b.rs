use alloc::string::String;

use crate::records::property_type::Property;
pub fn attach_tag_property_string(prop: &mut Property, tag_name: &str) {
  prop.tags.push(String::from(tag_name));
}
