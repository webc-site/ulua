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

pub(crate) unsafe fn removeentry(n: *mut LuaNode) {
  unsafe {
    LUAU_ASSERT!(ttisnil!(gval!(n)));
    if iscollectable!(gkey!(n)) {
      setttype!(gkey!(n), LuaType::DeadKey as i32);
    }
  }
}
