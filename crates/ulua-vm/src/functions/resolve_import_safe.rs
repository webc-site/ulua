use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_pcall::lua_d_pcall, lua_gettop::lua_gettop},
  macros::{savestack::savestack, setnilvalue::setnilvalue},
  records::{lua_state::LuaState, lua_table::LuaTable, resolve_import::ResolveImport},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn resolve_import_safe(
  l: *mut LuaState,
  _env: *mut LuaTable,
  k: *mut TValue,
  id: u32,
) {
  // Safety: 契约保证 `l` 为存活调用帧且对应 pc 的导入空间已建立，解析出的模块表压入栈顶且表可读
  unsafe {
    let mut ri = ResolveImport { k, id };

    if (*(*l).gt).safeenv != 0 {
      let old_top = lua_gettop(l);
      let status = lua_d_pcall(
        l,
        Some(ResolveImport::run),
        &mut ri as *mut _ as *mut c_void,
        savestack!(l, (*l).top) as isize,
        0,
      );

      LUAU_ASSERT!(old_top + 1 == lua_gettop(l));

      if status != LuaStatus::Ok as i32 {
        setnilvalue!((*l).top.sub(1));
      }
    } else {
      setnilvalue!((*l).top);
      (*l).top = (*l).top.add(1);
    }
  }
}
