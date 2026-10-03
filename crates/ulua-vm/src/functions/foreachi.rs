use crate::{
  enums::lua_type::LuaType, functions::lua_rawgeti::lua_rawgeti, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 类型校验/取长/压栈/`lua_rawgeti`/`call` 全经安全门面与被调引用形转调，体内已无真实裸操作，
/// 故本体降为安全 `fn`）：`l` 须处于可抛错的受保护帧——`check_type` 要求索引 1 表、2 函数否则抛错
/// 发散；`obj_len(1)` 取表长 n，循环对 1..=n 压函数/整数/`lua_rawgeti` 后 `call(2,1)`
/// （可再入 Lua、抛错、扩栈、触发 GC），每次迭代需 `top` 后 ≥3 空槽。
/// cpp VM/src/ltablib.cpp:16
pub fn foreachi(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  l.check_type(2, LuaType::Function);

  let n = l.obj_len(1) as i32;

  // i 是被迭代的数据（Lua 表整数下标），收为 1..=n 区间迭代
  for i in 1..=n {
    l.push_value(2); // function
    l.push_integer(i); // 1st argument
    lua_rawgeti(l, 1, i); // 2nd argument
    l.call(2, 1);

    if !l.is_nil(-1) {
      return 1;
    }
    l.pop(1); // remove nil result
  }

  0
}

lua_lib_fn!(pub fn foreachi @ref, foreachi_arm);
