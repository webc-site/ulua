use core::slice::from_raw_parts;

use ulua_vm::{
  functions::{
    lua_gettop::lua_gettop, lua_insert::lua_insert, lua_l_argerror_l::lua_l_argerror_l,
    lua_l_checklstring::lua_l_checklstring, lua_replace::lua_replace, lua_settable::lua_settable,
  },
  macros::{lua_l_error::luaL_error, lua_pop::lua_pop},
  records::lua_state::LuaState,
};

use crate::functions::{
  c_str_prefix::push_lowered_c_str, cache_table_keys::REGISTERED_CACHE_TABLE_KEY,
  path_bytes::ALIAS_PREFIX, registry_table::push_registry_table,
};

/// 对应 cpp `luarequire_registermodule`（Require.cpp 壳 + RequireImpl.cpp 实体
/// 两段）：Rust 侧无 TU 分离需求，壳与实体合一（review.md §3「只包一层的壳
/// 合并」）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈上需有注册模块所需的两个参数。
pub unsafe extern "C-unwind" fn luarequire_registermodule(l: *mut LuaState) -> i32 {
  // Safety: l 是 VM 调本 C 函数时传入的存活 state，参数个数先校验为 2；
  // lua_l_checklstring 非字符串即内部 tag_error 报错发散，返回指针必非空且长度与
  // len 匹配，from_raw_parts 构造的切片仅在该串本次调用期存活（入口一次转切片，
  // 内部链不再出现裸指针）。
  let path = unsafe {
    if lua_gettop(l) != 2 {
      luaL_error!(
        l,
        "expected 2 arguments: aliased require path and desired result"
      );
    }
    let mut len = 0usize;
    from_raw_parts(lua_l_checklstring(l, 1, &mut len).cast::<u8>(), len)
  };

  // 对应 cpp `path.length() == 0 || path[0] != '@'`：空串同样不满足首字节判定。
  // Safety: 非法路径经 lua_l_argerror_l 报错（与 cpp 同样由 VM 侧发散终止），
  // 返回码与原实现一样不消费。
  if path.first() != Some(&ALIAS_PREFIX) {
    unsafe { lua_l_argerror_l(l, 1, "path must begin with '@'") };
  }

  // Safety: push_lowered_c_str/push_registry_table 按各自契约即时消费本地指针，
  // cpp `lua_pushstring(pathLower.c_str())` 按首个 NUL 截断 + ASCII 小写归一，
  // 与 check_registered_modules 查表侧同一形态；replace/insert/settable/pop 为
  // 纯栈操作，与原内联块同序配平。
  unsafe {
    push_lowered_c_str(l, path);
    lua_replace(l, 1);

    push_registry_table(l, REGISTERED_CACHE_TABLE_KEY);
    lua_insert(l, 1);
    lua_settable(l, 1);
    lua_pop(l, 1);
  }

  0
}
