use crate::{
  functions::newkey::newkey,
  macros::{cast_num::cast_num, luai_numeq::luai_numeq, nvalue::nvalue, ttisnumber::ttisnumber},
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn arrayornewkey(
  l: *mut lua_State,
  t: *mut LuaTable,
  key: *const TValue,
) -> *mut TValue {
  unsafe {
    if ttisnumber!(key) {
      let n = nvalue!(key);
      let k = n as i32;

      if luai_numeq(cast_num!(k), n) && (k as u32).wrapping_sub(1) < (*t).sizearray as u32 {
        return (*t).array.add((k - 1) as usize);
      }
    }

    newkey(l, t, key)
  }
}
