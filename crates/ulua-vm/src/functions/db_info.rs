use core::mem::zeroed;

use crate::{
  functions::{
    cstr_bytes, getthread::getthread, lua_getinfo::lua_getinfo, lua_gettop::lua_gettop,
    lua_isnumber::lua_isnumber, lua_pushboolean::lua_pushboolean, lua_pushinteger::lua_pushinteger,
    lua_pushstring::lua_pushstring, lua_pushvalue::lua_pushvalue,
    lua_rawcheckstack::lua_rawcheckstack, lua_settop::lua_settop, lua_xmove::lua_xmove,
  },
  macros::{
    lua_emptystr::LUA_EMPTYSTR, lua_isfunction::lua_isfunction, lua_l_argcheck::luaL_argcheck,
    lua_l_argerror::luaL_argerror, lua_l_checkstring::luaL_checkstring,
    lua_tointeger::lua_tointeger,
  },
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`getthread` 取回的 `l1` 须为存活线程，`l!=l1` 时先 `rawcheckstack(l1,1)`
/// 预留槽；栈 `arg+1` 为 level 数或函数、`arg+2` 为 NUL 结尾选项串（`luaL_checkstring`），
/// `lua_getinfo` 可写 `ar` 并可抛错，须在受保护帧内调用。cpp `ldblib.cpp:25`。
pub(crate) unsafe extern "C-unwind" fn db_info(l: *mut LuaState) -> i32 {
  unsafe {
    let (l1, arg) = getthread(l);
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

    // C++ `for (; *it; it++)`：CStr 零拷贝迭代选项串
    for ch in cstr_bytes(options).iter().copied() {
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
              LUA_EMPTYSTR.as_ptr().cast()
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
          lua_pushinteger(l, ar.nparams as i32);
          lua_pushboolean(l, ar.isvararg as i32);
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
    }

    results
  }
}
