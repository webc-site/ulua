use core::fmt::{Debug, Formatter, Result};

use crate::{
  records::{g_cheader::GCheader, gc_object::GcObject, lua_table::LuaTable, proto::Proto},
  type_aliases::{
    lua_c_function::LuaCFunction, lua_continuation::LuaContinuation, t_value::TValue,
  },
};
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CClosure {
  pub f: LuaCFunction,
  pub cont: LuaContinuation,
  /// 调试名（review.md §10 收形）：`'static` 字节串（不含终止 NUL），`None` = 无调试名
  /// （原 null 哨兵）。VM 只存引用不复制——与原 `*const c_char` 的「须随闭包存活」
  /// 契约同一，`'static` 即该契约的类型表达（全仓注入点均为静态字面量）。
  pub debugname: Option<&'static [u8]>,
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
