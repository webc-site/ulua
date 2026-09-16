use core::ptr::null;

use crate::{
  functions::{
    follow_type::follow_type_id, get_table_type::get_table_type, get_type_alt_j::get_type_id,
  },
  records::singleton_type::SingletonType,
  type_aliases::{name_type::Name, type_id::TypeId},
};
pub fn get_table_match_tag(type_id: TypeId) -> Option<(Name, *const SingletonType)> {
  let ttv = get_table_type(type_id)?;

  for (name, prop) in &ttv.props {
    let prop_type_id = prop.read_ty.or(prop.write_ty).unwrap_or(null());
    let followed_type_id = follow_type_id(prop_type_id);
    if let Some(singleton) = get_type_id::<SingletonType>(followed_type_id) {
      return Some((name.clone(), singleton as *const SingletonType));
    }
  }

  None
}
