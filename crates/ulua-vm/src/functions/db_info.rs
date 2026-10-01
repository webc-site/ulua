use core::mem::zeroed;

use crate::{
  functions::{
    cstr_bytes, getthread::getthread, lua_getinfo::lua_getinfo, lua_isnumber::lua_isnumber,
    lua_rawcheckstack::lua_rawcheckstack, lua_xmove::lua_xmove,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`getthread` 取回的 `l1` 须为存活线程，`l!=l1` 时先 `rawcheckstack(l1,1)`
/// 预留槽；栈 `arg+1` 为 level 数或函数、`arg+2` 为 NUL 结尾选项串（`luaL_checkstring`），
/// `lua_getinfo` 可写 `ar` 并可抛错，须在受保护帧内调用。cpp `ldblib.cpp:25`。
pub(crate) unsafe fn db_info(l: *mut LuaState) -> i32 {
  unsafe {
    let (l1, arg) = getthread(l);
    let mut l1top: i32 = 0;

    // if l1 != l, l1 can be in any state, and therefore there are no guarantees about its stack space
    if l != l1 {
      // for 'f' option, we reserve one slot and we also record the stack top
      lua_rawcheckstack(l1, 1);
      l1top = (*l1).get_top();
    }

    let level: i32;
    if lua_isnumber(&*l, arg + 1) != 0 {
      level = (*l).to_integer(arg + 1).unwrap_or(0);
      (*l).arg_check(level >= 0, arg + 1, "level can't be negative");
    } else if arg == 0 && (*l).is_function(1) {
      // convert absolute index to relative index
      level = -(*l).get_top();
    } else {
      (*l).arg_error(arg + 1, "function or level expected");
    }

    let options = (*l).check_bytes(arg + 2);

    let mut ar: LuaDebug = zeroed();
    if lua_getinfo(l1, level, options.as_ptr().cast(), &mut ar) == 0 {
      return 0;
    }

    let mut results: i32 = 0;
    let mut occurs = [false; 26];

    // C++ `for (; *it; it++)`：零拷贝迭代选项串
    for ch in options.iter().copied() {
      if ch.is_ascii_lowercase() {
        let idx = (ch - b'a') as usize;
        if occurs[idx] {
          // restore stack state of another thread as 'f' option might not have been visited yet
          if l != l1 {
            (*l1).set_top(l1top);
          }

          (*l).arg_error(arg + 2, "duplicate option");
        }
        occurs[idx] = true;
      }

      match ch {
        b's' => {
          (*l).push_bytes(cstr_bytes(ar.short_src));
          results += 1;
        }
        b'l' => {
          (*l).push_integer(ar.currentline);
          results += 1;
        }
        b'n' => {
          let name = if !ar.name.is_null() {
            cstr_bytes(ar.name)
          } else {
            b"".as_slice()
          };
          (*l).push_bytes(name);
          results += 1;
        }
        b'f' => {
          if l1 == l {
            (*l).push_value(-1 - results); // function is right before results
          } else {
            lua_xmove(l1, l, 1); // function is at top of l1
          }
          results += 1;
        }
        b'a' => {
          (*l).push_integer(ar.nparams as i32);
          (*l).push_boolean(ar.isvararg != 0);
          results += 2;
        }
        _ => {
          // restore stack state of another thread as 'f' option might not have been visited yet
          if l != l1 {
            (*l1).set_top(l1top);
          }

          (*l).arg_error(arg + 2, "invalid option");
        }
      }
    }

    results
  }
}

lua_lib_fn!(pub(crate) fn db_info, db_info_arm);
