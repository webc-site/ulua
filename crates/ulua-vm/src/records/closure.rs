use core::fmt::{Debug, Formatter, Result};

use crate::{
  records::{
    g_cheader::GCheader, gc_object::GcObject, lua_table::LuaTable, proto::Proto, t_string::tstring,
  },
  type_aliases::{
    lua_c_function::LuaCFunction, lua_continuation::LuaContinuation, t_value::TValue,
  },
};
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CClosure {
  pub f: LuaCFunction,
  pub cont: LuaContinuation,
  /// 调试名锚定串表内的 TString（cpp lobject.h:477 `TString* debugname`）：VM 经
  /// `luaS_new` intern 复制持有，非调用方指针；GC 经 traverseclosure 标记。
  pub debugname: *mut tstring,
  pub upvals: [TValue; 1],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LClosure {
  pub p: *mut Proto,
  pub uprefs: [TValue; 1],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union ClosureInner {
  pub c: CClosure,
  pub l: LClosure,
}

impl Debug for ClosureInner {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("ClosureInner").finish_non_exhaustive()
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct Closure {
  pub hdr: GCheader,
  pub is_c: u8,
  pub nupvalues: u8,
  pub stacksize: u8,
  pub preload: u8,
  pub gclist: *mut GcObject,
  pub env: *mut LuaTable,
  pub inner: ClosureInner,
}
