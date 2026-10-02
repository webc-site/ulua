use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{lua_type::LuaType, value_view::ValueView},
  functions::{validateobjref::validateobjref, validateref::validateref},
  records::{
    gc_object::GCObject, global_state::global_State, lua_table::LuaTable,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// `g` 须指向存活 global_State；`h` 须为存活 LuaTable，其数组窗 `array_window()`、
/// 哈希窗 `node_window()` 引用区须完整可读（校验仅读取着色位与 tag）。
pub(crate) unsafe fn validatetable(g: *mut global_State, h: &LuaTable) {
  unsafe {
    // 哈希段切 node_window 共享窗：哨兵表窗长恒 1（读单格 dummy，与 C++ 按
    // sizenode=1 走查一致），实向量窗长 twoto(lsizenode) ⇔ 原 `1 << lsizenode`。
    let nodes = h.node_window();
    let sizenode = nodes.len() as i32;

    LUAU_ASSERT!(h.union.lastfree <= sizenode);

    // 引用源身份：只读校验其着色位，故从共享借用降 `*const`（不得升 `*mut`）
    let h_gco: *const GCObject = (h as *const LuaTable).cast();

    if !h.metatable.is_null() {
      validateobjref(g, h_gco, h.metatable as *const GCObject);
    }

    // 数组段切 array_window 共享窗（窗长 max(sizearray,0)，与原 c_slice 守卫同形）
    for val in h.array_window() {
      validateref(g, h_gco, val);
    }

    // 校验只读，窗内逐格判据与原 (i, n) 走查序逐位一致
    for (i, n) in nodes.iter().enumerate() {
      LUAU_ASSERT!(
        n.key.tt() != LuaType::DeadKey as i32
          || matches!(ValueView::from_tvalue(&n.val), ValueView::Nil)
      );

      let next_val = n.key.next();
      let i = i as i32;
      LUAU_ASSERT!(i + next_val >= 0 && i + next_val < sizenode);

      if !matches!(ValueView::from_tvalue(&n.val), ValueView::Nil) {
        let k = TValue {
          tt: n.key.tt(),
          value: n.key.value,
          ..Default::default()
        };

        validateref(g, h_gco, &k);
        validateref(g, h_gco, &n.val);
      }
    }
  }
}
