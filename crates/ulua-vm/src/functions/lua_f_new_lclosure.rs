use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::luaM_newgco_,
  macros::{lua_c_init::luaC_init, setnilvalue::setnilvalue, size_lclosure::size_lclosure},
  records::{closure::Closure, lua_table::LuaTable, proto::Proto},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_new_lclosure(
  l: *mut lua_State,
  nelems: c_int,
  e: *mut LuaTable,
  p: *mut Proto,
) -> *mut Closure {
  unsafe {
    let c = luaM_newgco_(l, size_lclosure(nelems as usize), (*l).activememcat) as *mut Closure;

    luaC_init!(l, c, LuaType::Function as c_int);
    (*c).is_c = 0;
    (*c).env = e;
    (*c).nupvalues = nelems as u8;
    (*c).stacksize = (*p).maxstacksize;
    (*c).preload = 0;
    (*c).usage = 0;
    (*c).gclist = null_mut();
    let lc = core::ptr::addr_of_mut!((*c).inner.l);
    (*lc).p = p;

    let mut i = 0;
    while i < nelems {
      setnilvalue!((*lc).uprefs.as_mut_ptr().add(i as usize));
      i += 1;
    }

    c
  }
}

pub use lua_f_new_lclosure as luaF_newLclosure;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaF_newLclosure")]
pub unsafe extern "C-unwind" fn lua_f_new_lclosure_export(
  l: *mut lua_State,
  nelems: c_int,
  e: *mut c_void,
  p: *mut c_void,
) -> *mut c_void {
  unsafe { lua_f_new_lclosure(l, nelems, e as *mut LuaTable, p as *mut Proto).cast() }
}
