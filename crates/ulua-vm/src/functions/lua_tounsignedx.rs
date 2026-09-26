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
/// 折叠（对应 cpp 出参宏，`(unsigned)(long long)(n)`）。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引，使 `lua_tonumberx` 内的
/// `index_2_addr` 返回指向栈上有效 TValue 的指针。
pub(crate) unsafe fn lua_tounsignedx(l: *mut LuaState, idx: i32) -> Option<u32> {
  // Safety: 契约随 `lua_tonumberx` 的 `# Safety` 原样透传。
  unsafe { lua_tonumberx(l, idx).map(luai_num2unsigned) }
}
