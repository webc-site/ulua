use crate::{
  functions::{get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id},
  records::{
    normalized_string_type::NormalizedStringType, singleton_type::SingletonType,
    string_singleton::StringSingleton,
  },
};

pub fn is_normalized_string(ty: &NormalizedStringType) -> bool {
  if ty.is_string() {
    return true;
  }

  for (str, &type_id) in &ty.singletons {
    let Some(stv) = get_type_id::<SingletonType>(type_id) else {
      return false;
    };

    let Some(sstv) = get_singleton_type::<StringSingleton>(stv) else {
      return false;
    };

    if sstv.value != *str {
      return false;
    }
  }

  true
}
