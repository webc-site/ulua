/// cpp `LUAU_ASSERT(cond)` / `LUAU_ASSERT(cond, msg)` 对应：断言失败时经
/// `assert_call_handler` 上报，并按其返回值决定是否断点——host 注入的
/// `AssertHandler` 返回 0 即表示"已接管"，此时不再 debugbreak。
/// 仅在 `LUAU_ASSERTENABLED`（debug 或 `luau_assert` feature）下生效。
///
/// cpp 原型（`Common/include/Luau/Common.h:69`）：
/// `(void)(!!(expr) || (assertCallHandler(#expr, ...) && (LUAU_DEBUGBREAK(), 0)))`
#[macro_export]
macro_rules! LUAU_ASSERT {
  ($expr:expr $(, $msg:expr)?) => {
    if $crate::macros::luau_assertenabled::LUAU_ASSERTENABLED {
      if !($expr)
        && $crate::functions::assert_call_handler::assert_fail(
          concat!(stringify!($expr) $(, " : ", stringify!($msg))?, "\0"),
          concat!(file!(), "\0"),
          line!() as i32,
        )
      {
        $crate::LUAU_DEBUGBREAK!();
      }
    }
  };
}

pub use LUAU_ASSERT;
