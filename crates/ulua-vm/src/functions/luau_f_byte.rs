use core::ffi::c_int;

use crate::{
  macros::{
    getstr::getstr, nvalue::nvalue, setnvalue::setnvalue, tsvalue::tsvalue, ttisnumber::ttisnumber,
    ttisstring::ttisstring,
  },
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_f_byte(
  _l: *mut LuaState,
  res: StkId,
  arg0: *mut TValue,
  nresults: c_int,
  args: StkId,
  nparams: c_int,
) -> c_int {
  unsafe {
    if nparams >= 2 && ttisstring!(arg0) && ttisnumber!(args) {
      let ts = tsvalue!(arg0);
      let i = nvalue!(args) as i32;
      let j = if nparams >= 3 {
        if ttisnumber!(args.add(1)) {
          nvalue!(args.add(1)) as i32
        } else {
          i
        }
      } else {
        i
      };

      if i >= 1 && j >= i && j <= (*ts).len as i32 {
        let c = j - i + 1;
        let s = getstr(ts);

        if c == (if nresults < 0 { 1 } else { nresults }) {
          for k in 0..c {
            setnvalue!(
              res.add(k as usize),
              (*s.add((i + k - 1) as usize)) as u8 as f64
            );
          }

          return c;
        }
      }
    }

    -1
  }
}

pub use luau_f_byte as luauF_byte;
