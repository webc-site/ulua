#[macro_export]
macro_rules! luau_require_no_error {
  ($result:expr, $type:ty) => {
    loop {
      let res = &$result;
      if $crate::functions::find_error::find_error::<$type>(res) {
        $crate::functions::dump_errors::dump_errors(res);
        $crate::REQUIRE_MESSAGE!(false, "Expected to find no {} error", stringify!($type));
      }
      break;
    }
  };
}

pub use luau_require_no_error;
