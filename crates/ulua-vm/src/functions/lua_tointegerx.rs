use crate::{functions::lua_tonumberx::lua_tonumberx, records::lua_state::LuaState};

/// cpp `lua_tointegerx`（`VM/src/lapi.cpp:432`）：`idx` 槽位可作数值时返回其
/// 截断为 `i32` 的整数（cpp `luai_num2int` 即 `(int)(n)`，Rust `as` 为饱和转换，
/// 与本仓库既有 VM 行为一致）。
///
/// `int* isnum` 出参收口为 `Option<i32>`：`None` 即 cpp 的 `*isnum = 0` 失败
/// 路径（其返回值 0 只是占位）。读取强转整体复用 [`lua_tonumberx`] 的
/// [`ValueView`](crate::enums::value_view::ValueView) match 链，对应 cpp
/// `tonumber(o,&n)` + `nvalue(o)` 两步。
///
/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引，使 `lua_tonumberx` 内的
/// `index2addr` 返回指向栈上有效 TValue 的指针。cpp `lapi.cpp:432`。
pub unsafe fn lua_tointegerx(l: *mut LuaState, idx: i32) -> Option<i32> {
  // Safety: 契约随 `lua_tonumberx` 的 `# Safety` 原样透传。
  unsafe { lua_tonumberx(l, idx).map(|n| n as i32) }
}
