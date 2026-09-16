use ulua_common::FFlag;

use crate::{
  functions::{get_singleton_type::get_singleton_type, get_type_alt_j::get_type_id},
  records::{
    boolean_singleton::BooleanSingleton, normalized_type::NormalizedType,
    singleton_type::SingletonType,
  },
};

impl NormalizedType {
  pub fn is_falsy(&self) -> bool {
    let mut has_a_false = false;
    if let Some(singleton_ptr) = get_type_id::<SingletonType>(self.booleans)
      && let Some(boolean_ptr) = get_singleton_type::<BooleanSingleton>(singleton_ptr)
    {
      has_a_false = !boolean_ptr.value;
    }

    if FFlag::LuauIntegerType2.get() {
      (has_a_false || self.has_nils())
        && !self.has_tops()
        && !self.has_extern_types()
        && !self.has_errors()
        && !self.has_numbers()
        && !self.has_strings()
        && !self.has_threads()
        && !self.has_buffers()
        && !self.has_tables()
        && !self.has_functions()
        && !self.has_tyvars()
        && !self.has_integers()
    } else {
      (has_a_false || self.has_nils())
        && !self.has_tops()
        && !self.has_extern_types()
        && !self.has_errors()
        && !self.has_numbers()
        && !self.has_strings()
        && !self.has_threads()
        && !self.has_buffers()
        && !self.has_tables()
        && !self.has_functions()
        && !self.has_tyvars()
    }
  }
}
