use core::ptr::null;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    arrayindex::arrayindex, lua_g_runerror_l::lua_g_runerror_l,
    lua_o_rawequal_key::luaO_rawequalKey, mainposition::mainposition,
  },
  macros::{
    cast_int::cast_int, gcvalue::gcvalue, gkey::gkey, gnode::gnode, iscollectable::iscollectable,
    nvalue::nvalue, ttisnil::ttisnil, ttisnumber::ttisnumber, ttype::ttype,
  },
  records::lua_node::LuaNode,
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, stk_id::StkId},
};

pub(crate) unsafe fn findindex(l: *mut lua_State, t: *mut LuaTable, key: StkId) -> i32 {
  unsafe {
    let mut i: i32;

    if ttisnil!(key) {
      return -1; // first iteration
    }

    i = if ttisnumber!(key) {
      arrayindex(nvalue!(key))
    } else {
      -1
    };

    if i > 0 && i <= (*t).sizearray {
      i - 1 // yes; that's the index (corrected to C)
    } else {
      let mut n: *mut LuaNode = mainposition(t, key as *const _);

      loop {
        // check whether `key' is somewhere in the chain
        // key may be dead already, but it is ok to use it in `next'
        if luaO_rawequalKey(gkey!(n) as *const _, key as *const _) != 0
          || (ttype!(gkey!(n)) == LuaType::DeadKey as i32
            && iscollectable!(key)
            && gcvalue!(gkey!(n)) == gcvalue!(key))
        {
          i = cast_int!(n.offset_from(gnode!(t, 0)) as i32);
          // hash elements are numbered after array ones
          return i + (*t).sizearray;
        }

        // gnext(n) is defined as ((n)->key.next) in ltable.h
        let next_offset = (*n).key.next();

        if next_offset == 0 {
          break;
        }
        n = n.offset(next_offset as isize);
      }

      lua_g_runerror_l(l, null(), format_args!("invalid key to 'next'"));
    }
  }
}
