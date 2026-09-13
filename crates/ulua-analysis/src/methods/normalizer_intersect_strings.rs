use alloc::{collections::BTreeMap, string::String};

use crate::{
  records::{normalized_string_type::NormalizedStringType, normalizer::Normalizer},
  type_aliases::type_id::TypeId,
};
impl Normalizer {
  pub fn intersect_strings(
    &mut self,
    here: &mut NormalizedStringType,
    there: &NormalizedStringType,
  ) {
    self.consume_fuel();

    // Case 1,2,3
    if there.is_string() {
    }
    // Case 4, Case 7
    else if here.is_string() {
      here.singletons.clear();
      for (key, type_id) in &there.singletons {
        here.singletons.insert(key.clone(), *type_id);
      }
      here.is_cofinite = here.is_cofinite && there.is_cofinite;
    }
    // Case 5
    else if here.is_intersection() && there.is_intersection() {
      here.is_cofinite = true;
      for (key, type_id) in &there.singletons {
        here.singletons.insert(key.clone(), *type_id);
      }
    }
    // Case 6
    else if here.is_union() && there.is_intersection() {
      here.is_cofinite = false;
      for key in there.singletons.keys() {
        here.singletons.remove(key);
      }
    }
    // Case 8
    else if here.is_intersection() && there.is_union() {
      here.is_cofinite = false;
      let mut result: BTreeMap<String, TypeId> = there.singletons.clone();
      for key in here.singletons.keys().cloned().collect::<Vec<_>>() {
        result.remove(&key);
      }
      here.singletons = result;
    }
    // Case 9
    else if here.is_union() && there.is_union() {
      here.is_cofinite = false;

      let mut result = BTreeMap::new();
      result.extend(here.singletons.iter().map(|(k, v)| (k.clone(), *v)));
      result.extend(there.singletons.iter().map(|(k, v)| (k.clone(), *v)));

      let mut to_remove = Vec::new();
      for key in result.keys() {
        if !here.singletons.contains_key(key) || !there.singletons.contains_key(key) {
          to_remove.push(key.clone());
        }
      }
      for key in to_remove {
        result.remove(&key);
      }

      here.singletons = result;
    } else {
      ulua_common::LUAU_ASSERT!(false);
    }
  }
}
