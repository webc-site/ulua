use crate::{
  functions::baseof::baseof,
  records::{arena_handle::alias_ref, field::Field, symbol::Symbol},
  type_aliases::l_value::{LValue, LValueMember},
};

pub fn get_base_symbol(lvalue: &LValue) -> Symbol {
  let mut current: *const LValue = lvalue;

  while <Field as LValueMember>::get_if(alias_ref(current)).is_some() {
    current = baseof(alias_ref(current));
  }

  let symbol = <Symbol as LValueMember>::get_if(alias_ref(current));
  debug_assert!(symbol.is_some());
  // debug_assert 同前提：cpp 遍历契约保证 Field 链终点必为 Symbol。
  symbol
    .expect("Field 链终点按 cpp `get<Symbol>` 契约必为 Symbol")
    .clone()
}
