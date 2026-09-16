use core::ffi::c_int;

use crate::{
  macros::{
    luai_num_2_unsigned::luai_num2unsigned, nvalue::nvalue, setnvalue::setnvalue,
    ttisnumber::ttisnumber,
  },
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_f_extract(
  _l: *mut LuaState,
  res: StkId,
  arg0: *mut TValue,
  nresults: c_int,
  args: StkId,
  nparams: c_int,
) -> c_int {
  unsafe {
    if nparams >= 2 && nresults <= 1 && ttisnumber!(arg0) && ttisnumber!(args) {
      let a1 = nvalue!(arg0);
      let a2 = nvalue!(args);

      let mut n: u32 = 0;
      luai_num2unsigned(&mut n, a1);
      let f = a2 as i32;

      if nparams == 2 {
        if (f as u32) < 32 {
          let m: u32 = 1;
          let r: u32 = (n >> (f as u32)) & m;

          setnvalue!(res, r as f64);
          return 1;
        }
      } else if ttisnumber!(args.offset(1)) {
        let a3 = nvalue!(args.offset(1));
        let w = a3 as i32;

        if f >= 0 && w > 0 && f as i64 + w as i64 <= 32 {
          let m: u32 = !(0xFFFF_FFFE_u32 << (w - 1));
          let r: u32 = (n >> (f as u32)) & m;

          setnvalue!(res, r as f64);
          return 1;
        }
      }
    }

    -1
  }
}

pub use luau_f_extract as luauF_extract;
