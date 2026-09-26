use core::ptr::{null, null_mut};

use ulua_common::fflag::LuauNativeCodeTargetCheck;
use ulua_vm::{
  functions::lua_v_tryfunc_tm::lua_v_tryfunc_tm,
  macros::{
    lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_d_checkstackfornewci::lua_d_checkstackfornewci,
    lua_multret::LUA_MULTRET, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s,
  },
  records::{closure::Closure, lua_state::LuaState},
  type_aliases::stk_id::StkId,
};

use crate::{
  functions::call_prolog::incr_ci,
  macros::vm_frame_support::CALL_FALLBACK_YIELD,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn call_fallback(
  l: *mut LuaState,
  ra: StkId,
  mut argtop: StkId,
  nresults: i32,
) -> *mut Closure {
  // 契约: l/ra/argtop 由调用方(解释器分发或原生代码)按 Lua VM ABI 提供: ra 为活栈槽
  // (is_function()/clvalue! 解引用它), incr_ci 取的 ci 在 CallInfo 数组内且其 func/base/top
  // 由下方赋值界定在 l->stack_last 内(有 LUAU_ASSERT 兜底); ccl 闭包与其 inner.l.p/inner.c.f
  // 在闭包存活期内有效。拆分仅收窄 unsafe 词法范围, 语句顺序与实参逐字保持。
  if unsafe { !(*ra).is_function() } {
    // Safety: 依契约; __call 元方法就地改写活槽 ra, argtop 前移一参数位。
    unsafe {
      lua_v_tryfunc_tm(l, ra);
      argtop = argtop.add(1);
    };
  }

  // Safety: 依契约; 上方守卫后 ra 必为存活函数闭包, as_closure_ptr 类型化读数。
  let ccl = unsafe { (*ra).as_closure_ptr() };

  // Safety: 依契约; incr_ci 返回数组内活 ci, func/base/top 赋值为界内 StkId 算术。
  let ci = unsafe {
    let ci = incr_ci(l);
    (*ci).func = ra;
    (*ci).base = ra.add(1);
    (*ci).top = argtop.add((*ccl).stacksize as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;
    ci
  };

  // Safety: 依契约; 以新帧 ci 重接 l->base/l->top（均为界内 StkId 值）。
  unsafe {
    (*l).base = (*ci).base;
    (*l).top = argtop;
  }

  // Safety: 依契约; 新帧栈空间检查, 断言兜底 top<=stack_last。
  unsafe {
    lua_d_checkstackfornewci(l, (*ccl).stacksize as i32);
    ulua_common::LUAU_ASSERT!((*ci).top <= (*l).stack_last);
  }

  // Lua 闭包分支: 补 nil 实参、挂 savedpc, 命中原生目标则打 NATIVE 旗标。
  if unsafe { (*ccl).is_c } == 0 {
    // Safety: 依契约; L 闭包 inner.l.p 为非空活 Proto。
    let p = unsafe { (*ccl).inner.l.p };

    // Safety: 依契约; [top, base+numparams) 为帧内空槽补 nil, top 依 vararg 语义推进。
    unsafe {
      let mut argi = (*l).top;
      let argend = (*l).base.add((*p).numparams as usize);
      while argi < argend {
        setnilvalue!(argi);
        argi = argi.add(1);
      }
      (*l).top = if (*p).is_vararg != 0 { argi } else { (*ci).top };

      (*ci).savedpc = (*p).code;
    }

    // Safety: 依契约; exectarget/execdata 为活 Proto 字段读数。
    let has_native_target = unsafe {
      if LuauNativeCodeTargetCheck.get() {
        (*p).exectarget != 0
      } else {
        !(*p).execdata.is_null()
      }
    };
    if has_native_target {
      // Safety: 依契约; 活 ci 旗标位写入。
      unsafe { (*ci).flags = LUA_CALLINFO_NATIVE as u32 };
    }

    return ccl;
  }

  // C 闭包分支: 直接调用并按 nresults 收敛返回值。
  // Safety: 依契约; inner.c.f 为闭包存活期内有效函数指针, l 为活 state。
  let n = unsafe {
    let func = (*ccl).inner.c.f;
    match func {
      Some(f) => f(l),
      None => 0,
    }
  };

  if n < 0 {
    return CALL_FALLBACK_YIELD as usize as *mut Closure;
  }

  // Safety: 依契约; 此刻 C 帧已弹出, (*l).ci.sub(1) 即调用者帧 cip; func/top 读数界内。
  let (cip, mut res, mut vali, valend) = unsafe {
    let ci = (*l).ci;
    let cip = ci.sub(1);
    (cip, (*ci).func, (*l).top.sub(n as usize), (*l).top)
  };

  let mut i = nresults;
  // Safety: 依契约; 值拷贝循环以 i 与 [vali, valend) 双界收敛, 全程落在分配栈内。
  unsafe {
    while i != 0 && vali < valend {
      setobj_2_s!(l, res, vali);
      res = res.add(1);
      vali = vali.add(1);
      i -= 1;
    }
  }
  // Safety: 依契约; 返回值不足部分以 nil 补足至 nresults。
  unsafe {
    while i > 0 {
      setnilvalue!(res);
      res = res.add(1);
      i -= 1;
    }
  }
  // Safety: 依契约; 弹帧并恢复 base/top(MULTRET 时 top 取写入前沿 res)。
  unsafe {
    (*l).ci = cip;
    (*l).base = (*cip).base;
    (*l).top = if nresults == LUA_MULTRET {
      res
    } else {
      (*cip).top
    };
  }

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
