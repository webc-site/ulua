use crate::{
  functions::{follow_type, get_table_type::get_table_type, get_type},
  records::{
    any_type::AnyType, error_converter::ErrorConverter, extern_type::ExternType,
    metatable_type::MetatableType,
  },
  type_aliases::type_id::TypeId,
};

impl<'a> ErrorConverter<'a> {
  pub fn find_call_metamethod(&self, _type: TypeId) -> Option<TypeId> {
    let type_ = follow_type::follow(_type);

    let metatable = if let Some(metatable_type) = get_type::get::<MetatableType>(type_) {
      Some(metatable_type.metatable)
    } else if let Some(extern_type) = get_type::get::<ExternType>(type_) {
      extern_type.metatable
    } else {
      None
    };

    let metatable = metatable?;

    let unwrapped = follow_type::follow(metatable);

    if get_type::get::<AnyType>(unwrapped).is_some() {
      return Some(unwrapped);
    }

    let mtt = get_table_type(unwrapped)?;
    if let Some(prop) = mtt.props.get("__call") {
      return prop.read_ty;
    }

    None
  }
}
