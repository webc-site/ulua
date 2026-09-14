#[macro_export]
macro_rules! LUAU_ASSERT {
  ($expr:expr) => {
    if $crate::macros::luau_assertenabled::LUAU_ASSERTENABLED {
      if !($expr) {
        $crate::functions::assert_call_handler::assert_fail(
          concat!(stringify!($expr), "\0"),
          concat!(file!(), "\0"),
          line!() as i32,
        );
        $crate::LUAU_DEBUGBREAK!();
      }
    }
  };
  ($expr:expr, $msg:expr) => {
    if $crate::macros::luau_assertenabled::LUAU_ASSERTENABLED {
      if !($expr) {
        $crate::functions::assert_call_handler::assert_fail(
          concat!(stringify!($expr), " : ", stringify!($msg), "\0"),
          concat!(file!(), "\0"),
          line!() as i32,
        );
        $crate::LUAU_DEBUGBREAK!();
      }
    }
  };
}

pub use LUAU_ASSERT;
