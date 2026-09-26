use core::{mem::size_of, ptr::addr_of};

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_assert::{LUAU_ASSERT, LUAU_UNREACHABLE},
    luau_insn_ops::{luau_insn_a, luau_insn_d, luau_insn_op},
  },
};

use crate::{
  macros::{lua_callinfo_opyield::LUA_CALLINFO_OPYIELD, setobj_2_s::setobj_2_s, vm_reg::VM_REG},
  records::{closure::LClosure, lua_state::LuaState},
  type_aliases::{instruction::Instruction, stk_id::StkId},
};

/// # Safety
/// `l` 须为存活 LuaState 且当前帧 `(*l).ci` 已置 `LUA_CALLINFO_OPYIELD`、其 `savedpc - 1` 指向 `FORGLOOP` 指令；
/// `(*(*l).ci).func` 为 Lua 闭包（`clvalue!` 解引用），`VM_REG!(a, l, (*l).base)` 出的 `ra` 使 `ra+2..=ra+3` 落于栈内、
/// `(*(*lcl).p).code..+sizecode * size_of::<Instruction>()` 界住 `pc`（由 `LUAU_ASSERT` 断言）。须在 VM 主循环独占调用。
/// cpp/VM/src/lvmexecute.cpp:3823 luau_finishop。
pub(crate) unsafe fn luau_finishop(l: *mut LuaState) {
  unsafe {
    let ci = &mut *(*l).ci;
    ci.flags &= !(LUA_CALLINFO_OPYIELD as u32);

    let cl = (*(*(*l).ci).func).as_closure_ptr();

    let pc_ptr = ci.savedpc;
    let insn: Instruction = *pc_ptr.offset(-1); // the interrupted instruction

    let mut pc = pc_ptr;
    match luau_insn_op(insn) {
      op if op == LuauOpcode::LOP_FORGLOOP as u32 => {
        let ra: StkId = VM_REG!(luau_insn_a(insn), l, (*l).base);

        // copy first variable back into the iteration index
        setobj_2_s!(l, ra.add(2), ra.add(3));

        // note that we need to increment pc by 1 to exit the loop since we need to skip over aux
        if (*ra.add(3)).is_nil() {
          pc = pc.offset(1);
        } else {
          pc = pc.offset(luau_insn_d(insn) as isize);
        }

        let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
        let proto = (*lcl).p;
        LUAU_ASSERT!(
          (pc as usize).wrapping_sub((*proto).code as usize) / size_of::<Instruction>()
            < ((*proto).sizecode as usize)
        );
      }
      _ => {
        LUAU_ASSERT!(false);
        LUAU_UNREACHABLE!();
      }
    }

    (*(*l).ci).savedpc = pc;
  }
}
