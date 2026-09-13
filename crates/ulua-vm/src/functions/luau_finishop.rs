use core::mem::size_of;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::LUAU_INSN_A, luau_insn_d::LUAU_INSN_D,
    luau_insn_op::LUAU_INSN_OP, luau_unreachable::LUAU_UNREACHABLE,
  },
};

use crate::{
  macros::{
    clvalue::clvalue, lua_callinfo_opyield::LUA_CALLINFO_OPYIELD, setobj_2_s::setobj_2_s,
    ttisnil::ttisnil, vm_reg::VM_REG,
  },
  records::closure::LClosure,
  type_aliases::{instruction::Instruction, lua_state::lua_State, stk_id::StkId},
};

pub(crate) unsafe fn luau_finishop(l: *mut lua_State) {
  unsafe {
    let ci = &mut *(*l).ci;
    ci.flags &= !(LUA_CALLINFO_OPYIELD as u32);

    let cl = clvalue!((*(*l).ci).func);
    let _base = (*l).base;

    let pc_ptr = ci.savedpc;
    let insn: Instruction = *pc_ptr.offset(-1); // the interrupted instruction

    let mut pc = pc_ptr;
    match LUAU_INSN_OP!(insn) {
      op if op == LuauOpcode::LOP_FORGLOOP as u32 => {
        let ra: StkId = VM_REG!(LUAU_INSN_A!(insn), l, (*l).base);

        // copy first variable back into the iteration index
        setobj_2_s!(l, ra.add(2), ra.add(3));

        // note that we need to increment pc by 1 to exit the loop since we need to skip over aux
        if ttisnil!(ra.add(3)) {
          pc = pc.offset(1);
        } else {
          pc = pc.offset(LUAU_INSN_D!(insn) as isize);
        }

        let lcl = core::ptr::addr_of!((*cl).inner.l).cast::<LClosure>();
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
