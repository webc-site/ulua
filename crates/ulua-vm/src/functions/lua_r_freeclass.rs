use core::mem::size_of;

use crate::{
  functions::lua_m_freegco::lua_m_freegco,
  macros::lua_m_freearray::luaM_freearray,
  records::{
    gc_object::GCObject, lua_page::lua_Page, lua_state::LuaState, luau_class::LuauClass,
    t_string::tstring,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// 仅供 sweep 阶段调用：`classobject` 必须是构造完整的存活类（`staticmembers`/`offsettomember`
/// 分别按 numberofallmembers-numberofinstancemembers 与 numberofallmembers 分配，`memcat` 在类别界内），
/// `page` 为其所属的存活页、`l` 存活；违反即越界/非法释放。cpp lclass.cpp:467 `luaR_freeclass`
pub(crate) unsafe fn lua_r_freeclass(
  l: *mut LuaState,
  classobject: *mut LuauClass,
  page: *mut lua_Page,
) {
  // Safety: 契约保证 `l` 存活且 class 相关指针指向存活 LuauClass，成员偏移落在已分配数组界内，写引用处均按协议补 luaC_barriert 写屏障
  unsafe {
    let co = &*classobject;
    let numberof_all_members = co.numberofallmembers;
    let static_member_count = numberof_all_members - co.numberofinstancemembers;
    let memcat = co.memcat;

    luaM_freearray!(l, co.staticmembers, static_member_count, TValue, memcat);
    luaM_freearray!(
      l,
      co.offsettomember,
      numberof_all_members,
      *mut tstring,
      memcat
    );
    lua_m_freegco(
      l,
      classobject as *mut GCObject,
      size_of::<LuauClass>(),
      memcat,
      page,
    );
  }
}
