//! `VM/src/lgc.cpp` 中 tableResizeProtected / stringResizeProtected 各自的
//! 受保护调用中转上下文，原本按机器生成的后缀拆成独立模块隔离同名类型；
//! 并入本模块后按用途命名，消费者以 `use ... as CallContext` 维持 C++ 局部名不变。
//! shrinkstack 的中转变体在 `functions::shrinkstackprotected` 内以局部 fn 实现。
use crate::records::lua_table::LuaTable;

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
