use core::{
  ffi::c_char,
  ptr::{eq, null},
  str::from_utf8,
};

use crate::{
  functions::{c_slice, cstr_bytes, enumedge::enum_edge, enumnode::enumnode},
  macros::{
    dummynode::dummynode, edge_metatable::EDGE_METATABLE, obj_2_gco::obj2gco, sizenode::sizenode,
    sizeudata::sizeudata, svalue::svalue,
  },
  records::{enum_context::EnumContext, lua_node::LuaNode as LuaNodeAlias, udata::Udata},
};

/// # Safety
/// `ctx` 须指向存活 EnumContext，`u` 须指向存活 Udata：`(*u).len` 给出 udata 尺寸；`metatable` 若非空须为合法表
/// 且非 dummynode，其 `node..node+sizenode` 桶数组可遍历，桶内 key/val 经 `is_string` 判定后才解引用为 C 串。
/// 只读遍历。cpp/VM/src/lgcdebug.cpp:886 enumudata。
pub(crate) unsafe fn enumudata(ctx: *mut EnumContext, u: *mut Udata) {
  unsafe {
    let mut name: *const c_char = null();

    let h = (*u).metatable;
    if !h.is_null() && !eq((*h).node, dummynode) {
      let n = (*h).node;
      let size = sizenode!(h) as usize;
      // Safety:node 为哈希桶数组，长度 1<<lsizenode，与表分配一致。
      for node in c_slice(n, size) {
        let node: &LuaNodeAlias = node;

        if node.key.is_string() && node.val.is_string() {
          let key_str = svalue!(&node.key);
          let val_str = svalue!(&node.val);

          let key_cmp = from_utf8(cstr_bytes(key_str)).unwrap_or("");
          if key_cmp == "__type" {
            name = val_str;
            break;
          }
        }
      }
    }

    let gco = obj2gco!(u);
    enumnode(ctx, gco, sizeudata((*u).len as usize), name);

    if !(*u).metatable.is_null() {
      let metatable_gco = obj2gco!((*u).metatable as *mut Udata);
      enum_edge(ctx, gco, metatable_gco, EDGE_METATABLE);
    }
  }
}
