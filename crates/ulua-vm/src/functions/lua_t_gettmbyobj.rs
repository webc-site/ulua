use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::lua_h_getstr::lua_h_getstr,
  macros::{
    hvalue::hvalue, lua_o_nilobject::luaO_nilobject, objectvalue::objectvalue, ttype::ttype,
    uvalue::uvalue,
  },
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn lua_t_gettmbyobj(
  l: *mut lua_State,
  o: *const TValue,
  event: TMS,
) -> *const TValue {
  unsafe {
    /*
      NB: Tag-methods were replaced by meta-methods in Lua 5.0, but the
      old names are still around (this function, for example).
    */

    let mt: *mut LuaTable = match ttype!(o) {
      t if t == LuaType::Table as i32 => (*hvalue!(o)).metatable,
      t if t == LuaType::UserData as i32 => uvalue!(o).metatable,
      t if t == LuaType::Object as i32 => (*objectvalue!(o).lclass).instancemetatable,
      _ => (*(*l).global).mt[ttype!(o) as usize],
    };

    if !mt.is_null() {
      lua_h_getstr(mt, (*(*l).global).tmname[event as usize])
    } else {
      luaO_nilobject
    }
  }
}
