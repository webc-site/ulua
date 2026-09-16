use crate::{
  functions::{follow_type::follow_type_id, get_mutable_type::get_mutable_type_id},
  records::{extern_type::ExternType, function_type::FunctionType, table_type::TableType},
  type_aliases::{tags::Tags, type_id::TypeId},
};

pub fn get_tags(ty: TypeId) -> Option<&'static mut Tags> {
  let ty = follow_type_id(ty);

  if let Some(ftv) = get_mutable_type_id::<FunctionType>(ty) {
    Some(&mut ftv.tags)
  } else if let Some(ttv) = get_mutable_type_id::<TableType>(ty) {
    Some(&mut ttv.tags)
  } else if let Some(etv) = get_mutable_type_id::<ExternType>(ty) {
    Some(&mut etv.tags)
  } else {
    None
  }
}
