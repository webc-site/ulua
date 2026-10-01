use crate::{
  functions::{lua_setsafeenv::lua_setsafeenv, vector_shared::vector_push},
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
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
    (*l).push_nil();
    while (*l).next(LUA_GLOBALSINDEX) {
      // lua_istable! macro uses lua_type internally; we check the type directly.
      if (*l).is_table(-1) {
        (*l).set_readonly(-1, true);
      }
      (*l).pop(1);
    }

    // set all builtin metatables to read-only
    (*l).push_bytes(b"");
    if (*l).get_metatable(-1) {
      (*l).set_readonly(-1, true);
      (*l).pop(2);
    } else {
      (*l).pop(1);
    }

    vector_push(l, [0.0, 0.0, 0.0, 0.0]);
    if (*l).get_metatable(-1) {
      (*l).set_readonly(-1, true);
      (*l).pop(2);
    } else {
      (*l).pop(1);
    }

    // set globals to readonly and activate safeenv since the env is immutable
    (*l).set_readonly(LUA_GLOBALSINDEX, true);
    lua_setsafeenv(l, LUA_GLOBALSINDEX, 1);
  }
}
