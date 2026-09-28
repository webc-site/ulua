use core::ptr::{null, null_mut};

use ulua_common::fflag::LuauNativeCodeTargetCheck;
use ulua_vm::{
  macros::{lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_multret::LUA_MULTRET},
  records::closure::Closure,
  type_aliases::stk_id::StkId,
};

use crate::{
  macros::vm_frame_support::CALL_FALLBACK_YIELD,
  records::vm_frame::VmFrame,
  type_aliases::{api::LuaState, ir::Instruction},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn call_fallback(
  l: *mut LuaState,
  ra: StkId,
  mut argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  // 契约: l/ra/argtop 由调用方(解释器分发或原生代码)按 Lua VM ABI 提供——ra 为活栈槽、
  // argtop 为参数区上界。边界契约集中于 `VmFrame::current`，以下建帧/搬移逻辑全部经帧
  // 方法以安全 API 进行，语句顺序与 cpp callFallback 逐字保持。
  // Safety: 本函数头 ABI 契约保证 `l` 为存活 LuaState；VmFrame::current 依 VM 调度不变量以 L->base 收帧，不解引用。
  let frame = unsafe { VmFrame::current(l) };

  if !frame.is_function(ra) {
    // __call 元方法就地改写活槽 ra, argtop 前移一参数位。
    frame.try_func_tm(ra);
    argtop = frame.slot_at(argtop, 1);
  }

  // 上方守卫后 ra 必为存活函数闭包。
  let ccl = frame.clvalue(ra);

  // incr_ci 推进的 ci 即当前帧（后续帧字段读写均经帧访问器落在该帧上），
  // func/base/top/savedpc/flags/nresults 按建帧序列接线。
  frame.incr_ci();
  frame.set_ci_func(ra);
  frame.set_ci_base(frame.slot_at(ra, 1));
  // cpp: ci->top = argtop + ccl->stacksize（栈尚未扩容，断言兜底）
  frame.set_ci_top(frame.slot_at(argtop, frame.closure_stacksize(ccl)));
  frame.set_ci_savedpc(null::<Instruction>());
  frame.set_ci_flags(0);
  frame.set_ci_nresults(nresults);

  // 以新帧重接 l->base/l->top（均为界内 StkId 值）。
  frame.set_lbase(frame.ci_base());
  frame.set_top(argtop);

  // 新帧栈空间检查, 断言兜底 top<=stack_last。
  frame.check_stack_for_new_ci(frame.closure_stacksize(ccl) as i32);
  ulua_common::LUAU_ASSERT!(frame.ci_top() <= frame.stack_last());

  // Lua 闭包分支: 补 nil 实参、挂 savedpc, 命中原生目标则打 NATIVE 旗标。
  if !frame.closure_is_c(ccl) {
    let p = frame.closure_proto(ccl);

    // [top, base+numparams) 为帧内空槽补 nil, top 依 vararg 语义推进。
    // cpp: L->top = p->is_vararg ? argi : ci->top，argi 为补 nil 循环终点——参数充足时
    // 为 argend，实参超出 numparams 时保持原 top（varargs 不被截断）。
    let argi = frame.top();
    let argend = frame.slot_at(frame.lbase(), frame.proto_numparams(p) as usize);
    let nfill = frame.slots_diff(argend, argi);
    for slot in frame.slots_mut(argi, nfill.max(0) as usize) {
      slot.set_nil();
    }
    frame.set_top(if frame.proto_is_vararg(p) != 0 {
      if nfill > 0 { argend } else { argi }
    } else {
      frame.ci_top()
    });

    frame.set_ci_savedpc(frame.proto_code(p));

    // exectarget/execdata 为活 Proto 字段读数。
    let has_native_target = if LuauNativeCodeTargetCheck.get() {
      frame.proto_exectarget(p) != 0
    } else {
      !frame.proto_execdata(p).is_null()
    };
    if has_native_target {
      frame.set_ci_flags(LUA_CALLINFO_NATIVE as u32);
    }

    return ccl;
  }

  // C 闭包分支: 直接调用并按 nresults 收敛返回值（inner.c.f 为闭包存活期内有效的
  // 宿主函数指针，经 C-unwind ABI 调出）。
  let n = match frame.closure_c_function(ccl) {
    // Safety: `ccl` 为上方守卫确认的存活 C 闭包，`inner.c.f` 随闭包存活且为
    // extern "C-unwind" 宿主函数指针，`l` 即本 VM 状态——与 cpp 同款 lua_call 语义。
    Some(f) => unsafe { f(l) },
    None => 0,
  };

  if n < 0 {
    return CALL_FALLBACK_YIELD as usize as *mut Closure;
  }

  // 此刻 C 帧仍挂在 L->ci 上: cip 即调用者帧, res 为其被调槽, [vali, valend) 为返回值区。
  let cip = frame.parent_ci();
  let mut res = frame.ci_func();
  let valend = frame.top();
  let mut vali = frame.slot_back(valend, n as usize);

  let mut i = nresults;
  // 值拷贝循环以 i 与 [vali, valend) 双界收敛, 全程落在分配栈内。
  while i != 0 && vali < valend {
    frame.set_stack_value(res, vali);
    res = frame.slot_at(res, 1);
    vali = frame.slot_at(vali, 1);
    i -= 1;
  }
  // 返回值不足部分以 nil 补足至 nresults。
  while i > 0 {
    frame.set_nil(res);
    res = frame.slot_at(res, 1);
    i -= 1;
  }
  // 弹帧并恢复 base/top(MULTRET 时 top 取写入前沿 res)。
  frame.set_ci(cip);
  frame.set_lbase(frame.ci_base_at(cip));
  frame.set_top(if nresults == LUA_MULTRET {
    res
  } else {
    frame.ci_top_at(cip)
  });

  null_mut()
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn call_fallback_export(
  l: *mut LuaState,
  ra: StkId,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  // Safety: extern "C-unwind" 导出入口按原样转发 l/ra/argtop/nresults 给同契约的 unsafe fn
  // call_fallback; 参数由 VM/原生代码按 C ABI 提供且指向活对象, 满足被调前置条件, 本块不新增解引用。
  unsafe { call_fallback(l, ra, argtop, nresults) }
}
