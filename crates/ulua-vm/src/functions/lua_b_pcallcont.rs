use core::ptr::copy;

use crate::{
  functions::lua_rawcheckstack::lua_rawcheckstack,
  macros::{lua_lib_fn::lua_cont_fn, setbvalue::setbvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_b_pcallcont(l: *mut LuaState, status: i32) -> i32 {
  unsafe {
    lua_rawcheckstack(l, 1);
    if status == 0 {
      let base = (*l).base;
      let top = (*l).top;
      let count = top.offset_from(base) as usize;
      if count > 0 {
        copy(base, base.add(1), count);
      }
      setbvalue!(base, 1);
      (*l).top = top.add(1);
      (count + 1) as i32
    } else {
      let top = (*l).top;
      setobj_2_s!(l, top, top.sub(1));
      setbvalue!(top.sub(1), 0);
      (*l).top = top.add(1);
      2
    }
  }
}

lua_cont_fn!(pub(crate) fn lua_b_pcallcont, lua_b_pcallcont_arm);
