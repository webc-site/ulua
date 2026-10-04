use crate::{
  functions::{
    auxresume::auxresume, coresumefinish::coresumefinish, interrupt_thread::interrupt_thread,
  },
  macros::{co_status_break::CO_STATUS_BREAK, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：栈 1 号位经 `to_thread` 解析，非 thread 由 `arg_expected`
/// 抛错发散，故取回的 `co` 必为非空且存活的协程帧指针；`narg = get_top() - 1` 须 ≥0（索引 2 起为
/// 传入的恢复实参，均在 `(*l).base..(*l).top` 窗内）。首参已收形为引用形，保留 `unsafe fn` 的真
/// 前提在 `co` 一侧而非 `l` 的存活/独占：`auxresume`/`interrupt_thread` 本批未收形（其并发消费方
/// `auxwrapy`/`auxwrapfinish` 只许读），须把 `l` 的自身地址随 `co.resume(from)` 一路带上，被调方
/// 在驱动协程期间经该裸指针读写 `l` 的栈槽，与本帧持有的 `&mut l` 构成同帧两用借用——该跨帧转手
/// 前提不是 `&mut LuaState` 所能承载，故调用期间本函数不得再经 `l` 访问（`coresumefinish` 已收形
/// 为 `&mut`，末路直传独占借用、无裸转手）。
/// cpp VM/src/lcorolib.cpp:218
pub(crate) unsafe fn coresumey(l: &mut LuaState) -> i32 {
  let co = l.to_thread(1);
  l.arg_expected(co.is_some(), 1, "thread");
  let co = co.expect("arg_expected 已证 co 非空");

  // r16-b2 收编：顶-基槽距读数落既有 get_top 门面——其本体 slot_distance(base, top)
  // 即被替代式 `(top.offset_from(base) as i32) - 1` 的同址同宽镜像（现读位点不变、
  // `- 1` 次序保持）；isize→i32 折形在现域无截差（栈槽距受 LUAI_MAXSTACK 约束）
  let narg = l.get_top() - 1;

  // SAFETY: `l.as_mut_ptr()` 为本帧存活 LuaState 自身地址、`co` 为上方已证的非空存活协程，
  // 借用窗止于本次调用；`auxresume` 的可 GC/可抛错前提即其自身 `# Safety`
  // SAFETY: `l.as_mut_ptr()` 为本帧存活 LuaState 自身地址，`r` 为 `auxresume` 返回的 CO_STATUS_* 码
  let r = unsafe { auxresume(l.as_mut_ptr(), co, narg) };

  if r == CO_STATUS_BREAK {
    // SAFETY: 两指针前提同上；被调方只读 `l` 的 global 调试回调并驱动 `co`
    return unsafe { interrupt_thread(l.as_mut_ptr(), co) };
  }

  coresumefinish(l, r)
}

lua_lib_fn!(pub(crate) fn coresumey @ref, coresumey_arm);
