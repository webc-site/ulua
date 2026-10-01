use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{lua_type::LuaType, value_view::ValueView},
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  records::{
    gc_object::GCObject, global_state::global_State, lua_node::LuaNode, lua_table::LuaTable,
  },
  type_aliases::t_value::TValue,
};

/// # Safety
/// `g` 须指向存活 global_State；`h` 须为存活 LuaTable，其 `array[0..sizearray]`、
/// `node[0..1<<lsizenode]` 引用区须完整可读（校验仅读取着色位与 tag）。
pub(crate) unsafe fn validatetable(g: *mut global_State, h: &LuaTable) {
  unsafe {
    let sizenode = 1 << h.lsizenode;

    LUAU_ASSERT!(h.union.lastfree <= sizenode);

    // 引用源身份：只读校验其着色位，故从共享借用降 `*const`（不得升 `*mut`）
    let h_gco: *const GCObject = (h as *const LuaTable).cast();

    if !h.metatable.is_null() {
      validateobjref(g, h_gco, h.metatable as *const GCObject);
    }

    // SAFETY:array 为 C 指针 + sizearray 计数，与表分配一致。
    for val in c_slice(h.array, h.sizearray as usize) {
      validateref(g, h_gco, val);
    }

    // SAFETY:node 数组与 lsizenode 分配一致；空表时 node 指向 dummynode
    // （合法静态对象），按 sizenode 读取与 C++ 遍历语义一致。校验只读，故用 `c_slice`。
    for (i, n) in c_slice(h.node.cast::<LuaNode>(), sizenode as usize)
      .iter()
      .enumerate()
    {
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
