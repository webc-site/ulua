//! Source: `VM/src/lgc.cpp:311-319` (hand-ported)

use crate::{
  enums::tms::TMS,
  functions::cstr_bytes_ref::cstr_bytes_ref,
  macros::{gfasttm::gfasttm, svalue::svalue},
  records::{global_state::global_State, lua_table::LuaTable},
};

/// 读表元表的 `__mode` 字段（w6e §10 收口：`*const c_char` → `Option<&'a [u8]>`，
/// `c_char` 只准活在 FFI 边界；null 哨兵 → `None`，空串形态一并折叠）。
///
/// # Safety
/// `g`/`h` 的存活已由 `&` 形参承载；调用方仍须保证：返回切片借用自 `h.metatable`
/// 所指 `__mode` TValue 的 TString payload（经 `gfasttm` fasttm 缓存读出），在 `'a`
/// 有效窗内该元表/串不被回收、不改写——即调用方在持有切片期间不得推进 GC 或替换
/// 元表。`metatable` 缺失、`__mode` 非字符串（cpp NULL 哨兵两态）均返回 `None`。
/// 只读，不分配、不抛错、不回收。
/// cpp VM/src/lgc.cpp:333
pub(crate) unsafe fn gettablemode<'a>(g: &global_State, h: &LuaTable) -> Option<&'a [u8]> {
  // SAFETY: `gfasttm` 仅经 `*const` 换出的 `*mut` 读数 `g.tmname`（只读转呈，本函数
  // 从不写 `g`）与 `h.metatable` 的 tmcache/tm 数组（宏自身 `# Safety` 契约，metatable
  // 允许 null 短路）；命中分支 `mode` 指向元表 tm 槽内 TValue，`is_string` 判定先行，
  // `svalue!` 取串 payload 起始指针后由 `cstr_bytes_ref` 收拢 NUL 扫描单点。
  unsafe {
    let mode = gfasttm(
      (g as *const global_State).cast_mut(),
      h.metatable,
      TMS::TmMode,
    );
    if !mode.is_null() && (*mode).is_string() {
      Some(cstr_bytes_ref(svalue!(mode)))
    } else {
      None
    }
  }
}
