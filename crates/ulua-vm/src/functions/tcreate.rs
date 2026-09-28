use crate::{
  functions::{c_slice_mut, lua_createtable::lua_createtable},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
  setobj2t,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tcreate(l: *mut LuaState) -> i32 {
  unsafe {
    let size = (*l).check_integer(1);
    if size < 0 {
      (*l).arg_error(1, "size out of range");
    }

    if !(*l).is_none_or_nil(2) {
      lua_createtable(l, size, 0);
      let t = (*(*l).top.offset(-1)).as_table_ptr();

      let v: StkId = (*l).base.add(1);

      for e in c_slice_mut((*t).array, size as usize) {
        setobj2t!(l, e as *mut TValue, v);
      }
    } else {
      lua_createtable(l, size, 0);
    }

    1
  }
}

lua_lib_fn!(pub fn tcreate, tcreate_arm);
