use core::{ffi::c_int, mem::size_of};

use crate::{
  functions::lua_m_realloc::lua_m_realloc_, records::call_info::CallInfo,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_realloc_ci(l: *mut lua_State, newsize: c_int) {
  unsafe {
    let oldci = (*l).base_ci;
    let oldoffset = (*l).ci.offset_from(oldci);

    (*l).base_ci = lua_m_realloc_(
      l,
      (*l).base_ci as *mut u8,
      (*l).size_ci as usize * size_of::<CallInfo>(),
      newsize as usize * size_of::<CallInfo>(),
      (*l).hdr.memcat,
    ) as *mut CallInfo;

    (*l).size_ci = newsize;
    (*l).ci = (*l).base_ci.offset(oldoffset);
    (*l).end_ci = (*l).base_ci.add(((*l).size_ci - 1) as usize);
  }
}

pub use lua_d_realloc_ci as luaD_reallocCI;
