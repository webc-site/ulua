use std::panic::catch_unwind;

use crate::functions::{
  assert_code_allocator_testing_panic::assert_code_allocator_testing_panic,
  throwing_code_allocator_test_alt_b::throwing,
};
pub fn obscure_throw_case(f: extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64) {
  // To simplify debugging, CHECK_THROWS_WITH_AS is not used here
  let result = catch_unwind(|| {
    let _ = f(10, throwing);
  });

  assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));
}
