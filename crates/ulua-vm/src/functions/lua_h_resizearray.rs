use core::{
  ffi::c_void,
  ptr::{eq, null},
};

use crate::{
  functions::{adjustasize::adjustasize, resize::resize},
  macros::{dummynode::dummynode, sizenode::sizenode},
  records::{lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`（resize 分配失败可经其报错）；`t` 必须指向存活 `LuaTable`
/// （本调用会重排其 array/node，调用方缓存的旧数组/节点指针即失效）。
pub unsafe fn lua_h_resizearray(l: *mut LuaState, t: *mut LuaTable, nasize: i32) {
  // Safety: 契约保证 t 存活——读其 node/sizearray 判 dummynode 与求 nsize 合法
  unsafe {
    let nsize = if eq((*t).node as *const LuaNode, dummynode) {
      0
    } else {
      sizenode!(t)
    };

    let asize = adjustasize(t, nasize, null());

    resize(l, t, asize, nsize);
  }
}

/// # Safety
///
/// C ABI 导出壳：`l`/`t` 须满足 `lua_h_resizearray` 的入约，且 `t` 必须确为 `LuaTable`
/// 对象地址（由 *mut c_void 还原，类型错置即 UB）。
pub unsafe extern "C-unwind" fn lua_h_resizearray_export(
  l: *mut LuaState,
  t: *mut c_void,
  nasize: i32,
) {
  // Safety: 契约保证 t 实为 LuaTable 地址且 l/t 满足内层入约，指针还原合法
  unsafe {
    lua_h_resizearray(l, t as *mut LuaTable, nasize);
  }
}
