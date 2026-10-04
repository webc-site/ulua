use crate::{
  functions::lua_rawseti::lua_rawseti, macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；收形后取长/
/// 校验/建表/填槽全经 `check_integer`/`create_table`/`push_value`/`lua_rawseti` 安全门面，不再直写
/// 新建表的 `array` 裸槽，体内已无裸操作，故本体降为安全 `fn`）：`l` 须处于可抛错、可分配/GC 的
/// 受保护 C 帧——1 号位为整数 `size`（负值经 `arg_error` 抛错发散），2 号位为可选填充值，
/// `create_table(size, 0)` 把新建表压上栈顶（fill 分支可达时 1..2 号位已被实参占据，
/// 故该槽恒为绝对槽 3，以 `get_top` 现读为准）；其后每轮 `push_value` 的顶槽即
/// `lua_rawseti` 待弹 value（帧余量 ≥ LUA_MINSTACK，与 C 帧约定同形）。填值与 cpp `setobj2t`
/// 直写数组段逐位同值，仅多出保守写屏障（不成观测差）。cpp/VM/src/ltablib.cpp:571 tcreate。
pub fn tcreate(l: &mut LuaState) -> i32 {
  let size = l.check_integer(1);
  if size < 0 {
    l.arg_error(1, "size out of range");
  }

  // cpp 同序：先判 2 号位（此刻它仍在实参区/界外为 None——若先建表，压表会把
  // 实参值抬进 2 号槽观测面，`table.create(n)` 无填充也将误走 fill）
  let fill = !l.is_none_or_nil(2);

  l.create_table(size, 0);

  // 2 号位为填充值时把整段数组铺成该值。
  if fill {
    let t = l.get_top(); // 新建表所在绝对槽（= 3：1=size、2=填充值已被占据）
    for i in 1..=size {
      l.push_value(2);
      lua_rawseti(l, t, i);
    }
  }

  1
}

lua_lib_fn!(pub fn tcreate @ref, tcreate_arm);
