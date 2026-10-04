use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_checkstack::lua_checkstack, lua_rawcheckstack::lua_rawcheckstack, lua_xmove::lua_xmove,
  },
  macros::{co_status_error::CO_STATUS_ERROR, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 与 `co` 须为**互不别名**的存活 `LuaState` 指针（`co` 为刚结束 resume 的协程，不得等于 `l`，
/// 否则两侧 `&mut` 重建即同一对象两用借用），且调用点处于可抛错、可 GC 的受保护帧：本函数只读
/// `(*co).status`，随后按 `nres`（= `co` 顶-基槽距，须 ≥0）在 `l`/`co` 间 `lua_xmove`，故 `co` 的
/// `base..top` 窗须恰有 nres 个可读结果槽、`l` 侧须能容 nres + 1 槽（不足即经 `lua_l_error_l` 抛错
/// 发散，或经 `lua_rawcheckstack` 强制扩容）。status 判定按 `u8` 原值比较（`LuaState::status()`
/// 门面把越界码折成 `Ok`，会翻改未知值的分支走向，故此处不走门面）。首参未收形为 `&mut LuaState`：
/// 其并发消费方 `auxwrapcont`（本批只许读）仍以裸指针调用，收形会外溢到该文件。返回 CO_STATUS_* 码。
/// cpp VM/src/lcorolib.cpp:180 auxresumecont。
pub unsafe fn auxresumecont(l: *mut LuaState, co: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l`/`co` 互不别名且均存活；本体内每次 `&mut *` 重建的借用窗止于当次调用，
  // 指针不外传、不跨调用持有
  unsafe {
    if (*co).status == LuaStatus::Ok as u8 || (*co).status == LuaStatus::Yield as u8 {
      // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
      // 即被替代式 `top.offset_from(base) as i32` 的同址同宽镜像（现读位点不变）；
      // isize→i32 折形在现域无截差（协程栈槽距受 LUAI_MAXSTACK 约束、远小于 i32::MAX）
      let nres = (*co).get_top();
      if lua_checkstack(&mut *l, nres + 1) == 0 {
        luaL_error!(&mut *l, "too many results to resume");
      }
      lua_xmove(&mut *co, &mut *l, nres);
      nres
    } else {
      lua_rawcheckstack(&mut *l, 2);
      lua_xmove(&mut *co, &mut *l, 1);
      CO_STATUS_ERROR
    }
  }
}
