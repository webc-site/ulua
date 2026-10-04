use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    auxresumecont::auxresumecont, coresumefinish::coresumefinish,
    interrupt_thread::interrupt_thread,
  },
  macros::lua_lib_fn::lua_cont_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 并处于 resume 续体的受保护帧，栈 1 号位为 coroutine：`to_thread` 解析后
/// `arg_expected` 先证 `co` 非空，本函数才经该指针读 `(*co).status`（越界即解引用空/悬垂指针）。首参
/// 已收形为引用形，保留 `unsafe fn` 的真前提仍在 `co` 一侧：`auxresumecont`/`interrupt_thread`/
/// `coresumefinish` 本批未收形，须把 `l` 的自身地址交其重建借用，被调方在续延期经该裸指针读写 `l`
/// 栈槽，与本帧 `&mut l` 构成同帧两用借用，调用期间本函数不得再经 `l` 访问；`status` 按裸 `i32`
/// 透传且本实现不读它（cpp 同形 `_status` 未用）。cpp/VM/src/lcorolib.cpp:239 coresumecont。
pub(crate) unsafe fn coresumecont(l: &mut LuaState, _status: i32) -> i32 {
  let co = l.to_thread(1);
  l.arg_expected(co.is_some(), 1, "thread");
  let co = co.expect("arg_expected 已证 co 非空");

  // if coroutine still hasn't yielded after the break, break current thread again
  // SAFETY: `co` 为上方 arg_expected 已证的存活协程帧，此处只读其 status 字段
  if unsafe { (*co).status == LuaStatus::Break as u8 } {
    // SAFETY: `l.as_mut_ptr()` 为本帧存活 LuaState 自身地址、`co` 同上，借用窗止于本次调用
    return unsafe { interrupt_thread(l.as_mut_ptr(), co) };
  }

  // SAFETY: 两指针前提同上；`auxresumecont` 的可 GC/可抛错前提即其自身 `# Safety`
  let r = unsafe { auxresumecont(l.as_mut_ptr(), co) };
  // SAFETY: 两指针前提同上；`r` 为 auxresumecont 返回的 CO_STATUS_* 码
  unsafe { coresumefinish(l.as_mut_ptr(), r) }
}

lua_cont_fn!(pub(crate) fn coresumecont @ref, coresumecont_arm);
