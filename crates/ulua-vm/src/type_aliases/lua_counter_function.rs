use core::ffi::c_void;

/// 计数函数访问回调（cpp `lua_CounterFunction` 的 `const char* funcName` 面，review.md
/// §10 收形为原生 `Option<&[u8]>` 串体窗，`None` 即原 null）。
///
/// DELIBERATE DEVIATION（review.md §0）：同 [`crate::type_aliases::lua_coverage::LuaCoverage`]
/// ——全仓无真实 C 消费者，C ABI 声明收口为 `unsafe fn`（`context` 裸指针由回调自行解引用）。
pub type LuaCounterFunction =
  Option<for<'a> unsafe fn(context: *mut c_void, function: Option<&'a [u8]>, linedefined: i32)>;
