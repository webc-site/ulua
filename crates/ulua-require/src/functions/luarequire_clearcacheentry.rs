use ulua_vm::records::lua_state::LuaState;

use crate::functions::{
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY, push_str::c_str_prefix_owned,
  registry_table::set_registry_mark,
};

/// 对应 cpp `luarequire_clearcacheentry`（Require.cpp 壳 + RequireImpl.cpp 实体
/// 两段）：Rust 侧无 TU 分离需求，壳与实体合一（review.md §3「只包一层的壳
/// 合并」）。清条目即 `_MODULES[cacheKey] = nil`，收口 registry_table mark 门面
/// （false → pushnil + 写字段 + 弹表，与原内联四步同净效果）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`（VM 以 Lua/C API 调本 C 函数时传入），栈顶参数
/// 布局由 C 调用约定保证。
pub unsafe extern "C-unwind" fn luarequire_clearcacheentry(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 l 为存活独占 state，重建借用无别名冲突。
  let l = unsafe { &mut *l };
  // cacheKey 拷贝为本地字节串（cpp 直接以 VM 串指针查表；此处门面内部压栈可能
  // 触发 GC，先取本地副本避免与栈借用互踩）。
  let cache_key = c_str_prefix_owned(l.check_bytes(1));
  set_registry_mark(l, REQUIRED_CACHE_TABLE_KEY, &cache_key, false);
  0
}
