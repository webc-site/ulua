use core::{
  ffi::{CStr, c_char},
  ptr::{eq, null},
};

use crate::{
  functions::{c_slice, enumnode::enumnode},
  macros::{
    dummynode::dummynode, obj_2_gco::obj2gco, sizenode::sizenode, sizeudata::sizeudata,
    svalue::svalue, ttisstring::ttisstring,
  },
  records::{enum_context::EnumContext, udata::Udata},
  type_aliases::lua_node::LuaNode as LuaNodeAlias,
};

pub(crate) unsafe fn enumudata(ctx: *mut EnumContext, u: *mut Udata) {
  unsafe {
    let mut name: *const c_char = null();

    let h = (*u).metatable;
    if !h.is_null() && !eq((*h).node, dummynode) {
      let n = (*h).node;
      let size = sizenode!(h) as usize;
      // SAFETY：node 为哈希桶数组，长度 1<<lsizenode，与表分配一致。
      for node in c_slice(n, size) {
        let node: &LuaNodeAlias = node;

        if ttisstring!(&node.key) && ttisstring!(&node.val) {
          let key_str = svalue!(&node.key);
          let val_str = svalue!(&node.val);

          let key_cmp = CStr::from_ptr(key_str).to_str().unwrap_or("");
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
      enumedge(ctx, gco, metatable_gco, c"metatable".as_ptr());
    }
  }
}

use crate::functions::enumedge::enumedge;
