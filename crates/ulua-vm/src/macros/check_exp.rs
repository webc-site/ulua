#[macro_export]
macro_rules! check_exp {
  ($c:expr, $e:expr) => {{
    ulua_common::LUAU_ASSERT!($c);
    $e
  }};
}

pub use check_exp;
