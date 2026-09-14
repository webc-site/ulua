use ulua_common::FInt;

use crate::{
  records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_int::ScopedFastInt,
};
#[derive(Debug)]
pub struct LimitFixture {
  #[cfg(debug_assertions)]
  pub luau_type_infer_recursion_limit: ScopedFastInt,
  pub base: BuiltinsFixture,
}

impl Default for LimitFixture {
  fn default() -> Self {
    Self {
      #[cfg(debug_assertions)]
      luau_type_infer_recursion_limit: ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 100),
      base: BuiltinsFixture::default(),
    }
  }
}
