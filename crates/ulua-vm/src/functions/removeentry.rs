use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  macros::{
    gkey::{gkey, gval},
    iscollectable::iscollectable,
    setttype::setttype,
    ttisnil::ttisnil,
  },
  type_aliases::lua_node::LuaNode,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn removeentry(n: *mut LuaNode) {
  unsafe {
    LUAU_ASSERT!(ttisnil!(gval!(n)));
    if iscollectable!(gkey!(n)) {
      setttype!(gkey!(n), LuaType::DeadKey as i32);
    }
  }
}
