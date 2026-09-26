use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_call::lua_call, lua_l_checktype::lua_l_checktype, lua_objlen::lua_objlen,
    lua_pushinteger::lua_pushinteger, lua_pushvalue::lua_pushvalue, lua_rawgeti::lua_rawgeti,
  },
  macros::{lua_isnil::lua_isnil, lua_pop::lua_pop},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checktype(l,1,TABLE)`、`(l,2,FUNCTION)` 要求索引 1 表、2 函数
/// 否则抛错回退；`lua_objlen(l,1)` 取表长度 n，循环对 1..=n 压函数/整数/`lua_rawgeti` 后 `lua_call(...,2,1)`
/// （可再入 Lua、抛错、扩栈、触发 GC），每次迭代需 ≥3 栈槽；`lua_isnil`/`lua_pop` 读写 `(*l).top`。
/// cpp VM/src/ltablib.cpp:16
pub unsafe extern "C-unwind" fn foreachi(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_l_checktype(l, 2, LuaType::Function as i32);

    let mut i: i32 = 1;
    let n = lua_objlen(l, 1);

    while i <= n {
      lua_pushvalue(l, 2); // function
      lua_pushinteger(l, i); // 1st argument
      lua_rawgeti(l, 1, i); // 2nd argument
      lua_call(l, 2, 1);

      if !lua_isnil!(l, -1) {
        return 1;
      }
      lua_pop(l, 1); // remove nil result

      i += 1;
    }

    0
  }
}
