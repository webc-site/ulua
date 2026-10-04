use crate::{
  enums::lua_type::LuaType,
  functions::lua_getreadonly::lua_getreadonly,
  macros::{lua_lib_fn::lua_lib_fn, tm_metatable::TM_METATABLE},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 判型/只读标志/元方法查探/落标志/压栈全经安全门面，体内已无真实裸操作，故本体降为安全 `fn`）：`l`
/// 须处于可抛错的受保护帧——栈 1 号位为 table（`check_type` 校验、非表即抛错发散）；两处 `arg_check`
/// 前提不成立即抛错不返回；`get_metafield_bytes` 命中时元方法留栈顶（与 cpp 同形），
/// `set_readonly`/`push_value` 可触发分配与 GC。
///
/// 只读标志与元方法各拆两语句现读（tfreeze r16-v3 #60 同判例）：被调收形为 `&mut`/`&` 实参形后与
/// `arg_check` 独占接收者借用冲突（E0499）；求值序本即先读判定后落 `arg_check`，句间无场写，拆句逐位等价。
/// cpp/VM/src/ltablib.cpp:639 tfreeze。
pub fn tfreeze(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);

  let not_frozen = lua_getreadonly(l, 1) == 0;
  l.arg_check(not_frozen, 1, "table is already frozen");

  let no_meta = !l.get_metafield_bytes(1, TM_METATABLE);
  l.arg_check(no_meta, 1, "table has a protected metatable");

  l.set_readonly(1, true);

  l.push_value(1);
  1
}

lua_lib_fn!(pub fn tfreeze @ref, tfreeze_arm);
