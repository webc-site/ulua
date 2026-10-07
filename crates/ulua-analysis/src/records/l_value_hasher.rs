use ulua_common::collections::fast_hash;

use crate::{
  functions::baseof::baseof,
  records::{arena_handle::alias_ref, field::Field, symbol::Symbol},
  type_aliases::l_value::{LValue, LValueMember},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LValueHasher;

impl LValueHasher {
  pub fn hash(&self, lvalue: &LValue) -> usize {
    let mut acc: usize = 0;
    let mut offset: usize = 0;

    let mut current: *const LValue = lvalue;
    while !current.is_null() {
      if let Some(field) = <Field as LValueMember>::get_if(alias_ref(current)) {
        let key_hash = fast_hash(&field.key);
        offset += 1;
        acc ^= (key_hash << 1) >> offset;
      } else if let Some(symbol) = <Symbol as LValueMember>::get_if(alias_ref(current)) {
        acc ^= fast_hash(symbol) << 1;
      } else {
        debug_assert!(
          false,
          "Hash not accumulated for this new LValue alternative."
        );
      }

      current = baseof(alias_ref(current));
    }

    acc
  }
}
