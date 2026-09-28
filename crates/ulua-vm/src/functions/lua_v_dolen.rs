use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::{
    call_t_mres::call_t_mres, lua_h_getn::lua_h_getn, lua_t_gettmbyobj::lua_t_gettmbyobj,
  },
  macros::{
    fasttm::fasttm, lua_g_runerror::lua_g_runerror, lua_g_typeerror::luaG_typeerror,
    lua_o_nilobject::LUA_O_NILOBJECT, setnvalue::setnvalue, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 lua_State 且处于可抛错/可 GC 的受保护帧；`ra` 指向 `(*l).stack` 内一个可写栈槽（写回长度结果），
/// `rb` 须指向存活 TValue 且其类型分支所需的元表/字符串字段有效。`call_t_mres` 可能重分配栈，`ra`/`rb` 的存活
/// 由调用方（VM 帧根）保证。cpp/VM/src/lvmutils.cpp:727 luaV_dolen。
pub(crate) unsafe fn lua_v_dolen(l: *mut LuaState, ra: StkId, rb: *const TValue) {
  unsafe {
    let tm: *const TValue = match ttype!(rb) {
      x if x == LuaType::Table as u32 => {
        let h = (*rb).as_table_ptr();
        let tm = fasttm(l, (*h).metatable, TMS::TmLen);
        if tm.is_null() {
          setnvalue!(ra, lua_h_getn(h) as f64);
          return;
        }
        tm
      }
      x if x == LuaType::String as u32 => {
        let ts = (*rb).as_string_ptr();
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
/// 前置条件同 `lua_v_dolen`：`l` 为存活且处于受保护帧的 lua_State，`ra` 为可写栈槽，
/// `rb` 为指向存活 TValue 的合法指针。cpp/VM/src/lvmutils.cpp:727 luaV_dolen。
pub unsafe extern "C-unwind" fn lua_v_dolen_export(l: *mut LuaState, ra: StkId, rb: *const TValue) {
  unsafe {
    lua_v_dolen(l, ra, rb);
  }
}
