use core::{
  ffi::{c_char, c_void},
  ptr::null_mut,
};

use crate::{
  functions::{enumgco::enumgco, lua_m_visitgco::lua_m_visitgco},
  records::{enum_context::EnumContext, gc_object::GCObject},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_c_enumheap(
  l: *mut lua_State,
  context: *mut c_void,
  node: Option<unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, u8, u8, usize, *const c_char)>,
  edge: Option<unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, *mut c_void, *const c_char)>,
) {
  unsafe {
    let g = (*l).global;

    let mut ctx = EnumContext {
      l,
      context,
      node,
      edge,
    };

    // In Luau, lua_State is a collectible object. Its first field is hdr (GCheader),
    // which contains the tt, marked, and memcat fields required by GCObject.
    // The obj2gco macro expects a pointer to something that has these fields.
    // We cast the mainthread (lua_State*) to GCObject* to satisfy the macro and the enumgco signature.
    let mainthread_gco = (*g).mainthread as *mut GCObject;

    enumgco(
      &mut ctx as *mut EnumContext as *mut c_void,
      null_mut(),
      mainthread_gco,
    );

    lua_m_visitgco(
      l,
      &mut ctx as *mut EnumContext as *mut c_void,
      enumgco as *mut c_void,
    );
  }
}

pub use lua_c_enumheap as luaC_enumheap;
