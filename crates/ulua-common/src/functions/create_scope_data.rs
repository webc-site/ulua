//! Source: `Common/src/TimeTrace.cpp:263-266` (hand-ported)
//! C++ `LUAU_NOINLINE uint16_t createScopeData(const char* name, const char* category)`:
//! `return createToken(*Luau::TimeTrace::getGlobalContext(), name, category);`

use crate::functions::{create_token::create_token, get_global_context::get_global_context};

pub fn create_scope_data(name: &'static str, category: &'static str) -> u16 {
  let context = get_global_context();
  create_token(&context, name, category)
}
