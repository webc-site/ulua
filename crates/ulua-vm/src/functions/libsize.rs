//! Source: `VM/src/laux.cpp:297-302` (hand-ported)

use crate::records::lua_l_reg::LuaLReg;

/// 计算注册表切片大小。
pub(crate) fn libsize(l: &[LuaLReg]) -> i32 {
  l.len() as i32
}
