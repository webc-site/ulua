use core::convert::Infallible;

use ulua_common::collections::fast_hash;

use crate::{
  functions::baseof::baseof,
  records::{field::Field, symbol::Symbol},
  type_aliases::l_value::{LValue, LValueMember},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LValueHasher {
  pub(crate) _unused: Option<Infallible>,
}

impl LValueHasher {
  pub fn hash(&self, lvalue: &LValue) -> usize {
    let mut acc: usize = 0;
    let mut offset: usize = 0;

    let mut current: *const LValue = lvalue;
    unsafe {
      while !current.is_null() {
        if let Some(field) = <Field as LValueMember>::get_if(&*current) {
          let key_hash = fast_hash(&field.key);
          offset += 1;
          acc ^= (key_hash << 1) >> offset;
        } else if let Some(symbol) = <Symbol as LValueMember>::get_if(&*current) {
          acc ^= fast_hash(symbol) << 1;
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
