//! `VM/src/lgc.cpp` 的三个互不相干局部 `struct CallContext`（shrinkstack /
//! tableResizeProtected / stringResizeProtected 各自的受保护调用中转上下文）
//! 原本按机器生成的后缀拆成独立模块隔离同名类型；并入本模块后按用途命名，
//! 消费者以 `use ... as CallContext` 维持 C++ 局部名不变。
use crate::records::lua_table::LuaTable;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CallContext {}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TableResizeCallContext {
  pub(crate) t: *mut LuaTable,
  pub(crate) nhsize: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct StringResizeCallContext {
  pub(crate) newsize: i32,
}
