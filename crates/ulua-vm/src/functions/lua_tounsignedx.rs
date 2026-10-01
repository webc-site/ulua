use crate::{
  functions::lua_tonumberx::lua_tonumberx, macros::luai_num_2_unsigned::luai_num2unsigned,
  records::lua_state::LuaState,
};

/// cpp `lua_tounsignedx`（`VM/src/lapi.cpp:453-471`）的内部精简版。
///
/// cpp 的 `int* isnum` out 参数改为 `Option<u32>`：`None` 即"非数字"
/// （cpp 置 *isnum = 0 并返回 0），唯一调用方 `luaL_checkunsigned` 只消费
/// 该标志。强转链整体复用 [`lua_tonumberx`] 的 [`ValueView`](crate::enums::value_view::ValueView)
/// match（cpp `tonumber(o,&n)` + `nvalue(o)`），成功路径再经 `luai_num2unsigned`
/// 折叠（对应 cpp 出参宏，`(unsigned)(long long)(n)`）。只读；`lua_tonumberx`
/// 内部经硬化的 `index_2_addr` 解析索引，越界返回 `None`（与 cpp 越界正索引
/// 行为一致），不抛错/不分配。
pub(crate) fn lua_tounsignedx(l: &LuaState, idx: i32) -> Option<u32> {
  lua_tonumberx(l, idx).map(luai_num2unsigned)
}
