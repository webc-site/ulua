use core::{ptr::from_ref, slice::from_raw_parts};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::validateobjref::validateobjref,
  macros::{blackbit::BLACKBIT, checkliveness::checkliveness, upisopen::upisopen},
  records::{gc_object::GCObject, global_state::global_State, lua_state::LuaState, up_val::UpVal},
};

/// # Safety
/// `g` 须指向存活 global_State；`l` 须为存活线程 LuaState，其 `base_ci..=ci`、`stack..top`
/// 为同数组内合法区间，`openupval` 链各节点的 `u.open.next/prev` 自洽（校验仅读取，不改对象）。
pub(crate) unsafe fn validatestack(g: *mut global_State, l: &LuaState) {
  unsafe {
    // 引用源身份：只读校验，从共享借用降 `*const` 即够用
    let self_gco: *const GCObject = from_ref(l).cast();
    validateobjref(g, self_gco, l.gt as *const GCObject);

    // 调用帧窗口 base_ci..=ci 一次定界为切片迭代（等价原指针游走 `while ci <=
    // (*l).ci`；max(0) 仅作空窗口防御，与 dumpthread 的收口手法同款）
    let nci = l.ci.offset_from(l.base_ci).max(0) as usize + 1;
    for ci in from_raw_parts(l.base_ci, nci) {
      LUAU_ASSERT!(l.stack <= ci.base);
      LUAU_ASSERT!(ci.func <= ci.base && ci.base <= ci.top);
      LUAU_ASSERT!(ci.top <= l.stack_last);
    }

    // 栈窗口 [stack, top) 收为只读切片迭代；checkliveness 只读槽位 tag 与引用
    let stack_count = l.top.offset_from(l.stack).max(0) as usize;
    for slot in from_raw_parts(l.stack, stack_count) {
      checkliveness!(g, slot);
    }

    if !l.namecall.is_null() {
      validateobjref(g, self_gco, l.namecall as *const GCObject);
    }

    let mut uv: *mut UpVal = l.openupval;
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
