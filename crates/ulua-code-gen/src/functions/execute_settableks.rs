use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16, luau_insn_b::LUAU_INSN_B,
    luau_insn_c::LUAU_INSN_C, luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{
  enums::tms::TMS,
  functions::{
    lua_h_setstr::lua_h_setstr as luaH_setstr, lua_v_call_tm::lua_v_call_tm,
    lua_v_settable::lua_v_settable,
  },
  macros::{
    clvalue::clvalue, fastnotm::fastnotm, fasttm::fasttm, gval_2_slot::gval2slot, hvalue::hvalue,
    lua_c_barriert::luaC_barriert, setobj_2_s::setobj_2_s as setobj2s, setobj_2_t::setobj2t,
    tsvalue::tsvalue, ttisfunction::ttisfunction, ttisstring::ttisstring, ttistable::ttistable,
    ttisuserdata::ttisuserdata, uvalue::uvalue,
  },
  records::closure::Closure,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::{
    vm_patch_c::vm_patch_c, vm_protect::vm_protect, vm_protect_pc::vm_protect_pc, vm_reg::VM_REG,
  },
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

#[inline]
unsafe fn vm_kv(i: u32, cl: *mut Closure, k: *mut TValue) -> *mut TValue {
  unsafe {
    let p = {
      let l = &(*cl).inner.l;
      l.p
    };
    ulua_common::LUAU_ASSERT!(i < (*p).sizek as u32);
    k.add(i as usize)
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_settableks(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe {
    let cl = clvalue!((*(*l).ci).func);
    let mut pc_ptr = pc;
    let insn = *pc_ptr;
    pc_ptr = pc_ptr.add(1);
    let op = LUAU_INSN_OP(insn);
    let ra = VM_REG!(LUAU_INSN_A(insn) as i32, l, base) as *mut TValue;
    let rb = VM_REG!(LUAU_INSN_B(insn) as i32, l, base) as *mut TValue;
    let aux = *pc_ptr;
    pc_ptr = pc_ptr.add(1);
    let kv = vm_kv(
      if op == LuauOpcode::LOP_SETUDATAKS as u32 {
        LUAU_INSN_AUX_KV16(aux)
      } else {
        aux
      },
      cl,
      k,
    );
    ulua_common::LUAU_ASSERT!(ttisstring!(kv as *const TValue));

    if ttistable!(rb as *const TValue) {
      let h = hvalue!(rb as *const TValue);

      if fastnotm((*h).metatable, TMS::TmNewIndex as i32) && (*h).readonly == 0 {
        vm_protect_pc(l, pc_ptr);

        let res = luaH_setstr(l, h, tsvalue!(kv as *const TValue) as *mut _);
        vm_patch_c(pc_ptr.sub(2), gval2slot!(h, res as *const TValue));
        setobj2t!(l, res, ra as *const TValue);
        luaC_barriert!(l, h, ra as *const TValue);
        return pc_ptr;
      }

      let slot = (LUAU_INSN_C(insn) as i32) & (*h).nodemask8 as i32;
      (*l).cachedslot = slot;
      vm_protect!(l, pc_ptr, {
        lua_v_settable(l, rb as *const TValue, kv, ra);
      });
      vm_patch_c(pc_ptr.sub(2), (*l).cachedslot);
      return pc_ptr;
    }

    let fn_tm: *const TValue;
    if ttisuserdata!(rb as *const TValue)
      && {
        fn_tm = fasttm(
          l,
          uvalue!(rb as *const TValue).metatable,
          TMS::TmNewIndex as i32,
        );
        !fn_tm.is_null()
      }
      && ttisfunction!(fn_tm)
      && (*clvalue!(fn_tm)).is_c != 0
    {
      ulua_common::LUAU_ASSERT!((*l).top.add(4) < (*l).stack.add((*l).stacksize as usize));
      let top = (*l).top;
      setobj2s!(l, top.add(0), fn_tm);
      setobj2s!(l, top.add(1), rb as *const TValue);
      setobj2s!(l, top.add(2), kv as *const TValue);
      setobj2s!(l, top.add(3), ra as *const TValue);
      (*l).top = top.add(4);

      (*l).cachedslot = LUAU_INSN_C(insn) as i32;
      vm_protect!(l, pc_ptr, {
        lua_v_call_tm(l, 3, -1);
      });
      vm_patch_c(pc_ptr.sub(2), (*l).cachedslot);
      return pc_ptr;
    }

    vm_protect!(l, pc_ptr, {
      lua_v_settable(l, rb as *const TValue, kv, ra);
    });
    pc_ptr
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeSETTABLEKS")]
pub unsafe extern "C-unwind" fn execute_settableks_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_settableks(l, pc, base, k) }
}
