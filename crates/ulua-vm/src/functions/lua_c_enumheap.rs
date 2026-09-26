//! Source: `VM/src/lgcdebug.cpp:1128` (hand-ported)

use core::{
  ffi::{c_char, c_void},
  ptr::{from_mut, null_mut},
};

use crate::{
  functions::{enumgco::enumgco, lua_m_visitgco::lua_m_visitgco},
  records::{enum_context::EnumContext, gc_object::GCObject, lua_state::LuaState},
};

/// # Safety
/// `l` 须指向存活 `LuaState` 且停顿在安全点（cpp lgcdebug.cpp:1128）：枚举期间 heap 页与
/// 对象保持存活；`context` 须与 `node`/`edge` 回调约定的宿主指针类型匹配，回调须为合法
/// `unsafe extern "C-unwind"` 函数。
pub unsafe fn lua_c_enumheap(
  l: *mut LuaState,
  context: *mut c_void,
  node: Option<unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, u8, u8, usize, *const c_char)>,
  edge: Option<unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, *mut c_void, *const c_char)>,
) {
  // Safety: 契约保证 `l` 存活且回调 context 与其类型匹配，逐页遍历时对象在枚举期间保持存活
  unsafe {
    let g = (*l).global;

    let mut ctx = EnumContext {
      l,
      context,
      node,
      edge,
    };

    // cpp lgc.cpp luaC_enumheap：mainthread 的 obj2gco 在移植记录上无 GCObject
    // 头字段访问，统一提升到 GCObject* 以匹配 enumgco 签名（与 obj2gco 等价）
    let mainthread_gco = (*g).mainthread as *mut GCObject;

    enumgco(
      from_mut(&mut ctx).cast::<c_void>(),
      null_mut(),
      mainthread_gco,
    );

    lua_m_visitgco(l, from_mut(&mut ctx).cast::<c_void>(), enumgco);
  }
}
