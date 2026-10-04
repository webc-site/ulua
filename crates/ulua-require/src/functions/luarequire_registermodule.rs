use ulua_vm::{
  functions::lua_l_argerror_l::lua_l_argerror_l, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

use crate::functions::{
  cache_table_keys::REGISTERED_CACHE_TABLE_KEY,
  path_bytes::ALIAS_PREFIX,
  push_str::{c_str_prefix_owned, push_lowered_c_str},
  registry_table::push_registry_table,
};

/// 注册模块所需的确切栈参数个数（cpp `lua_gettop(L) != 2`）。
const REGISTER_MODULE_ARGS: i32 = 2;

/// 对应 cpp `luarequire_registermodule`（Require.cpp 壳 + RequireImpl.cpp 实体
/// 两段）：Rust 侧无 TU 分离需求，壳与实体合一（review.md §3「只包一层的壳
/// 合并」）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`（VM 以 Lua/C API 调本 C 函数时传入），栈上需有
/// 注册模块所需的两个参数。
pub unsafe extern "C-unwind" fn luarequire_registermodule(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 l 为存活独占 state，重建借用无别名冲突；入口一次把路径
  // 拷为本地字节串（cpp std::string(path, len) 拷贝），此后内部链不再借出 VM 串。
  let l = unsafe { &mut *l };

  if l.get_top() != REGISTER_MODULE_ARGS {
    luaL_error!(
      l,
      "expected 2 arguments: aliased require path and desired result"
    );
  }

  let path = c_str_prefix_owned(l.check_bytes(1));

  // 对应 cpp `path.length() == 0 || path[0] != '@'`：空串同样不满足首字节判定。
  if path.first() != Some(&ALIAS_PREFIX) {
    // Safety: l 存活，lua_l_argerror_l 报错发散（与 cpp 同样由 VM 侧终止）。
    // vm 侧 `lua_l_argerror_l` 仍收 `*mut LuaState`（禁区门面，收编 `&mut` 形
    // 已列越界待办）；`l.as_mut_ptr()` 由上方独占借用借出、窗止于当句。
    unsafe { lua_l_argerror_l(l.as_mut_ptr(), 1, "path must begin with '@'") };
  }

  // 键 ASCII 小写归一后压栈，替换栈 1 路径（cpp pathLower + lua_replace），再以
  // 归一键写入 `_REGISTEREDMODULES` 缓存表，收尾弹掉缓存表（纯栈操作，配平）。
  push_lowered_c_str(l, &path);
  l.replace(1);

  push_registry_table(l, REGISTERED_CACHE_TABLE_KEY);
  l.insert(1);
  l.set_table(1);
  l.pop(1);

  0
}
