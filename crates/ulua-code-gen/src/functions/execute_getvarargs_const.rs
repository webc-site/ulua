use ulua_vm::{
  macros::{clvalue::clvalue, setnilvalue::setnilvalue, setobj_2_s::setobj2s},
  records::lua_state::lua_State,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::macros::vm_reg::VM_REG;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn execute_getvarargs_const(l: *mut lua_State, base: StkId, rai: i32, b: i32) {
  unsafe {
    let l_ptr = l;
    let cl = clvalue!((*(*l_ptr).ci).func);
    let p = {
      let l = &(*cl).inner.l;
      l.p
    };
    let n = base.offset_from((*(*l_ptr).ci).func) as i32 - (*p).numparams as i32 - 1;

    let ra = VM_REG!(rai, l_ptr, base) as *mut TValue;

    let mut j = 0;
    while j < b && j < n {
      setobj2s!(
        l_ptr,
        ra.add(j as usize),
        base.sub(n as usize).add(j as usize) as *const TValue
      );
      j += 1;
    }
    let mut j = n;
    while j < b {
      setnilvalue!(ra.add(j as usize));
      j += 1;
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_executeGETVARARGSConst")]
pub unsafe extern "C-unwind" fn execute_getvarargsconst(
  l: *mut lua_State,
  base: StkId,
  rai: i32,
  b: i32,
) {
  unsafe {
    execute_getvarargs_const(l, base, rai, b);
  }
}
