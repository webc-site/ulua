use core::{ffi::c_int, mem::zeroed};

use crate::{
  functions::{
    getthread::getthread, lua_getinfo::lua_getinfo, lua_gettop::lua_gettop,
    lua_isnumber::lua_isnumber, lua_pushboolean::lua_pushboolean, lua_pushinteger::lua_pushinteger,
    lua_pushstring::lua_pushstring, lua_pushvalue::lua_pushvalue,
    lua_rawcheckstack::lua_rawcheckstack, lua_settop::lua_settop, lua_xmove::lua_xmove,
  },
  macros::{
    lua_isfunction::lua_isfunction, lua_l_argcheck::luaL_argcheck, lua_l_argerror::luaL_argerror,
    lua_l_checkstring::luaL_checkstring, lua_tointeger::lua_tointeger,
  },
  records::lua_debug::LuaDebug,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn db_info(l: *mut lua_State) -> c_int {
  unsafe {
    let mut arg: i32 = 0;
    let l1 = getthread(l, &mut arg);
    let mut l1top: i32 = 0;

    // if l1 != l, l1 can be in any state, and therefore there are no guarantees about its stack space
    if l != l1 {
      // for 'f' option, we reserve one slot and we also record the stack top
      lua_rawcheckstack(l1, 1);
      l1top = lua_gettop(l1);
    }

    let level: i32;
    if lua_isnumber(l, arg + 1) != 0 {
      level = lua_tointeger!(l, arg + 1);
      luaL_argcheck!(l, level >= 0, arg + 1, "level can't be negative");
    } else if arg == 0 && lua_isfunction!(l, 1) {
      // convert absolute index to relative index
      level = -lua_gettop(l);
    } else {
      luaL_argerror!(l, arg + 1, "function or level expected");
    }

    let options = luaL_checkstring!(l, arg + 2);

    let mut ar: LuaDebug = zeroed();
    if lua_getinfo(l1, level, options, &mut ar) == 0 {
      return 0;
    }

    let mut results: i32 = 0;
    let mut occurs = [false; 26];

    let mut it = options;
    while *it != 0 {
      let ch = *it as u8;
      if ch.is_ascii_lowercase() {
        let idx = (ch - b'a') as usize;
        if occurs[idx] {
          // restore stack state of another thread as 'f' option might not have been visited yet
          if l != l1 {
            lua_settop(l1, l1top);
          }

          luaL_argerror!(l, arg + 2, "duplicate option");
        }
        occurs[idx] = true;
      }

      match ch {
        b's' => {
          lua_pushstring(l, ar.short_src);
          results += 1;
        }
        b'l' => {
          lua_pushinteger(l, ar.currentline);
          results += 1;
        }
        b'n' => {
          lua_pushstring(
            l,
            if !ar.name.is_null() {
              ar.name
            } else {
              c"".as_ptr()
            },
          );
          results += 1;
        }
        b'f' => {
          if l1 == l {
            lua_pushvalue(l, -1 - results); // function is right before results
          } else {
            lua_xmove(l1, l, 1); // function is at top of l1
          }
          results += 1;
        }
        b'a' => {
          lua_pushinteger(l, ar.nparams as c_int);
          lua_pushboolean(l, ar.isvararg as c_int);
          results += 2;
        }
        _ => {
          // restore stack state of another thread as 'f' option might not have been visited yet
          if l != l1 {
            lua_settop(l1, l1top);
          }

          luaL_argerror!(l, arg + 2, "invalid option");
        }
      }

      it = it.add(1);
    }

    results
  }
}
