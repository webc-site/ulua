use core::ptr::null;

use crate::{
  functions::{
    auxwrapcont::auxwrapcont_arm, auxwrapy::auxwrapy_arm, cocreate::cocreate,
    lua_pushcclosurek::lua_pushcclosurek_ref,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 首参 `l` 的存活/独占前提已由 `&mut` 接收者类型承载；保留 `unsafe fn` 的真前提在 `cocreate`
/// 被调一侧：`cocreate` 内部 `lua_newthread` → `lua_c_check_gc!`/线程屏障并解引用新建线程指针，
/// 仍是带 `# Safety` 的 `unsafe fn`，故本函数以 `unsafe` 调用之（借用窗止于本次调用）；`cocreate`
/// 对非函数实参经 `check_type` 抛错发散。
/// 随后的 `auxwrapy_arm`/`auxwrapcont_arm` 是写进闭包的静态 `extern "C-unwind"` 函数指针（由被调方
/// 存储、不在本帧解引用），其续延契约要求上值 1 恰为 `cocreate` 刚压入的新线程槽（`nup = 1` 即捕获
/// 该槽），`debugname` 传 null 对应 cpp 的 `NULL`（被调只存指针不读）。cpp `lcorolib.cpp:340`。
pub unsafe fn cowrap(l: &mut LuaState) -> i32 {
  // SAFETY: `cocreate` 为带 `# Safety` 的 `unsafe fn`（内部 lua_newthread 走 GC/线程屏障且解
  // 引用新线程指针）；`l` 为接收者存活独占借用，借用于本次调用，返回后本函数不再经 `l` 之前的
  // 裸地址访问
  unsafe { cocreate(l) };

  lua_pushcclosurek_ref(l, Some(auxwrapy_arm), null(), 1, Some(auxwrapcont_arm));

  1
}

lua_lib_fn!(pub fn cowrap @ref, cowrap_arm);
