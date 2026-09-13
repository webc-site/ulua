use ulua_vm::{
  functions::lua_d_growstack::lua_d_growstack,
  macros::{clvalue::clvalue, setobj_2_s::setobj2s, stacklimitreached::stacklimitreached},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  macros::{vm_protect::vm_protect, vm_reg::VM_REG},
  type_aliases::{instruction_ir_builder::Instruction, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_getvarargs_mult_ret(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  rai: i32,
) {
  unsafe {
    let l_ptr = l;
    let cl = clvalue!((*(*l_ptr).ci).func);
    let p = {
      let l = &(*cl).inner.l;
      l.p
    };
    let n = base.offset_from((*(*l_ptr).ci).func) as i32 - (*p).numparams as i32 - 1;

    let current_base: StkId;
    vm_protect!(l_ptr, pc, current_base, {
      if stacklimitreached(l_ptr, n) {
        lua_d_growstack(l_ptr, n);
      }
    });

    let ra = VM_REG!(rai, l_ptr, current_base) as *mut TValue;
    // setobj2s 为逐元素拷贝并带 checkliveness，不可合并为 ptr::copy
    (0..n).for_each(|j| {
      setobj2s!(
        l_ptr,
        ra.add(j as usize),
        current_base.sub(n as usize).add(j as usize) as *const TValue
      );
    });

    (*l_ptr).top = ra.add(n as usize);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeGETVARARGSMultRet")]
pub unsafe extern "C-unwind" fn execute_getvarargsmult_ret(
  l: *mut lua_State,
  pc: *const Instruction,
  base: StkId,
  rai: i32,
) {
  unsafe {
    execute_getvarargs_mult_ret(l, pc, base, rai);
  }
}
