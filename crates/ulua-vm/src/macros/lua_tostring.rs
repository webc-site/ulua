//! Source: `VM/include/lua.h:447` (hand-ported)
// #define lua_tostring(l, i) lua_tolstring(l, (i), NULL)
/// lua.h 兼容宏：展开为 C-ABI 垫片 `lua_tolstring`（`size_t*` 出参传 NULL），返回
/// NUL 结尾 `*const c_char`（内嵌 `\0` 处截断视图，与 cpp `lua_tostring` 一致）。
/// Rust 侧需要全字节/长度时直接用 `lua_tolstring_ref`。
#[macro_export]
macro_rules! lua_tostring {
  ($l:expr, $i:expr) => {
    $crate::functions::lua_tolstring::lua_tolstring($l, $i, core::ptr::null_mut())
  };
}
pub use lua_tostring;
