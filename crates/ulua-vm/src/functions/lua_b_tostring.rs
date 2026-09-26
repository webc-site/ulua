use crate::{
  functions::{lua_l_checkany::lua_l_checkany, lua_l_tolstring::lua_l_tolstring_ref},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkany(l,1)` 要求索引 1 有值否则抛错回退；
/// `luaL_tolstring(l,1,NULL)` 会读取该栈值、可能调用 __tostring 元方法（再入 Lua、可抛错/触发 GC）并把结果串
/// 压栈替换（允许 NULL 表示不回报长度），需 `(*l).top` 后 ≥1 空槽。
/// cpp VM/src/lbaselib.cpp:405
pub unsafe extern "C-unwind" fn lua_b_tostring(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);
    // 结果串压栈即目的（返回 1 即栈顶该串），切片引用不外传
    let _ = lua_l_tolstring_ref(l, 1);
    1
  }
}
