use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16, luau_insn_b::LUAU_INSN_B,
    luau_insn_c::LUAU_INSN_C, luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{
  enums::tms::TMS,
  functions::{lua_g_methoderror::luaG_methoderror, lua_v_gettable::lua_v_gettable},
  macros::{
    clvalue::clvalue,
    fasttm::fasttm,
    gkey::{gkey, gval},
    hvalue::hvalue,
    setobj_2_s::setobj_2_s as setobj2s,
    tsvalue::tsvalue,
    ttisnil::ttisnil,
    ttisstring::ttisstring,
    ttistable::ttistable,
    ttisuserdata::ttisuserdata,
    ttype::ttype,
    uvalue::uvalue,
  },
  records::closure::Closure,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::{vm_patch_c::vm_patch_c, vm_protect::vm_protect, vm_reg::VM_REG},
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
pub unsafe fn execute_namecall(
  l: *mut lua_State,
  pc: *const Instruction,
  mut base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe {
    let cl = clvalue!((*(*l).ci).func);
    let mut pc_ptr = pc;
    let insn = *pc_ptr;
    pc_ptr = pc_ptr.add(1);
    let op = LUAU_INSN_OP(insn);
    let mut ra = VM_REG!(LUAU_INSN_A(insn) as i32, l, base) as *mut TValue;
    let rb = VM_REG!(LUAU_INSN_B(insn) as i32, l, base) as *mut TValue;
    let aux = *pc_ptr;
    pc_ptr = pc_ptr.add(1);
    let kv = vm_kv(
      if op == LuauOpcode::LOP_NAMECALLUDATA as u32 {
        LUAU_INSN_AUX_KV16(aux)
      } else {
        aux
      },
      cl,
      k,
    );
    ulua_common::LUAU_ASSERT!(ttisstring!(kv as *const TValue));

    if ttistable!(rb as *const TValue) {
      setobj2s!(l, ra.add(1), rb as *const TValue);
      (*l).cachedslot = LUAU_INSN_C(insn) as i32;
      vm_protect!(l, pc_ptr, base, {
        lua_v_gettable(l, rb as *const TValue, kv, ra);
      });
      vm_patch_c(pc_ptr.sub(2), (*l).cachedslot);

      ra = VM_REG!(LUAU_INSN_A(insn) as i32, l, base) as *mut TValue;
      if ttisnil!(ra as *const TValue) {
        luaG_methoderror(l, ra.add(1) as *const TValue, tsvalue!(kv as *const TValue));
      }
    } else {
      let mt = if ttisuserdata!(rb as *const TValue) {
        uvalue!(rb as *const TValue).metatable
      } else {
        (*(*l).global).mt[ttype!(rb as *const TValue) as usize]
      };

      let fn_nc = fasttm(l, mt, TMS::TmNameCall as i32);
      if !fn_nc.is_null() {
        setobj2s!(l, ra.add(1), rb as *const TValue);
        setobj2s!(l, ra, fn_nc);

        (*l).namecall = tsvalue!(kv as *const TValue) as *mut _;
      } else {
        let tmi = fasttm(l, mt, TMS::TmIndex as i32);
        if !tmi.is_null() && ttistable!(tmi) {
          let h = hvalue!(tmi);
          let slot = (LUAU_INSN_C(insn) as i32) & (*h).nodemask8 as i32;
          let n = (*h).node.add(slot as usize);

          if ttisstring!(gkey!(n) as *const TValue)
            && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
            && !ttisnil!(gval!(n))
          {
            setobj2s!(l, ra.add(1), rb as *const TValue);
            setobj2s!(l, ra, gval!(n));
          } else {
            setobj2s!(l, ra.add(1), rb as *const TValue);
            (*l).cachedslot = slot;
            vm_protect!(l, pc_ptr, base, {
              lua_v_gettable(l, rb as *const TValue, kv, ra);
            });
            vm_patch_c(pc_ptr.sub(2), (*l).cachedslot);

            ra = VM_REG!(LUAU_INSN_A(insn) as i32, l, base) as *mut TValue;
            if ttisnil!(ra as *const TValue) {
              luaG_methoderror(l, ra.add(1) as *const TValue, tsvalue!(kv as *const TValue));
            }
          }
        } else {
          setobj2s!(l, ra.add(1), rb as *const TValue);
          vm_protect!(l, pc_ptr, base, {
            lua_v_gettable(l, rb as *const TValue, kv, ra);
          });

          ra = VM_REG!(LUAU_INSN_A(insn) as i32, l, base) as *mut TValue;
          if ttisnil!(ra as *const TValue) {
            luaG_methoderror(l, ra.add(1) as *const TValue, tsvalue!(kv as *const TValue));
          }
        }
      }
    }

    ulua_common::LUAU_ASSERT!(
      LUAU_INSN_OP(*pc_ptr) == LuauOpcode::LOP_CALL as u32
        || LUAU_INSN_OP(*pc_ptr) == LuauOpcode::LOP_CALLFB as u32
    );
    pc_ptr
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeNAMECALL")]
pub unsafe extern "C-unwind" fn execute_namecall_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_namecall(l, pc, base, k) }
}
