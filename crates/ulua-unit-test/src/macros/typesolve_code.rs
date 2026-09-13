#[macro_export]
macro_rules! TYPESOLVE_CODE {
  ($code:expr) => {
    let result = self.check($code);
    $crate::macros::luau_require_no_errors::LUAU_REQUIRE_NO_ERRORS!(result);
  };
}

pub use TYPESOLVE_CODE;
