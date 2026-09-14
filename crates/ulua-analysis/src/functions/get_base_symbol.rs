use crate::{
  functions::baseof::baseof,
  records::{field::Field, symbol::Symbol},
  type_aliases::l_value::{LValue, LValueMember},
};

pub fn get_base_symbol(lvalue: &LValue) -> Symbol {
  let mut current: *const LValue = lvalue;

  unsafe {
    while <Field as LValueMember>::get_if(&*current).is_some() {
      current = baseof(&*current);
    }

    let symbol = <Symbol as LValueMember>::get_if(&*current);
    debug_assert!(symbol.is_some());
    symbol.unwrap().clone()
  }
}
