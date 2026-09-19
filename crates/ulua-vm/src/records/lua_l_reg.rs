//! Source: `VM/include/lualib.h`

use core::ffi::c_char;

use crate::type_aliases::lua_c_function::LuaCFunction;
#[derive(Debug, Clone, Copy)]
pub struct LuaLReg {
  pub name: *const c_char,
  pub func: LuaCFunction,
}

/// 静态注册表包装：`LuaLReg` 含原始指针，`[LuaLReg; N]` 不自动 Sync，
/// 但条目全部指向 'static 名称与函数，只读共享安全。
#[repr(transparent)]
pub struct SyncLuaLReg<const N: usize>(pub [LuaLReg; N]);
// SAFETY：name 指向静态字符串字面量，func 为 'static 函数项，无内部可变性。
unsafe impl<const N: usize> Sync for SyncLuaLReg<N> {}
