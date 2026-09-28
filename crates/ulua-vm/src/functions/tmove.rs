use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_resizearray::lua_h_resizearray,
    moveelements::moveelements,
  },
  macros::{lua_lib_fn::lua_lib_fn, sizenode::sizenode},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn tmove(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    let f = (*l).check_integer(2);
    let e = (*l).check_integer(3);
    let t = (*l).check_integer(4);
    let tt = if !(*l).is_none_or_nil(5) { 5 } else { 1 };

    (*l).check_type(tt, LuaType::Table);

    if e >= f {
      (*l).arg_check(f > 0 || e < i32::MAX + f, 3, "too many elements to move");
      let n = e - f + 1;
      (*l).arg_check(t <= i32::MAX - n + 1, 4, "destination wrap around");

      let src = (*(*l).base).as_table_ptr();
      let dst = (*(*l).base.offset((tt - 1) as isize)).as_table_ptr();

      check_writable(l, dst);

      let srcelems = (*src).sizearray + sizenode!(src);
      let dstelems = (*dst).sizearray + sizenode!(dst);
      let maxelems = srcelems.max(dstelems);
      let minsparsemoveelems = 32;
      let sparsemove = n > minsparsemoveelems && n / 2 > maxelems;

      if t > 0 && (t - 1) <= (*dst).sizearray && (t - 1 + n) > (*dst).sizearray {
        lua_h_resizearray(l, dst, t - 1 + n);
      }

      moveelements(l, 1, tt, f, e, t, sparsemove);
    }

    (*l).push_value(tt);
    1
  }
}

lua_lib_fn!(pub(crate) fn tmove, tmove_arm);
