use core::{
  ffi::{CStr, c_char, c_int},
  mem::size_of,
  ptr::{self, addr_of, eq},
};

use crate::{
  enums::tms::TMS,
  functions::{c_slice, enumedge::enumedge, enumedges::enumedges, enumnode::enumnode},
  macros::{
    dummynode::dummynode, gcvalue::gcvalue, getstr::getstr, gfasttm::gfasttm, hvalue::hvalue,
    iscollectable::iscollectable, nvalue::nvalue, obj_2_gco::obj2gco, registry::registry,
    sizenode::sizenode, svalue::svalue, ttisnil::ttisnil, ttisnumber::ttisnumber,
    ttisstring::ttisstring,
  },
  records::{
    enum_context::EnumContext, lua_node::LuaNode, lua_t_value::TValue, lua_table::LuaTable,
    t_string::tstring,
  },
};

pub(crate) unsafe fn enumtable(ctx: *mut EnumContext, h: *mut LuaTable) {
  unsafe {
    let size = size_of::<LuaTable>()
      + if eq((*h).node, dummynode) {
        0
      } else {
        sizenode!(h) as usize * size_of::<LuaNode>()
      }
      + (*h).sizearray as usize * size_of::<TValue>();

    let obj = obj2gco!(h);

    let _h_ref = &*h;
    let registry_ptr = registry!((*ctx).l);
    let is_registry = eq(h, hvalue!(addr_of!(*registry_ptr) as *mut TValue));

    enumnode(
      ctx,
      obj,
      size,
      if is_registry {
        c"registry".as_ptr() as *const c_char
      } else {
        ptr::null()
      },
    );

    if !eq((*h).node, dummynode) {
      let mut weakkey = false;
      let mut weakvalue = false;

      let g = (*(*ctx).l).global;
      let metatable = (*h).metatable;
      if !metatable.is_null() {
        let mode = gfasttm(g, metatable, TMS::TmMode as i32);
        if !mode.is_null() && ttisstring!(mode) {
          let mode_str = svalue!(mode);
          let mode_slice = CStr::from_ptr(mode_str).to_bytes();
          weakkey = mode_slice.contains(&b'k');
          weakvalue = mode_slice.contains(&b'v');
        }
      }

      let node_count = sizenode!(h);
      for n in c_slice((*h).node, node_count as usize) {
        if !ttisnil!(&n.val) && (iscollectable!(&n.key) || iscollectable!(&n.val)) {
          if !weakkey && iscollectable!(&n.key) {
            enumedge(
              ctx,
              obj,
              gcvalue!(&n.key),
              c"[key]".as_ptr() as *const c_char,
            );
          }

          if !weakvalue && iscollectable!(&n.val) {
            if ttisstring!(&n.key) {
              enumedge(ctx, obj, gcvalue!(&n.val), svalue!(&n.key));
            } else if ttisnumber!(&n.key) {
              let mut buf = [0 as c_char; 32];
              let nvalue_ptr = nvalue!(&n.key);
              unsafe extern "C" {
                fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
              }
              snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                c"%.14g".as_ptr() as *const c_char,
                nvalue_ptr,
              );
              enumedge(ctx, obj, gcvalue!(&n.val), buf.as_ptr());
            } else {
              let mut buf = [0 as c_char; 32];
              let tt = n.key.tt();
              let global = (*(*ctx).l).global;
              let ttname_ptr = (*global).ttname.as_ptr().add(tt as usize);
              let name = getstr(ttname_ptr as *const tstring);
              unsafe extern "C" {
                fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
              }
              snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                c"[%s]".as_ptr() as *const c_char,
                name,
              );
              enumedge(ctx, obj, gcvalue!(&n.val), buf.as_ptr());
            }
          }
        }
      }
    }

    if (*h).sizearray > 0 {
      enumedges(
        ctx,
        obj,
        (*h).array,
        (*h).sizearray as usize,
        c"array".as_ptr() as *const c_char,
      );
    }

    if !(*h).metatable.is_null() {
      enumedge(
        ctx,
        obj,
        obj2gco!((*h).metatable),
        c"metatable".as_ptr() as *const c_char,
      );
    }
  }
}
