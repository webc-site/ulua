use crate::{
  enums::lua_type::LuaType,
  functions::{lua_g_readonlyerror::check_writable, lua_h_clear::lua_h_clear},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 判型经安全门面，体内裸操作仅余「1 号槽取表句柄 + 该句柄交 `check_writable`/`lua_h_clear`」的
/// api 索引域读数，其前提由紧邻上方的 `check_type` 与 `&mut` 借用窗就地建立、非调用方另证义务，
/// 故本体降为安全 `fn`）：`l` 须处于可抛错的受保护帧——栈 1 号位为 table（`check_type` 校验、非表
/// 即抛错发散）；只读表经 `check_writable` 抛错不返回。清空只动表自身的数组段/哈希段窗（窗长取自
/// 该表分配元数据，`lua_h_clear` 契约），不触栈。
/// cpp/VM/src/ltablib.cpp:627 tclear。
pub fn tclear(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);

  // SAFETY: 上方 `check_type` 保证 1 号槽为存活表；`slot` 为正帧索引界内换算，所得句柄与其
  // 分配元数据在本块内一致有效，`check_writable` 的 `l` 前提由 `as_mut_ptr` 的存活借用承载。
  unsafe {
    let tt = l.slot(1).get().as_table_ptr();
    check_writable(l.as_mut_ptr(), tt);
    lua_h_clear(tt);
  }

  0
}

lua_lib_fn!(pub fn tclear @ref, tclear_arm);
