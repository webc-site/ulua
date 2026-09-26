use ulua_vm::type_aliases::stk_id::StkId;

use crate::{records::vm_frame::VmFrame, type_aliases::lua_state::LuaState};

/// 生成码回写的 GETVARARGS（定计数）慢路径解释器（cpp `executeGETVARARGS` 常量分支）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（当前被调为 L 闭包），`base` 为本帧活动
/// 栈基址（`ci->func` 与变参段同属分配栈数组），`rai`/`b` 为指令编码的目标寄存器号与
/// 请求个数。边界契约集中于 [`VmFrame::new`]，其余为安全逻辑。
pub unsafe fn execute_getvarargs_const(l: *mut LuaState, base: StkId, rai: i32, b: i32) {
  let frame = unsafe { VmFrame::new(l, base) };

  let n = frame.varargs_count();
  let ra = frame.reg(rai);

  // setobj_2_s 逐元素拷贝并带 checkliveness，不可合并为整段复制
  let copy_count = b.min(n);
  if copy_count > 0 {
    let src = frame.slots(frame.slot_back(base, n as usize), copy_count as usize);
    let dst = frame.slots_mut(ra, copy_count as usize);
    for (d, s) in dst.iter_mut().zip(src) {
      frame.set_stack_value(d, s);
    }
  }
  if b > n {
    let nils = frame.slots_mut(
      frame.slot_at(ra, n.max(0) as usize),
      (b - n.max(0)) as usize,
    );
    for slot in nils {
      frame.set_nil(slot);
    }
  }
}

/// # Safety
/// C-ABI 导出边界：由生成码/VM 按 codegen 回调约定调用，`l`/`base`/`rai`/`b` 的合法性与
/// 存活性与 [`execute_getvarargs_const`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn execute_getvarargsconst(
  l: *mut LuaState,
  base: StkId,
  rai: i32,
  b: i32,
) {
  // Safety: 导出 C ABI 入口原样转发 l/base/rai/b 给同契约 unsafe fn execute_getvarargs_const;
  // 调用方按 ABI 提供活 LuaState 与帧内 base 栈指针, 满足被调前置条件。
  unsafe {
    execute_getvarargs_const(l, base, rai, b);
  }
}
