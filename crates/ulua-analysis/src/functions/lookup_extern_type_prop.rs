use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{extern_type::ExternType, property_type::Property},
};

pub fn lookup_extern_type_prop<'a>(cls: &'a ExternType, name: &str) -> Option<&'a Property> {
  let mut cls_ref = cls;
  loop {
    if let Some(prop) = cls_ref.props.get(name) {
      return Some(prop);
    }

    // SAFETY: parent_id 是 arena 句柄，有效性由 C++ 同契约保证。
    cls_ref = cls_ref.parent.and_then(get_type_id::<ExternType>)?;
  }
}
