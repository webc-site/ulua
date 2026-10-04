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
/// `l` 须为存活 `LuaState` 且**不得被宿主回调重入**：本票把首参收形为引用形，但体内仍有一处真实
/// 裸指针转手——`cocreate` 尚未收形，须把 `l` 的自身地址交其重建独占借用（借用窗止于当次调用，
/// 转交期间本函数不得再经 `l` 访问）。该转手不是纯形式：`cocreate` → `lua_newthread` 会走
/// `lua_c_check_gc!`/线程屏障并在末尾把同一 `*mut LuaState` 交给宿主 `cb.userthread` 回调，若宿主
/// 回调就地再取该帧的可变引用，`&mut` 接收者所承诺的独占即被打破（别名 UB），故此前提超出
/// `&mut LuaState` 所能承载，保留 `unsafe fn`；`cocreate` 对非函数实参经 `check_type` 抛错发散。
/// 随后的 `auxwrapy_arm`/`auxwrapcont_arm` 是写进闭包的静态 `extern "C-unwind"` 函数指针，其续延
/// 契约要求上值 1 恰为 `cocreate` 刚压入的新线程槽（`nup = 1` 即捕获该槽），`debugname` 传 null
/// 对应 cpp 的 `NULL`（被调只存指针不读）。cpp `lcorolib.cpp:340`。
pub unsafe fn cowrap(l: &mut LuaState) -> i32 {
  // SAFETY: `l.as_mut_ptr()` 即本帧存活 `LuaState` 自身地址（由 `&mut` 接收者保证存活/对齐），
  // 被调方按上述契约就地重建借用、不留存该指针，借用窗止于本次调用
  unsafe { cocreate(l.as_mut_ptr()) };

  lua_pushcclosurek_ref(l, Some(auxwrapy_arm), null(), 1, Some(auxwrapcont_arm));

  1
}

lua_lib_fn!(pub fn cowrap @ref, cowrap_arm);
