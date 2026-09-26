use core::{
  ffi::c_char,
  mem::size_of,
  ptr::{self, eq},
};

use crate::{
  enums::{t_key_view::TKeyView, tms::TMS, value_view::ValueView},
  functions::{
    c_slice, cstr_bytes,
    enumedge::{edge_name, enum_edge, enumedge},
    enumedges::enumedges,
    enumnode::enumnode,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{
    dummynode::dummynode, edge_metatable::EDGE_METATABLE, gcvalue::gcvalue, getstr::getstr,
    gfasttm::gfasttm, iscollectable::iscollectable, obj_2_gco::obj2gco, registry::registry,
    sizenode::sizenode,
  },
  records::{
    enum_context::EnumContext, lua_node::LuaNode, lua_t_value::TValue, lua_table::LuaTable,
    t_string::tstring,
  },
};
/// NUL 结尾字节串（边名指针由 `edge_name`/`enum_edge` 取；§10 不引入 `CStr`/`c"…"`）。
const NODE_REGISTRY: &[u8] = b"registry\0";
const EDGE_KEY: &[u8] = b"[key]\0";
const EDGE_ARRAY: &[u8] = b"array\0";

/// # Safety
/// `ctx` 须为存活 `EnumContext` 且 `(*ctx).l` 为存活 `LuaState`（经其取 `registry`/`global` 读 fasttm）；
/// `h` 须为存活 `LuaTable`：`(*h).node` 为 dummynode 或覆盖 `sizenode(h)` 个 `LuaNode`，`(*h).array` 覆盖
/// `(*h).sizearray` 个 `TValue`，`(*h).metatable` 允许 NULL；遍历 node/array 时对 collectable 键值 `gcvalue!`
/// 递归 `enumedge`。只读枚举，不改对象、不回收。
/// cpp VM/src/lgcdebug.cpp:784
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

    let registry_ptr = registry!((*ctx).l);
    let is_registry = eq(h, (*registry_ptr).as_table_ptr());

    enumnode(
      ctx,
      obj,
      size,
      if is_registry {
        edge_name(NODE_REGISTRY)
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
        let mode = gfasttm(g, metatable, TMS::TmMode);
        // `ttisstring! + svalue!` 链收敛为 ValueView::String 臂：tag 判定与串数据
        // 指针提取同臂完成（svalue! == getstr(tsvalue!)，payload 同形）
        if !mode.is_null()
          && let ValueView::String(ts) = ValueView::from_tvalue(&*mode)
        {
          let mode_slice = cstr_bytes(getstr(ts as *const _));
          weakkey = mode_slice.contains(&b'k');
          weakvalue = mode_slice.contains(&b'v');
        }
      }

      let node_count = sizenode!(h);
      for n in c_slice((*h).node, node_count as usize) {
        if !matches!(ValueView::from_tvalue(&n.val), ValueView::Nil)
          && (iscollectable!(&n.key) || iscollectable!(&n.val))
        {
          if !weakkey && iscollectable!(&n.key) {
            enum_edge(ctx, obj, gcvalue!(&n.key), EDGE_KEY);
          }

          if !weakvalue && iscollectable!(&n.val) {
            // 键轴 tag/payload 读链收敛为 TKeyView 变体 match：串键取数据区、数字键
            // 走 %.14g；其余 tag 走 ttname 表查名（tt 为字段访问，非读宏）
            match TKeyView::from_tkey(&n.key) {
              TKeyView::String(ts) => {
                enumedge(ctx, obj, gcvalue!(&n.val), getstr(ts as *const _));
              }
              TKeyView::Number(num) => {
                let mut buf = [0 as c_char; 32];
                // %.14g → 14 位有效数字，core::fmt 精度参数等效
                fmt_cstr_buf(&mut buf, format_args!("{:.14}", num));
                enumedge(ctx, obj, gcvalue!(&n.val), buf.as_ptr());
              }
              _ => {
                let mut buf = [0 as c_char; 32];
                let tt = n.key.tt();
                let global = (*(*ctx).l).global;
                let ttname_ptr = (*global).ttname.as_ptr().add(tt as usize);
                let name = cstr_display(getstr(ttname_ptr as *const tstring));
                fmt_cstr_buf(&mut buf, format_args!("[{name}]"));
                enumedge(ctx, obj, gcvalue!(&n.val), buf.as_ptr());
              }
            }
          }
        }
      }
    }

    if (*h).sizearray > 0 {
      enumedges(ctx, obj, (*h).array, (*h).sizearray as usize, EDGE_ARRAY);
    }

    if !(*h).metatable.is_null() {
      enum_edge(ctx, obj, obj2gco!((*h).metatable), EDGE_METATABLE);
    }
  }
}
