//! Source: `VM/src/laux.cpp:304-327` (hand-ported)

use alloc::string::String;

use crate::{
  functions::{libsize::libsize, lua_l_findtable::lua_l_findtable_bytes},
  macros::{
    getstr::getstr, lua_globalsindex::LUA_GLOBALSINDEX, lua_l_error::luaL_error,
    lua_registryindex::LUA_REGISTRYINDEX, lua_s_new::lua_s_new,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 引用形承载，
/// r12-w6d 收形降为安全 `fn`，同 `lua_l_checkstack`/`ensure_stack` 先例）：`libname`
/// 允许为 `None`（此时跳过建模块表）；`lr` 为 `LuaLReg` 纯切片，每项 `name` 为静态
/// 字节切片（不含尾部 `\0`）、`func` 为合法 C 函数指针；建表/注册可分配、`luaL_error`
/// 可抛错，须受保护帧。体内裸指针转手面（`lua_l_error_l`/`lua_s_new`/`getstr`/
/// `push_c_function` 的裸形被调）落逐句窄 `unsafe` 块，各自 `# Safety` 由本契约与
/// `lr` 静态表不变量满足——裸操作未清零，屏障只下沉不外溢为签名 `unsafe`。
pub fn lua_l_register_bytes(l: &mut LuaState, libname: Option<&[u8]>, lr: &[LuaLReg]) {
  if let Some(libname) = libname {
    let size = libsize(lr);
    lua_l_findtable_bytes(l, LUA_REGISTRYINDEX, b"_LOADED", 1);
    l.get_field_bytes(-1, libname);
    if !l.is_table(-1) {
      l.pop(1);
      if !lua_l_findtable_bytes(l, LUA_GLOBALSINDEX, libname, size).is_null() {
        let name = String::from_utf8_lossy(libname);
        // SAFETY: `l.as_mut_ptr()` 自独占借用重建、`l` 按契约为可抛错受保护帧，
        // 借用窗止于当句（抛错发散）
        unsafe { luaL_error!(l.as_mut_ptr(), "name conflict for module '{}'", name) };
      }
      l.push_value(-1);
      l.set_field_bytes(-3, libname);
    }
    l.remove(-2);
  }

  for reg in lr {
    // SAFETY: `lua_s_new` 裸形参自独占借用重建，`l` 按契约存活可完成字符串驻留；
    // `reg.name` 为契约保证的静态字节切片
    let ts = unsafe { lua_s_new(l.as_mut_ptr(), reg.name) };
    // SAFETY: `reg.func` 为 `lr` 契约保证的合法 C 函数指针；`getstr(ts)` 的 `ts`
    // 是上一句 `lua_s_new` 刚驻留的堆上 TString（非空、含 NUL 终止，寿命随该对象，
    // 本句注册闭包即完成读取）
    unsafe { l.push_c_function(reg.func, getstr(ts)) };
    l.set_field_bytes(-2, reg.name);
  }
}
