use crate::records::lua_state::LuaState;

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_l_optboolean(l: *mut LuaState, narg: i32, def: bool) -> bool {
  // SAFETY: 契约保证 `l` 存活且 narg 槽可读；nil/false 由标签区分，其余情况返回 def
  unsafe {
    if (*l).is_none_or_nil(narg) {
      def
    } else {
      (*l).check_boolean(narg)
    }
  }
}
