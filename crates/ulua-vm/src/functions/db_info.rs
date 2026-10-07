use crate::{
  functions::{
    getthread::getthread, lua_getinfo::lua_getinfo, lua_rawcheckstack::lua_rawcheckstack,
    lua_xmove::lua_xmove,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState`；`getthread` 取回的 `l1` 须为存活线程，`l!=l1` 时先 `rawcheckstack(l1,1)`
/// 预留槽；栈 `arg+1` 为 level 数或函数、`arg+2` 为 NUL 结尾选项串（`luaL_checkstring`），
/// `lua_getinfo` 可写 `ar` 并可抛错，须在受保护帧内调用。cpp `ldblib.cpp:25`。
pub(crate) unsafe fn db_info(l: *mut LuaState) -> i32 {
  // r13-w1a 逐点定性（w6d 口径保留面；原普查 14 行命中）：to_integer/arg_check/
  // is_function/get_top/arg_error×3/check_bytes/push_bytes×2/push_integer×2/
  // push_value/push_boolean 皆为 records/lua_state 既有方法收编形态（前波 safe-ify
  // 已收），零翻案、零新造门面；getthread/rawcheckstack/lua_getinfo/lua_xmove 为
  // 裸指针取参自由函数调用点，非解引用收编面，原样保留；线程侧 l1 的 get_top/
  // set_top 同为既有方法调用。本席新增收编一处（该行收编后普查计 15 行，系真实
  // 门面调用行非注记膨胀）：level 数值判定由 lua_isnumber 自由函数 + !=0 判定
  // 换用 access.rs 既有 is_number 方法——该方法即原调用的 inline(always) 字面
  // 转发，谓词逐位恒等、读数位点不变（use 头部同步去 lua_isnumber 导入）。
  unsafe {
    let (l1, arg) = getthread(l);
    let mut l1top: i32 = 0;

    // if l1 != l, l1 can be in any state, and therefore there are no guarantees about its stack space
    if l != l1 {
      // for 'f' option, we reserve one slot and we also record the stack top
      lua_rawcheckstack(&mut *l1, 1);
      l1top = (*l1).get_top();
    }

    let level: i32;
    if (*l).is_number(arg + 1) {
      level = (*l).to_integer(arg + 1).unwrap_or(0);
      (*l).arg_check(level >= 0, arg + 1, "level can't be negative");
    } else if arg == 0 && (*l).is_function(1) {
      // convert absolute index to relative index
      level = -(*l).get_top();
    } else {
      (*l).arg_error(arg + 1, "function or level expected");
    }

    let options = (*l).check_bytes(arg + 2);

    let mut ar: LuaDebug = LuaDebug::default();
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
          (*l).push_bytes(ar.short_src.as_deref().unwrap_or(b""));
          results += 1;
        }
        b'l' => {
          (*l).push_integer(ar.currentline);
          results += 1;
        }
        b'n' => {
          (*l).push_bytes(ar.name.as_deref().unwrap_or(b""));
          results += 1;
        }
        b'f' => {
          if l1 == l {
            (*l).push_value(-1 - results); // function is right before results
          } else {
            lua_xmove(&mut *l1, &mut *l, 1); // function is at top of l1
          }
          results += 1;
        }
        b'a' => {
          (*l).push_integer(ar.nparams as i32);
          (*l).push_boolean(ar.isvararg);
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
