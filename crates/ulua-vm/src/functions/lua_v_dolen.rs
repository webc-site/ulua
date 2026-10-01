use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::{
    call_t_mres::call_t_mres, lua_h_getn::lua_h_getn, lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    fasttm::fasttm, lua_g_runerror::lua_g_runerror, lua_g_typeerror::luaG_typeerror,
    lua_o_nilobject::LUA_O_NILOBJECT, setnvalue::setnvalue, ttype::ttype,
  },
  records::{lua_state::LuaState, slot::Slot},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 lua_State 且处于可抛错/可 GC 的受保护帧；`ra` 指向 `(*l).stack` 内一个可写栈槽
/// （写回长度结果），`rb` 引用保证可读且其类型分支所需的元表/字符串字段有效。`call_t_mres`
/// 可能重分配栈，句柄/引用所指槽的存活由调用方（VM 帧根）保证，服从「扩容先行、借用后派生」
/// 不变量——本函数内不重取栈基址，槽地址在调用期间不得迁移。
/// cpp/VM/src/lvmutils.cpp:727 luaV_dolen。
pub(crate) unsafe fn lua_v_dolen(l: *mut LuaState, ra: Slot<'_>, rb: &TValue) {
  // SAFETY: 契约保证 `l` 为存活调用帧、`ra` 为可写栈槽句柄且 `rb` 可读对齐，块内取值、TM 调用
  // 与报错路径均在该帧栈界内
  unsafe {
    // 句柄落回裸槽视图一处：`ra` 在本体内仅作 `setnvalue!`/`call_t_mres` 的写侧槽地址，
    // `as_ptr` 为 `inline(always)` 指针读出，与原 `StkId` 形参同址同宽度（§9.4）
    let ra = ra.as_ptr();

    let tm: *const TValue = match ttype!(rb) {
      x if x == LuaType::Table as u32 => {
        let h = rb.as_table_ptr();
        let tm = fasttm(l, (*h).metatable, TMS::TmLen);
        if tm.is_null() {
          setnvalue!(ra, lua_h_getn(h) as f64);
          return;
        }
        tm
      }
      x if x == LuaType::String as u32 => {
        let ts = rb.as_string_ptr();
        setnvalue!(ra, (*ts).len as f64);
        return;
      }
      _ => lua_t_gettmbyobj(l, rb, TMS::TmLen),
    };

    if (*tm).is_nil() {
      luaG_typeerror!(l, rb, "get length of");
    }

    let res = call_t_mres(l, ra, tm, rb, LUA_O_NILOBJECT);

    if !(*res).is_number() {
      lua_g_runerror!(l, "'__len' must return a number");
    }
  }
}

/// # Safety
/// C ABI 导出壳：签名与符号不动。前置条件同 `lua_v_dolen`：`l` 为存活且处于受保护帧的
/// lua_State，`ra` 为非空可写栈槽、`rb` 为指向存活对齐 TValue 的合法指针（被调方仅用读面），
/// 调用期间槽地址不迁移。cpp/VM/src/lvmutils.cpp:727 luaV_dolen。
pub unsafe extern "C-unwind" fn lua_v_dolen_export(l: *mut LuaState, ra: StkId, rb: *const TValue) {
  // SAFETY: 导出壳在边界显式重建句柄/只读借用后转调同契约 `lua_v_dolen`；
  // `ra` 为可写栈槽地址（写面前提满足 `from_raw` 纪律），`rb` 解引用窗口止于本调用
  unsafe {
    lua_v_dolen(l, Slot::from_raw(ra), &*rb);
  }
}
