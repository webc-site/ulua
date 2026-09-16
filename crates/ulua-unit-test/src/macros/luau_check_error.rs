#[macro_export]
macro_rules! LUAU_CHECK_ERROR {
  ($result:expr, $type:ty) => {
    loop {
      let res = &$result;
      if !$crate::functions::find_error::find_error::<$type>(res) {
        $crate::functions::dump_errors::dump_errors(res);
        $crate::CHECK_MESSAGE!(false, "Expected to find {} error", stringify!($type));
      }
      break;
    }
  };
}

pub use LUAU_CHECK_ERROR;
