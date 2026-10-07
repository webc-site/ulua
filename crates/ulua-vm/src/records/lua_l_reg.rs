//! Source: `VM/include/lualib.h`

use crate::{records::lua_state::LuaState, type_aliases::lua_c_function::LuaCFunction};

#[derive(Debug, Clone, Copy)]
pub struct LuaLReg {
  /// lua 名字：静态字节切片（不含尾部 `\0`），静态存活、`Sync`，
  /// 注册表可直接用作 `static`。
  pub name: &'static [u8],
  pub func: LuaCFunction,
}

impl LuaLReg {
  /// 具名函数条目构造（cpp `{name, fn}` 聚合初始化的直接对应）：全部
  /// `luaopen_*` 库注册表共用，取代每项 4 行的结构体字面量样板。
  pub const fn new(
    name: &'static [u8],
    func: unsafe extern "C-unwind" fn(l: *mut LuaState) -> i32,
  ) -> Self {
    Self {
      name,
      func: Some(func),
    }
  }
}
