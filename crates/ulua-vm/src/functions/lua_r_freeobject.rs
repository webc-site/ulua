use core::mem::size_of;

use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::lua_m_freearray::luaM_freearray,
  records::{
    gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState, luau_object::LuauObject,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// 仅供 sweep 阶段调用：`classinstance` 必须是构造完整的存活实例（`members` 按 `numberofmembers`
/// 分配、`memcat` 在类别界内），`page` 为其所属存活页、`l` 存活；违反即越界/非法释放。
/// cpp lclass.cpp:482 `luaR_freeobject`
pub(crate) unsafe fn lua_r_freeobject(
  l: *mut LuaState,
  classinstance: *mut LuauObject,
  page: *mut lua_Page,
) {
  // Safety: 契约保证 `l` 存活且 class 相关指针指向存活 LuauClass，成员偏移落在已分配数组界内，写引用处均按协议补 luaC_barriert 写屏障
  unsafe {
    let obj = &*classinstance;
    let memcat = obj.memcat;

    luaM_freearray!(l, obj.members, obj.numberofmembers, TValue, memcat);
    lua_m_freegco(
      l,
      classinstance as *mut GCObject,
      size_of::<LuauObject>(),
      memcat,
      page,
    );
  }
}
