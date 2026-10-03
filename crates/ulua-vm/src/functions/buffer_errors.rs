//! buffer 族共享错误消息（cpp lbuffer.cpp/lstrlib.cpp 同款字面量）。
//! format_args! 首参必须是字符串字面量，无法用 &str 常量传递，故用函数收口。

use crate::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

/// "buffer access out of bounds" 错误（必然抛出，不返回）
///
/// safe fn：`l` 以 `&mut` 借用承载「存活且独占的调用帧」前提（review.md §2），本函数
/// 无内存不安全面；抛错不返回属调用序契约——须处于可捕获错误的受保护帧（buffer 库
/// C 函数调用约定），否则文案槽读取失真。
#[inline]
pub(crate) fn buffer_oob_error(l: &mut LuaState) -> ! {
  // SAFETY: `l.as_mut_ptr()` 为由 `&mut` 借用重建的存活帧裸参（有效性与独占由引用承
  // 载）；`luaL_error` 经其格式化并抛出错误、不返回。受保护帧前提属调用序契约（见函数文档）。
  unsafe { luaL_error!(l.as_mut_ptr(), "buffer access out of bounds") }
}

/// "bit count is out of range of [0; 32]" 错误（必然抛出，不返回）
///
/// safe fn 与调用序契约同 [`buffer_oob_error`]。
#[inline]
pub(crate) fn buffer_bitcount_error(l: &mut LuaState) -> ! {
  // SAFETY: 同 [`buffer_oob_error`]——`l.as_mut_ptr()` 为借用重建的存活帧裸参，抛错不返回。
  unsafe { luaL_error!(l.as_mut_ptr(), "bit count is out of range of [0; 32]") }
}
