use crate::{
  enums::lua_type::LuaType,
  functions::{lua_newthread::lua_newthread, lua_xpush::lua_xpush},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 首参 `l` 的存活/独占前提已由 `&mut` 接收者类型承载（本票收形为引用形，`@ref` 臂于 FFI 一处
/// 重建）；保留 `unsafe fn` 的真前提在新线程指针 `nl` 一侧：`lua_newthread` 返回的 `*mut` 须被
/// 解引用为独占借用交 `lua_xpush`。
/// 调用序契约：`l` 须处于可分配、可 GC、可抛错的受保护帧——`check_type(1, FUNCTION)` 非函数实参
/// 抛错发散；`lua_newthread` 需 `top` 后 ≥1 空槽、写入 `top` 并可能触发 GC/线程屏障；
/// `lua_xpush(l, nl, 1)` 要求索引 1 的值存活并移入新线程 `nl` 栈。cpp VM/src/lcorolib.cpp:332。
pub unsafe fn cocreate(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Function);

  // SAFETY: `l.as_mut_ptr()` 为接收者保证的存活 LuaState 自身地址，交 `lua_newthread` 就地重建
  // 独占借用（其走 GC/线程屏障并在末尾压入新线程），借用窗止于本次调用
  let nl = unsafe { lua_newthread(l.as_mut_ptr()) };
  // SAFETY: `nl` 为 `lua_newthread` 刚压入的非空存活新协程线程；此处取独占借用交 `lua_xpush`
  // 把 `l` 索引 1 的值移入，`l` 为接收者独占借用、二者不同 LuaState 无别名，借用窗止于本次调用
  unsafe { lua_xpush(l, &mut *nl, 1) };

  1
}

lua_lib_fn!(pub fn cocreate @ref, cocreate_arm);
