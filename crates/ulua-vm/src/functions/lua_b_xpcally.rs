use crate::{
  enums::lua_type::LuaType,
  functions::lua_pcallyieldable::lua_pcallyieldable,
  macros::{lua_lib_fn::lua_lib_fn, lua_multret::LUA_MULTRET},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活与独占已由 `&mut LuaState` 承载（r16-v29 收形）；`lua_pcallyieldable` 仍收裸形，
/// 转手经一次 `l.as_mut_ptr()` 就地重建（借用窗止于当句），屏障按 r16-v21 判例保留；`l` 仍须处于
/// 受保护帧：`check_type(l,2,FUNCTION)` 要求索引 2 为错误处理函数否则抛错回退，
/// 索引 1 为被调函数、其后为实参；`push_value`/`replace` 交换 1、2 槽（各槽须存活）；随后
/// `lua_pcall`（errfunc=1、LUA_MULTRET 变长返回）可再入 Lua、抛错、扩栈与触发 GC，调用侧须承接展开。
/// cpp VM/src/lbaselib.cpp:344
pub(crate) unsafe fn lua_b_xpcally(l: &mut LuaState) -> i32 {
  unsafe {
    l.check_type(2, LuaType::Function);

    // swap function & error function
    l.push_value(1);
    l.push_value(2);
    l.replace(1);
    l.replace(2);
    // at this point the stack looks like err, f, args

    lua_pcallyieldable(l.as_mut_ptr(), l.get_top() - 2, LUA_MULTRET, 1)
  }
}

lua_lib_fn!(pub(crate) fn lua_b_xpcally @ref, lua_b_xpcally_arm);
