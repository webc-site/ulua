//! review.md §10 收形：函数名读取返回原生 `Option<&'static [u8]>`（不含终止 NUL
//! 的串体字节，`None` 即原 null 哨兵），与 `LuaDebug::name` 字段同形直连，
//! 不再经 `*const c_char`/`getstr` 指针面折转。

use crate::{
  functions::tstr_bytes::tstr_bytes,
  records::{closure::Closure, proto::Proto},
};

/// 取闭包的调试名（C 闭包 `c.debugname` / Lua 闭包 `p.debugname`）。
///
/// # Safety
///
/// `cl` 为 null 或指向存活 `Closure`；非空时其 `inner.c.debugname` 引用的静态串、
/// 或 `inner.l.p->debugname` TString 串体须在返回值寿命内保持有效（函数值在
/// 栈/帧上即存活、GC 不移动——与 `LuaDebug::name` 的存活契约同一）。
pub(crate) unsafe fn getfuncname<'a>(cl: *mut Closure) -> Option<&'a [u8]> {
  // SAFETY: 契约即上所列——null 直接返回 `None`（原 null 哨兵）；C 臂只透传
  // 闭包自有的 `Option<&'static [u8]>` 字段，Lua 臂经 `tstr_bytes` 单点把
  // TString 指针折成恰覆盖 payload 的字节窗
  unsafe {
    if cl.is_null() {
      return None;
    }

    if (*cl).is_c != 0 {
      (*cl).inner.c.debugname
    } else {
      let p: *mut Proto = (*cl).inner.l.p;
      if p.is_null() {
        return None;
      }
      let p_debugname = (&*p).debugname;
      if p_debugname.is_null() {
        None
      } else {
        Some(tstr_bytes(p_debugname))
      }
    }
  }
}
