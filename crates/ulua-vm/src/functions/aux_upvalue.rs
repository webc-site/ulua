//! `aux_upvalue` — 解析函数 TValue 中第 n 个 upvalue。
//! C++ source: `VM/src/lapi.cpp:1615`
//!
//! 成功时返回 `(upvalue 名称 C 串, 值指针)`；`fi` 非函数或 `n` 越界时返回
//! `None`（对应 C++ `NULL`）。名称可为空串（C 闭包、或 Lua 闭包中无名字的
//! upvalue），对应 C++ 的 `""`，与 `NULL` 语义不同，调用方须区分。

use core::ffi::c_char;

use crate::{
  enums::value_view::ValueView,
  macros::{getstr::getstr, lua_emptystr::LUA_EMPTYSTR},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `fi` 须指向栈内存活的 TValue；非函数时安全返回 `None`。为函数时按 C 闭包取 `inner.c.upvals`、
/// Lua 闭包取 `inner.l.p` 的 `upvalues`/`uprefs` 柔性数组，均须覆盖 `n-1` 下标（`n` 越界由返回值界守卫），
/// `uprefs` 项为 upval 时其 `uv.v` 指针可读。返回值指针仅在 `fi` 所指闭包存活期间有效。
/// cpp/VM/src/lapi.cpp:1697 aux_upvalue。
pub(crate) unsafe fn aux_upvalue(fi: StkId, n: i32) -> Option<(*const c_char, *mut TValue)> {
  unsafe {
    if !(*fi).is_function() {
      return None;
    }
    let f = &*(*fi).as_closure_ptr();

    if f.is_c != 0 {
      // C 闭包 — upvalues 内联存放在 `c.upvals`
      if !(1 <= n && n <= f.nupvalues as i32) {
        return None;
      }
      let val = f.inner.c.upvals.as_ptr().add((n - 1) as usize) as *mut TValue;
      Some((LUA_EMPTYSTR.as_ptr().cast(), val))
    } else {
      // Lua 闭包
      let p = f.inner.l.p;
      if !(1 <= n && n <= (*p).nups as i32) {
        return None;
      }
      // uprefs 是柔性数组 — 用指针运算寻址
      let r: *mut TValue = f.inner.l.uprefs.as_ptr().add((n - 1) as usize) as *mut TValue;
      // §11 pass B 三波簇1：ttisupval tag 判链收敛为 ValueView match——
      // cpp: `TValue* val = ttisupval(r) ? upvalue(r)->v : r;`（lapi.cpp:1716）
      // open 态经 `ValueView::UpVal` 带出 `*mut UpVal` 读 `.v`，close 后折回的直存槽
      // 落 `_` 臂用 `r`；纯读取指针透传，与收敛前宏链同址等价。
      let val = match ValueView::from_tvalue(&*r) {
        ValueView::UpVal(uv) => (*uv).v,
        _ => r,
      };
      if !(1 <= n && n <= (*p).sizeupvalues) {
        return Some((LUA_EMPTYSTR.as_ptr().cast(), val));
      }
      Some((getstr(*(*p).upvalues.add((n - 1) as usize)), val))
    }
  }
}
