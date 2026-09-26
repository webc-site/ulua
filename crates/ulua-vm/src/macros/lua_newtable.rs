use crate::{functions::lua_createtable::lua_createtable, records::lua_state::LuaState};

#[inline]
/// # Safety
///
/// `l` 必须指向存活的 `LuaState` 且栈顶已预留 1 槽承接新表。
pub unsafe fn lua_newtable(l: *mut LuaState) {
  // Safety: 契约保证 `l` 存活，createtable(0,0) 仅建表压栈
  unsafe {
    lua_createtable(l, 0, 0);
  }
}
