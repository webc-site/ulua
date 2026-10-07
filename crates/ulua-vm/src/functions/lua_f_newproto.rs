use core::{mem::size_of, ptr};

use crate::{
  enums::lua_type::LuaType,
  functions::lua_m_newgco::lua_m_newgco,
  macros::lua_c_init::luaC_init,
  records::{lua_state::LuaState, proto::Proto},
};

/// # Safety
/// `l` 须为存活 LuaState 且 `(*l).activememcat`/全局分配器有效，处于可分配/GC 的受保护帧（`lua_m_newgco` 可能触发 GC 或 OOM）；
/// 返回的新 Proto 已被 `luaC_init` 挂入 GC 且各数组指针/size 字段自洽归零，仅在 `l` 的 GC 接住前对其独占写入。
/// cpp/VM/src/lfunc.cpp:12 luaF_newproto。
pub(crate) unsafe fn lua_f_newproto(l: *mut LuaState) -> *mut Proto {
  unsafe {
    let proto = lua_m_newgco(l, size_of::<Proto>(), (*l).activememcat) as *mut Proto;

    // 类型化清零先于头初始化：Proto 各字段（指针/标量）的 `Default` 即合法空值，
    // 一次 `ptr::write` 覆盖原先 34 个逐字段 null_mut()/0 写；`luaC_init!` 随后
    // 只覆写头部 tt/marked/memcat 三字段，最终态与逐字段写完全一致
    ptr::write(proto, Proto::default());
    luaC_init!(l, proto, LuaType::Proto as i32);

    proto
  }
}
