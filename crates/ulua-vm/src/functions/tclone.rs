use crate::{
  enums::lua_type::LuaType,
  functions::{lua_a_pushvalue::lua_a_pushvalue, lua_h_clone::lua_h_clone},
  macros::{lua_lib_fn::lua_lib_fn, sethvalue::sethvalue, tm_metatable::TM_METATABLE},
  records::{lua_state::LuaState, lua_t_value::TValue},
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 判型/元方法查探/压栈全经安全门面，体内仅两处 api 索引域裸读数（1 号槽取表句柄、该句柄转手
/// `lua_h_clone`），其前提由紧邻上方的 `check_type` 与 `&mut` 借用窗就地建立、非调用方另证义务，
/// 故本体降为安全 `fn`）：`l` 须处于可抛错、可分配/GC 的受保护帧——栈 1 号位为 table（`check_type`
/// 校验、非表即抛错发散）；元方法命中时 `arg_check` 抛错不返回；`lua_h_clone` 可触发分配与 GC 步进，
/// `lua_a_pushvalue` 需栈顶留结果余量。
///
/// 元方法查探拆两语句（tfreeze r16-v3 #60 同判例）：被调收形为 `&mut` 实参形后与 `arg_check`
/// 独占接收者借用冲突（E0499）；求值序本即先查元方法后落 arg_check，句间无场写，拆句逐位等价。
/// cpp/VM/src/ltablib.cpp:659 tclone。
pub fn tclone(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);

  let no_meta = !l.get_metafield_bytes(1, TM_METATABLE);
  l.arg_check(no_meta, 1, "table has a protected metatable");

  // SAFETY: 上方 `check_type` 保证 1 号槽为存活表；`slot` 为正帧索引界内换算，`as_table_ptr`
  // 即该槽的类型化读数（cpp `hvalue(L->base)`）。
  let src = unsafe { l.slot(1).get().as_table_ptr() };

  // SAFETY: `as_mut_ptr` 自 `&mut` 独占借用就地派生（借用窗止于本次调用），`src` 为刚判过的存活
  // 表句柄；克隆体由 GC 接管存活，落值前无其它别名。
  let tt = unsafe { lua_h_clone(l.as_mut_ptr(), src) };

  let mut v = TValue::default();
  // SAFETY: `v` 为本帧独占局部槽（写权由 `&mut v` 承载），`sethvalue!` 仅字段写 + debug 存活断言。
  unsafe { sethvalue!(l, &mut v, tt) };

  lua_a_pushvalue(l, &v);

  1
}

lua_lib_fn!(pub fn tclone @ref, tclone_arm);
