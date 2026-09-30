use crate::{
  functions::{follow_type, get_type},
  records::{
    free_type::FreeType, function_type::FunctionType, table_type::TableType, type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};

pub fn get_level(ty: TypeId) -> Option<&'static TypeLevel> {
  let ty = follow_type::follow(ty);

  if let Some(ftv) = get_type::get::<FreeType>(ty).as_ref() {
    Some(&ftv.level)
  } else if let Some(ttv) = get_type::get::<TableType>(ty).as_ref() {
    Some(&ttv.level)
  } else if let Some(ftv) = get_type::get::<FunctionType>(ty).as_ref() {
    Some(&ftv.level)
  } else {
    None
  }
}
