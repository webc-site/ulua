use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{get_type, is_subclass_type::is_subclass_extern_type_extern_type},
  records::extern_type::ExternType,
  type_aliases::type_id::TypeId,
};

pub fn is_subclass_type_id_type_id(test: TypeId, parent: TypeId) -> bool {
  let test_ctv = get_type::get::<ExternType>(test);
  let parent_ctv = get_type::get::<ExternType>(parent);

  LUAU_ASSERT!(test_ctv.is_some() && parent_ctv.is_some());

  match (test_ctv, parent_ctv) {
    (Some(test_ctv), Some(parent_ctv)) => is_subclass_extern_type_extern_type(test_ctv, parent_ctv),
    _ => false,
  }
}
