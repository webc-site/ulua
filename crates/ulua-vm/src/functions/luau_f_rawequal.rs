use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::lua_o_rawequal_obj::luaO_rawequalObj,
  macros::luau_fastmath_end::LUAU_FASTMATH_END,
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_f_rawequal(
  _l: *mut LuaState,
  res: StkId,
  arg0: *mut TValue,
  nresults: c_int,
  args: StkId,
  nparams: c_int,
) -> c_int {
  unsafe {
    LUAU_FASTMATH_END!();

    if nparams >= 2 && nresults <= 1 {
      // setbvalue(res, luaO_rawequalObj(arg0, args));
      let b = luaO_rawequalObj(arg0 as *const TValue, args as *const TValue);
      let i_o: *mut TValue = res;
      (*i_o).value.b = b;
      (*i_o).tt = LuaType::Boolean as i32;
      return 1;
    }

    -1
  }
}

pub use luau_f_rawequal as luauF_rawequal;
