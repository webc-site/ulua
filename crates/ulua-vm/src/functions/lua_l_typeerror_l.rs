use crate::{
  functions::{
    cstr_cow, currfuncname::currfuncname, lua_a_toobject::lua_a_toobject,
    lua_t_objtypename::lua_t_objtypename,
  },
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可抛错的受保护帧，函数末尾经 `luaL_error` 组装类型错误并 unwind（返回 `!`）；
/// `narg` 为合法栈索引（`lua_a_toobject` 取对象、`currfuncname` 读当前帧名，前者可空需判 NULL）；`tname` 为期望类型名。
/// cpp/VM/src/laux.cpp:45 luaL_typeerrorL。
pub unsafe fn lua_l_typeerror_l(l: *mut LuaState, narg: i32, tname: &str) -> ! {
  // Safety: 契约保证 l 为存活受保护帧；fname 为切片化的帧名字节（None 即原 NULL 哨兵），
  // lua_a_toobject/lua_t_objtypename 按各自 C-API 契约读槽，luaL_error 抛错不返回
  unsafe {
    let fname = currfuncname(l);
    let obj: *const TValue = lua_a_toobject(l, narg);

    if !obj.is_null() {
      let objtypename = lua_t_objtypename(l, obj);
      let objtypename = cstr_cow(objtypename);

      match fname {
        Some(fname) => luaL_error!(
          l,
          "invalid argument #{} to '{}' ({} expected, got {})",
          narg,
          String::from_utf8_lossy(fname),
          tname,
          objtypename
        ),
        None => luaL_error!(
          l,
          "invalid argument #{} ({} expected, got {})",
          narg,
          tname,
          objtypename
        ),
      }
    } else if let Some(fname) = fname {
      luaL_error!(
        l,
        "missing argument #{} to '{}' ({} expected)",
        narg,
        String::from_utf8_lossy(fname),
        tname
      );
    } else {
      luaL_error!(l, "missing argument #{} ({} expected)", narg, tname);
    }
  }
}
