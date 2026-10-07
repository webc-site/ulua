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
      // 保留：错误消息槽的单次现读（cpp 同形最小读面 `setobj2s(L, oldtop, L->top - 1)`）——
      // 前两个分支经字面量串分配，场域必须保持现读语义，无可收编的重复重读
      setobj_2_s!(l, oldtop, (*l).top.offset(-1));
    }

    // 保留（恢复动作本体）：`oldtop` 是受保护恢复路径（lua_d_pcall/recover 族）经
    // restorestack 重派生后跨帧传入的保存槽，本行 `top = oldtop + 1` 即错误收尾的
    // 栈顶恢复动作，cpp 同形 `L->top = oldtop + 1;`；窗口跨调用方恢复点，禁就地重排
    (*l).top = oldtop.offset(1);
  }
}
