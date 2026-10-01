use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_h_getstr::lua_h_getstr,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_o_nilobject::LUA_O_NILOBJECT,
    lua_s_new::lua_s_new, setobj_2_s::setobj_2_s, setsvalue::setsvalue, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// `lua_rawgetfield` 核心（cpp `lapi.cpp:903`）。调用序契约（正确性，非内存安全）：
/// `l` 处于受保护帧，`ensure_stack(l, 1)` 留 1 结果槽；`idx` 解析出的槽须为 table
/// （`api_check`），`k` 为字段名字节切片（intern 可分配/GC）；命中与否均把结果写入
/// `top` 并 `api_incr_top`。跨线程经 threadbarrier 同步。
pub fn lua_rawgetfield_bytes(l: &mut LuaState, idx: i32, k: &[u8]) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());

    let mut key = TValue::default();
    setsvalue!(l, &mut key, lua_s_new(l, k));
    setobj_2_s!(
      l,
      l.top,
      // B2-2a 任务B：getstr 折叠 Option<Slot> 后在边界还原哨兵裸形——setobj_2_s!
      // 宏按裸源槽读形复制（跨边界值搬运消费链），句柄化归 B2-2c 裁决
      lua_h_getstr(&*(*t).as_table_ptr(), key.as_string_ptr() as *mut _)
        .map_or(LUA_O_NILOBJECT, |s| s.as_const_ptr())
    );
    api_incr_top!(l);

    ttype!(l.top.sub(1)) as i32
  }
}
