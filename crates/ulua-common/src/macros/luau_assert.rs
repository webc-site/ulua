/// cpp `LUAU_ASSERT(cond)` / `LUAU_ASSERT(cond, msg)` 对应：断言失败时经
/// `assert_call_handler` 上报（可被 host 的 AssertHandler 接管）后触发断点。
/// 仅在 `LUAU_ASSERTENABLED`（debug 或 `luau_assert` feature）下生效。
#[macro_export]
macro_rules! LUAU_ASSERT {
  ($expr:expr $(, $msg:expr)?) => {
    if $crate::macros::luau_assertenabled::LUAU_ASSERTENABLED {
      if !($expr) {
        $crate::functions::assert_call_handler::assert_fail(
          concat!(stringify!($expr) $(, " : ", stringify!($msg))?, "\0"),
          concat!(file!(), "\0"),
          line!() as i32,
        );
        $crate::LUAU_DEBUGBREAK!();
      }
    }
  };
}

pub use LUAU_ASSERT;
