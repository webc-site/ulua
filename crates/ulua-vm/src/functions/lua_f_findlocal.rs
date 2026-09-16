use core::{ffi::c_int, ptr::null};

use crate::records::{loc_var::LocVar, proto::Proto};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_f_findlocal(f: *const Proto, local_reg: c_int, pc: c_int) -> *const LocVar {
  unsafe {
    let mut i: i32 = 0;
    let sizelocvars = (*f).sizelocvars;
    let locvars = (*f).locvars;

    while i < sizelocvars {
      let locvar = &*locvars.add(i as usize);
      if local_reg == locvar.reg as c_int && pc >= locvar.startpc && pc < locvar.endpc {
        return locvars.add(i as usize) as *const LocVar;
      }
      i += 1;
    }

    null()
  }
}

pub use lua_f_findlocal as luaF_findlocal;
