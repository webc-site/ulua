use alloc::string::String;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{functions::get_tags::get_tags, type_aliases::type_id::TypeId};
pub fn attach_tag(ty: TypeId, tag_name: &str) {
  if let Some(tags) = get_tags(ty) {
    tags.push(String::from(tag_name));
  } else {
    LUAU_ASSERT!(false);
  }
}
