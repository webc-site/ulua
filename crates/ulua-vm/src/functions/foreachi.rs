use crate::{
  enums::lua_type::LuaType, functions::lua_rawgeti::lua_rawgeti, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checktype(l,1,TABLE)`、`(l,2,FUNCTION)` 要求索引 1 表、2 函数
/// 否则抛错回退；`lua_objlen(l,1)` 取表长度 n，循环对 1..=n 压函数/整数/`lua_rawgeti` 后 `lua_call(...,2,1)`
/// （可再入 Lua、抛错、扩栈、触发 GC），每次迭代需 ≥3 栈槽；`lua_isnil`/`lua_pop` 读写 `(*l).top`。
/// cpp VM/src/ltablib.cpp:16
pub unsafe fn foreachi(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    (*l).check_type(2, LuaType::Function);

    let n = (*l).obj_len(1) as i32;

    // i 是被迭代的数据（Lua 表整数下标），收为 1..=n 区间迭代
    for i in 1..=n {
      (*l).push_value(2); // function
      (*l).push_integer(i); // 1st argument
      lua_rawgeti(&mut *l, 1, i); // 2nd argument
      (*l).call(2, 1);

      if !(*l).is_nil(-1) {
        return 1;
      }
      (*l).pop(1); // remove nil result
    }

    0
  }
}

lua_lib_fn!(pub fn foreachi, foreachi_arm);
