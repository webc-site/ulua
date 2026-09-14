use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    free_type::FreeType, function_type::FunctionType, table_type::TableType, type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};

pub fn get_level(ty: TypeId) -> Option<&'static TypeLevel> {
  let ty = follow_type_id(ty);

  if let Some(ftv) = get_type_id::<FreeType>(ty).as_ref() {
    Some(&ftv.level)
  } else if let Some(ttv) = get_type_id::<TableType>(ty).as_ref() {
    Some(&ttv.level)
  } else if let Some(ftv) = get_type_id::<FunctionType>(ty).as_ref() {
    Some(&ftv.level)
  } else {
    None
  }
}
