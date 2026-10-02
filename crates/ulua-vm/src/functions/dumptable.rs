use core::{ffi::c_void, mem::size_of};

use crate::{
  functions::{c_file_write_bytes, dump_json_head, dumpref::dumpref, dumprefs::dumprefs},
  macros::{gcvalue::gcvalue, iscollectable::iscollectable, obj_2_gco::obj2gco},
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn dumptable(f: *mut c_void, h: &LuaTable) {
  unsafe {
    // 哨兵判据统一收口 is_hash_dummy（空哈希唯一判据）：工作量估算按 cpp 口径折 0，
    // 窗形遍历则恒读单格 dummy；实向量段直接以 node_window 共享窗为界。
    let nodes = h.node_window();
    let size = size_of::<LuaTable>()
      + if h.is_hash_dummy() {
        0
      } else {
        nodes.len() * size_of::<LuaNode>()
      }
      + h.array_window().len() * size_of::<TValue>();

    dump_json_head(f, "table", h.memcat, size as i32);

    if !h.is_hash_dummy() {
      c_file_write_bytes(f, b",\"pairs\":[");

      let mut first = true;

      // 哈希段遍历切共享窗（窗长 twoto(lsizenode) ⇔ 原手工 from_raw_parts 口径）
      for n in nodes {
        if !n.val.is_nil() && (iscollectable!(&n.key) || iscollectable!(&n.val)) {
          if !first {
            c_file_write_bytes(f, b",");
          }
          first = false;

          if iscollectable!(&n.key) {
            dumpref(f, gcvalue!(&n.key));
          } else {
            c_file_write_bytes(f, b"null");
          }

          c_file_write_bytes(f, b",");

          if iscollectable!(&n.val) {
            dumpref(f, gcvalue!(&n.val));
          } else {
            c_file_write_bytes(f, b"null");
          }
        }
      }

      c_file_write_bytes(f, b"]");
    }

    let arr = h.array_window();
    if !arr.is_empty() {
      c_file_write_bytes(f, b",\"array\":[");
      // dumprefs 消费 ptr+计数出参形（其内部自切窗），此处仅从窗取基址保持窗口径
      dumprefs(f, arr.as_ptr(), arr.len());
      c_file_write_bytes(f, b"]");
    }

    if !h.metatable.is_null() {
      c_file_write_bytes(f, b",\"metatable\":");
      dumpref(f, obj2gco!(h.metatable));
    }

    c_file_write_bytes(f, b"}");
  }
}
