use alloc::string::String;

use crate::{
  functions::{follow_type, get_tags::get_tags, get_type},
  records::extern_type::ExternType,
  type_aliases::type_id::TypeId,
};

pub fn has_tag(tags: &[String], tag_name: &str) -> bool {
  tags.iter().any(|t| t == tag_name)
}

pub fn has_tag_type_id(ty: TypeId, tag_name: &str) -> bool {
  let ty = follow_type::follow(ty);

  // We special case extern types because getTags only returns a pointer to one vector of tags.
  // But extern types has multiple vector of tags, represented throughout the hierarchy.
  if let Some(mut etv) = get_type::get::<ExternType>(ty) {
    loop {
      if has_tag_tags(&etv.tags, tag_name) {
        return true;
      } else if etv.parent.is_none() {
        return false;
      }

      // C++: parent 链上节点必为 ExternType（LUAU_ASSERT 必命中）
      // Safety: 上方 else-if 已对 parent.is_none() 早返；parent 链节点按 cpp
      // LUAU_ASSERT(get<ExternType>(parent)) 同位必为 ExternType。
      etv = get_type::get::<ExternType>(
        etv
          .parent
          .expect("parent.is_none() 分支已 return，至此必为 Some"),
      )
      .expect("cpp LUAU_ASSERT：parent 链上节点必为 ExternType");
    }
  } else if let Some(tags) = get_tags(ty) {
    return has_tag_tags(tags, tag_name);
  }

  false
}

use crate::{functions::has_tag_type::has_tag as has_tag_tags, records::property_type::Property};

pub fn has_tag_property_string(prop: &Property, tag_name: &str) -> bool {
  has_tag_tags(&prop.tags, tag_name)
}
