use core::slice::from_raw_parts_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::validateobjref::validateobjref,
  macros::{blackbit::BLACKBIT, checkliveness::checkliveness, upisopen::upisopen},
  records::{global_state::global_State, lua_state::LuaState, up_val::UpVal},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn validatestack(g: *mut global_State, l: *mut LuaState) {
  unsafe {
    validateobjref(g, l as *mut _, (*l).gt as *mut _);

    // 调用帧窗口 base_ci..=ci 一次定界为切片迭代（等价原指针游走 `while ci <=
    // (*l).ci`；max(0) 仅作空窗口防御，与 dumpthread 的收口手法同款）
    let nci = (*l).ci.offset_from((*l).base_ci).max(0) as usize + 1;
    for ci in from_raw_parts_mut((*l).base_ci, nci) {
      LUAU_ASSERT!((*l).stack <= ci.base);
      LUAU_ASSERT!(ci.func <= ci.base && ci.base <= ci.top);
      LUAU_ASSERT!(ci.top <= (*l).stack_last);
    }

    // 栈窗口 [stack, top) 收为切片迭代；checkliveness 以槽位指针为数据，从借用取回
    let stack_count = (*l).top.offset_from((*l).stack).max(0) as usize;
    for slot in from_raw_parts_mut((*l).stack, stack_count) {
      checkliveness!(g, slot as *mut TValue);
    }

    if !(*l).namecall.is_null() {
      validateobjref(g, l as *mut _, (*l).namecall as *mut _);
    }

    let mut uv: *mut UpVal = (*l).openupval;
    while !uv.is_null() {
      LUAU_ASSERT!((*uv).hdr.tt == LuaType::Upval as u8);
      LUAU_ASSERT!(upisopen!(uv));
      LUAU_ASSERT!(
        (*(*uv).u.open.next).u.open.prev == uv && (*(*uv).u.open.prev).u.open.next == uv
      );
      // open upvalue 永远不是黑色（BLACKBIT）
      LUAU_ASSERT!(((*uv).hdr.marked & (1u8 << BLACKBIT)) == 0);
      uv = (*uv).u.open.threadnext;
    }
  }
}
