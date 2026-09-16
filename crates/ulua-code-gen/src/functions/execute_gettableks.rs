use core::{ffi::c_void, mem::transmute};

use ulua_common::{
  FFlag::LuauDirectFieldGet,
  enums::luau_opcode::LuauOpcode,
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16, luau_insn_b::LUAU_INSN_B,
    luau_insn_c::LUAU_INSN_C, luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::{
    lua_h_getstr::lua_h_getstr, lua_v_call_tm::lua_v_call_tm, lua_v_gettable::lua_v_gettable,
  },
  macros::{
    clvalue::clvalue,
    fasttm::fasttm,
    getstr::getstr,
    gkey::{gkey, gval},
    gval_2_slot::gval2slot,
    hvalue::hvalue,
    lua_o_nilobject::LUA_O_NILOBJECT,
    lua_vector_size::LUA_VECTOR_SIZE,
    pvalue::pvalue,
    setnvalue::setnvalue,
    setobj_2_s::setobj_2_s as setobj2s,
    tsvalue::tsvalue,
    ttisfunction::ttisfunction,
    ttisnil::ttisnil,
    ttisstring::ttisstring,
    ttistable::ttistable,
    ttisuserdata::ttisuserdata,
    ttisvector::ttisvector,
    uvalue::uvalue,
    vvalue::vvalue,
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
pub unsafe fn execute_gettableks(
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
      if op == LuauOpcode::LOP_GETUDATAKS as u32 {
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

      if (*h).metatable.is_null() {
        let res = lua_h_getstr(h, tsvalue!(kv as *const TValue) as *mut _);

        if res != LUA_O_NILOBJECT {
          vm_patch_c(pc_ptr.sub(2), gval2slot!(h, res));
        }

        setobj2s!(l, ra, res);
        return pc_ptr;
      }

      let slot = (LUAU_INSN_C(insn) as i32) & (*h).nodemask8 as i32;
      (*l).cachedslot = slot;
      vm_protect!(l, pc_ptr, {
        lua_v_gettable(l, rb as *const TValue, kv, ra);
      });
      vm_patch_c(pc_ptr.sub(2), (*l).cachedslot);
      return pc_ptr;
    }

    if LuauDirectFieldGet.get() && ttisuserdata!(rb as *const TValue) {
      let dispatch = {
        let u = uvalue!(rb as *const TValue);
        (*(*l).global).udatadirectfields[u.tag as usize]
      };

      if !dispatch.is_null() {
        let slot = (LUAU_INSN_C(insn) as i32) & (*dispatch).nodemask8 as i32;
        let n = (*dispatch).node.add(slot as usize);

        if ttisstring!(gkey!(n) as *const TValue)
          && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
          && !ttisnil!(gval!(n))
        {
          let f: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void) =
            transmute(pvalue!(gval!(n) as *const TValue));
          let u = uvalue!(rb as *const TValue);
          f(u.data.as_ptr() as *mut c_void, ra as *mut c_void);
          return pc_ptr;
        }

        let fptr = lua_h_getstr(dispatch, tsvalue!(kv as *const TValue) as *mut _);
        if !ttisnil!(fptr) {
          vm_patch_c(pc_ptr.sub(2), gval2slot!(dispatch, fptr));
          let f: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void) = transmute(pvalue!(fptr));
          let u = uvalue!(rb as *const TValue);
          f(u.data.as_ptr() as *mut c_void, ra as *mut c_void);
          return pc_ptr;
        }
      }
    }

    let mut fn_tm: *const TValue;
    if ttisuserdata!(rb as *const TValue)
      && {
        fn_tm = fasttm(
          l,
          uvalue!(rb as *const TValue).metatable,
          TMS::TmIndex as i32,
        );
        !fn_tm.is_null()
      }
      && ttisfunction!(fn_tm)
      && (*clvalue!(fn_tm)).is_c != 0
    {
      ulua_common::LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
      let top = (*l).top;
      setobj2s!(l, top.add(0), fn_tm);
      setobj2s!(l, top.add(1), rb as *const TValue);
      setobj2s!(l, top.add(2), kv as *const TValue);
      (*l).top = top.add(3);

      (*l).cachedslot = LUAU_INSN_C(insn) as i32;
      vm_protect!(l, pc_ptr, {
        lua_v_call_tm(l, 2, LUAU_INSN_A(insn) as i32);
      });
      vm_patch_c(pc_ptr.sub(2), (*l).cachedslot);
      return pc_ptr;
    } else if ttisvector!(rb as *const TValue) {
      let name = getstr(tsvalue!(kv as *const TValue));
      let ic = ((*name.add(0)) as u8 | b' ') as i32 - b'x' as i32;

      if (ic as u32) < LUA_VECTOR_SIZE as u32 && *name.add(1) == 0 {
        let v = vvalue!(rb as *const TValue).as_ptr();
        setnvalue!(ra, *v.add(ic as usize) as f64);
        return pc_ptr;
      }

      fn_tm = fasttm(
        l,
        (*(*l).global).mt[LuaType::Vector as usize],
        TMS::TmIndex as i32,
      );

      if !fn_tm.is_null() && ttisfunction!(fn_tm) && (*clvalue!(fn_tm)).is_c != 0 {
        ulua_common::LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
        let top = (*l).top;
        setobj2s!(l, top.add(0), fn_tm);
        setobj2s!(l, top.add(1), rb as *const TValue);
        setobj2s!(l, top.add(2), kv as *const TValue);
        (*l).top = top.add(3);

        (*l).cachedslot = LUAU_INSN_C(insn) as i32;
        vm_protect!(l, pc_ptr, {
          lua_v_call_tm(l, 2, LUAU_INSN_A(insn) as i32);
        });
        vm_patch_c(pc_ptr.sub(2), (*l).cachedslot);
        return pc_ptr;
      }
    }

    vm_protect!(l, pc_ptr, {
      lua_v_gettable(l, rb as *const TValue, kv, ra);
    });
    pc_ptr
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeGETTABLEKS")]
pub unsafe extern "C-unwind" fn execute_gettableks_export(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  unsafe { execute_gettableks(l, pc, base, k) }
}
