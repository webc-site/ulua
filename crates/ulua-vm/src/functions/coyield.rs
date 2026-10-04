use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；本票收形后
/// 读数/让渡全经安全门面，体内已无裸操作，故本体降为安全 `fn`）：`l` 须为正被 `lua_resume`/
/// `auxresume` 驱动的协程帧，`get_top` 取出的 `nres` 即 `top - base` 的结果窗槽距，`yield_thread`
/// 据此写回 `base = top - nres`（恒等复位）与 `status`；非可 yield 帧（`n_ccalls > base_ccalls`）
/// 由被调方走抛错分支，不产生越界槽写。cpp VM/src/lcorolib.cpp:348 coyield。
pub fn coyield(l: &mut LuaState) -> i32 {
  // 顶-基槽距读数走 `get_top` 门面（r16-b2 收编留痕：其本体 `slot_distance(base, top)` 即被替代
  // 式 `top.offset_from(base) as i32` 的同址同宽镜像，现读位点不变；isize→i32 折形在协程栈槽距
  // 受 LUAI_MAXSTACK 约束的现域内无截差）
  let nres = l.get_top();

  l.yield_thread(nres)
}

lua_lib_fn!(pub fn coyield @ref, coyield_arm);
