use crate::{
  functions::lua_rawseti::lua_rawseti, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；收形后建表/
/// 拷元/落 `n` 字段全经 `create_table`/`push_value`/`lua_rawseti`/`set_field_bytes` 安全门面，
/// 不再直写新建表的 `array` 裸槽，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错、
/// 可分配/GC 的受保护 C 帧——1..=n 号位为待打包实参（n = `get_top`），`create_table` 后新建表落在
/// 绝对槽 `n + 1`，其后每轮 `push_value` 的顶槽即 `lua_rawseti` 待弹 value（帧余量 ≥ LUA_MINSTACK，
/// 与 C 帧约定同形）。写入值与 cpp `setobj2t` 直写数组段逐位同值，仅多出保守写屏障（不成观测差）。
/// cpp/VM/src/ltablib.cpp:345 tpack。
pub fn tpack(l: &mut LuaState) -> i32 {
  let n = l.get_top(); // number of elements to pack
  l.create_table(n, 1); // create result table

  let t = n + 1; // 新建表所在绝对槽
  for i in 1..=n {
    l.push_value(i);
    lua_rawseti(l, t, i);
  }

  l.push_integer(n); // t.n = number of elements
  l.set_field_bytes(t, b"n");

  1 // return table
}

lua_lib_fn!(pub fn tpack @ref, tpack_arm);
