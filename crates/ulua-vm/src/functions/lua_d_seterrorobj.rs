use crate::{
  enums::lua_status::LuaStatus,
  macros::{
    lua_errerrmsg::LUA_ERRERRMSG_STR, lua_memerrmsg::LUA_MEMERRMSG_STR,
    lua_s_newliteral::lua_s_newliteral, setobj_2_s::setobj_2_s, setsvalue::setsvalue,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`oldtop` 须为栈内合法 `StkId`（错误前保存的 top），`errcode` 为 LuaStatus；
/// ErrSyntax/ErrRun 分支要求当前 `(*l).top-1` 处确有错误消息 TValue 可读；`setsvalue!`/`lua_s_newliteral` 写
/// `oldtop` 槽可能分配字面量串（可触发 GC）；末尾把 `(*l).top` 截为 `oldtop+1`。由受保护的恢复路径调用。
/// cpp VM/src/ldo.cpp:408
pub unsafe fn lua_d_seterrorobj(l: *mut LuaState, errcode: i32, oldtop: StkId) {
  unsafe {
    if errcode == LuaStatus::ErrMem as i32 {
      setsvalue!(l, oldtop, lua_s_newliteral(l, LUA_MEMERRMSG_STR.as_bytes()));
    } else if errcode == LuaStatus::ErrErr as i32 {
      setsvalue!(l, oldtop, lua_s_newliteral(l, LUA_ERRERRMSG_STR.as_bytes()));
    } else if errcode == LuaStatus::ErrSyntax as i32 || errcode == LuaStatus::ErrRun as i32 {
      // error message on current top
      setobj_2_s!(l, oldtop, (*l).top.offset(-1));
    }

    (*l).top = oldtop.offset(1);
  }
}
