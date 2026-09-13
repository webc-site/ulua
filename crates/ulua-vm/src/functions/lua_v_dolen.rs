use core::{
  ffi::{c_int, c_void},
  mem::transmute,
};

use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::{
    call_t_mres::call_t_mres, lua_h_getn::lua_h_getn, lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    cast_num::cast_num, fasttm::fasttm, hvalue::hvalue, lua_g_runerror::lua_g_runerror,
    lua_g_typeerror::luaG_typeerror, lua_o_nilobject::luaO_nilobject, setnvalue::setnvalue,
    tsvalue::tsvalue, ttisnil::ttisnil, ttisnumber::ttisnumber, ttype::ttype,
  },
  records::lua_table::LuaTable,
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_v_dolen(l: *mut LuaState, ra: StkId, rb: *const TValue) {
  unsafe {
    let tm: *const TValue = match ttype!(rb) {
      x if x == LuaType::Table as i32 => {
        let h = hvalue!(rb);
        let tm = fasttm(l, (*h).metatable, TMS::TmLen as i32);
        if tm.is_null() {
          // lua_h_getn stub is currently pub fn lua_h_getn();
          // We must cast it to the real signature to call it with the table pointer.
          let lua_h_getn_ptr = lua_h_getn as *const c_void;
          let lua_h_getn_real: unsafe fn(*mut LuaTable) -> c_int = transmute(lua_h_getn_ptr);
          setnvalue!(ra, cast_num!(lua_h_getn_real(h)));
          return;
        }
        tm
      }
      x if x == LuaType::String as i32 => {
        let ts = tsvalue!(rb);
        setnvalue!(ra, cast_num!((*ts).len));
        return;
      }
      _ => lua_t_gettmbyobj(l, rb, TMS::TmLen),
    };

    if ttisnil!(tm) {
      luaG_typeerror!(l, rb, c"get length of".as_ptr());
    }

    let res = call_t_mres(l, ra, tm, rb, luaO_nilobject);

    if !ttisnumber!(res) {
      lua_g_runerror!(l, "'__len' must return a number");
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaV_dolen")]
pub unsafe extern "C-unwind" fn lua_v_dolen_export(l: *mut LuaState, ra: StkId, rb: *const TValue) {
  unsafe {
    lua_v_dolen(l, ra, rb);
  }
}
