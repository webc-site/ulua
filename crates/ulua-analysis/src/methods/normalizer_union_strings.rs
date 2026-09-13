use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  methods::normalized_string_type_reset_to_string::normalized_string_type_reset_to_string,
  records::{normalized_string_type::NormalizedStringType, normalizer::Normalizer},
};
impl Normalizer {
  pub fn union_strings(&mut self, here: &mut NormalizedStringType, there: &NormalizedStringType) {
    self.consume_fuel();

    if there.is_string() {
      normalized_string_type_reset_to_string(here);
    } else if here.is_union() && there.is_union() {
      for (name, ty) in &there.singletons {
        here.singletons.insert(name.clone(), *ty);
      }
    } else if here.is_union() && there.is_intersection() {
      here.is_cofinite = true;
      for (name, ty) in &there.singletons {
        if let Some(it) = here.singletons.remove(name) {
          let _ = it;
        } else {
          here.singletons.insert(name.clone(), *ty);
        }
      }
    } else if here.is_intersection() && there.is_union() {
      for name in there.singletons.keys() {
        here.singletons.remove(name);
      }
    } else if here.is_intersection() && there.is_intersection() {
      let mut keys_to_remove = Vec::new();
      for name in here.singletons.keys() {
        if !there.singletons.contains_key(name) {
          keys_to_remove.push(name.clone());
        }
      }
      for name in keys_to_remove {
        here.singletons.remove(&name);
      }
    } else {
      LUAU_ASSERT!(false);
    }
  }
}
