use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::{
  functions::baseof::baseof,
  records::{field::Field, l_value_hasher::LValueHasher, symbol::Symbol},
  type_aliases::l_value::{LValue, LValueMember},
};
impl LValueHasher {
  pub fn operator_call(&self, lvalue: &LValue) -> usize {
    // Most likely doesn't produce high quality hashes, but we're probably ok enough with it.
    // When an evidence is shown that operator==(LValue) is used more often than it should, we can have a look at improving the hash quality.
    let mut acc: usize = 0;
    let mut offset: usize = 0;

    let mut current: *const LValue = lvalue;
    unsafe {
      while !current.is_null() {
        if let Some(field) = <Field as LValueMember>::get_if(&*current) {
          let mut hasher = DefaultHasher::new();
          field.key.hash(&mut hasher);
          let key_hash = hasher.finish() as usize;
          offset += 1;
          acc ^= (key_hash << 1) >> offset;
        } else if let Some(symbol) = <Symbol as LValueMember>::get_if(&*current) {
          acc ^= symbol.hash_luau_symbol_operator_call() << 1;
        } else {
          debug_assert!(
            false,
            "Hash not accumulated for this new LValue alternative."
          );
        }

        current = baseof(&*current);
      }
    }

    acc
  }
}
