use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_getmetatable::lua_getmetatable, lua_next::lua_next, lua_pushnil::lua_pushnil,
    lua_setreadonly::lua_setreadonly, lua_setsafeenv::lua_setsafeenv, lua_type::lua_type,
  },
  macros::{
    lua_globalsindex::LUA_GLOBALSINDEX, lua_pop::lua_pop, lua_pushliteral::lua_pushliteral,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活、已 openlibs 的 `LuaState` 且处于受保护帧：以 `LUA_GLOBALSINDEX` 为遍历根，`lua_next` 迭代全局表
/// （读写 `(*l).top`，每轮需 ≥2 槽并 `lua_pop` 平衡）；对表项与内建 metatable 调 `lua_setreadonly`、对全局调
/// `lua_setreadonly`/`lua_setsafeenv` 将其冻结。须在加载用户脚本前一次性执行，操作会永久改变 env 只读状态。
/// cpp VM/src/linit.cpp:65
pub unsafe fn lua_l_sandbox(l: *mut LuaState) {
  unsafe {
    // set all libraries to read-only
    lua_pushnil(l);
    while lua_next(l, LUA_GLOBALSINDEX) != 0 {
      // lua_istable! macro uses lua_type internally; we check the type directly.
      if lua_type(l, -1) == LuaType::Table as i32 {
        lua_setreadonly(l, -1, 1);
      }
      lua_pop(l, 1);
    }

    // set all builtin metatables to read-only
    lua_pushliteral(l, b"");
    if lua_getmetatable(l, -1) != 0 {
      lua_setreadonly(l, -1, 1);
      lua_pop(l, 2);
    } else {
      lua_pop(l, 1);
    }

    // set globals to readonly and activate safeenv since the env is immutable
    lua_setreadonly(l, LUA_GLOBALSINDEX, 1);
    lua_setsafeenv(l, LUA_GLOBALSINDEX, 1);
  }
}
