use crate::{
  functions::{
    follow_type::follow_type_id, get_tags::get_tags, get_type_alt_j::get_type_id,
    has_tag_type::has_tag as has_tag_tags,
  },
  records::extern_type::ExternType,
  type_aliases::type_id::TypeId,
};

pub fn has_tag(ty: TypeId, tag_name: &str) -> bool {
  let ty = follow_type_id(ty);

  // We special case extern types because getTags only returns a pointer to one vector of tags.
  // But extern types has multiple vector of tags, represented throughout the hierarchy.
  if let Some(mut etv) = get_type_id::<ExternType>(ty) {
    loop {
      if has_tag_tags(&etv.tags, tag_name) {
        return true;
      } else if etv.parent.is_none() {
        return false;
      }

      // C++: parent 链上节点必为 ExternType（LUAU_ASSERT 必命中）
      etv = get_type_id::<ExternType>(etv.parent.unwrap()).unwrap();
    }
  } else if let Some(tags) = get_tags(ty) {
    return has_tag_tags(tags, tag_name);
  }

  false
}
