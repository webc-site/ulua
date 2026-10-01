use alloc::sync::Arc;
use core::ptr::null;

use crate::{
  records::{field::Field, symbol::Symbol},
  type_aliases::l_value::{LValue, LValueMember},
};
pub fn baseof(lvalue: &LValue) -> *const LValue {
  if let Some(field) = <Field as LValueMember>::get_if(lvalue) {
    return match &field.parent {
      Some(parent) => Arc::as_ptr(parent),
      None => null(),
    };
  }

  let symbol = <Symbol as LValueMember>::get_if(lvalue);
  debug_assert!(symbol.is_some());
  null() // Base of root is null.
}
