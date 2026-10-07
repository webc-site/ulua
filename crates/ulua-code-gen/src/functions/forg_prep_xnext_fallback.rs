use ulua_vm::{records::lua_state::LuaState, type_aliases::t_value::TValue};

use crate::records::vm_frame::VmFrame;

const ITERATE_OVER: &str = "iterate over";

/// 生成码回写的 FORGPREP 半程（xnext）回退（cpp `forgPrepXnextFallback`）：
/// `ra` 非函数时按迭代语义报类型错误。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（当前被调为本帧的 L 闭包，其 code 数组有效），
/// `ra` 为帧内活栈槽，`pc` 为本闭包 code 内的指令偏移。边界契约集中于 [`VmFrame::current`]，
/// 其余为安全逻辑。
pub unsafe extern "C-unwind" fn forg_prep_xnext_fallback(
  l: *mut LuaState,
  ra: *mut TValue,
  pc: i32,
) {
  let frame = unsafe { VmFrame::current(l) };

  if !frame.is_function(ra) {
    // p 为 None 仅见于 C 函数闭包，不进入本 FORGPREP 回退路径
    let code = frame.proto_code(frame.closure_proto(frame.current_closure()));
    frame.save_pc(frame.insn_offset(code, pc as usize));
    frame.type_error(ra, ITERATE_OVER);
  }
}
