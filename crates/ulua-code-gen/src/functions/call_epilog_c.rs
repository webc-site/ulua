use ulua_vm::{
  macros::{lua_multret::LUA_MULTRET, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn call_epilog_c(l: *mut LuaState, nresults: i32, n: i32) {
  // 契约: l 为活的 LuaState(VM 调用约定保证), (*l).ci 落在 CallInfo 数组内、
  // cip=ci-1 为其父帧亦在数组内; res=ci->func、vali=l->top-n、valend=l->top 均指向
  // 当前栈内活对象, 循环按 nresults 与 [vali,valend) 收敛拷贝, 指针加法不越出栈界。
  // 拆分仅收窄 unsafe 词法范围, 循环体逐字未动。
  // ci 是当前 CallInfo，cip 是父帧
  // Safety: 依契约; ci 与 ci.offset(-1) 均在 CallInfo 数组内, top 读数为活栈指针。
  let (cip, mut res, mut vali, valend) = unsafe {
    let ci = (*l).ci;
    let cip = ci.offset(-1);
    (cip, (*ci).func, (*l).top.offset(-n as isize), (*l).top)
  };

  // 返回值拷回父帧栈（只到 nresults），其余填 nil
  // MULTRET 时 nresults 为 -1，i != 0 条件有意永不生效
  let mut i: i32 = nresults;
  // Safety: 依契约; 拷贝以 i 与 [vali,valend) 双界收敛, res 跟随推进不越栈界。
  unsafe {
    while i != 0 && vali < valend {
      setobj_2_s!(l, res, vali);
      res = res.add(1);
      vali = vali.add(1);
      i -= 1;
    }
  }
  // Safety: 依契约; 缺口以 nil 补足至 nresults。
  unsafe {
    while i > 0 {
      setnilvalue!(res);
      res = res.add(1);
      i -= 1;
    }
  }

  // 弹出栈帧
  // Safety: 依契约; 弹帧与 base/top 回写均界内(MULTRET 时 top 取写入前沿 res)。
  unsafe {
    (*l).ci = cip;
    (*l).base = (*cip).base;
    (*l).top = if nresults == LUA_MULTRET {
      res
    } else {
      (*cip).top
    };
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn call_epilog_c_export(l: *mut LuaState, nresults: i32, n: i32) {
  // Safety: 本 extern "C-unwind" 入口原样转发其收到的参数给同契约的 unsafe fn call_epilog_c;
  // 调用方(native 代码/VM 分发)已按 Lua C ABI 保证 l 为活 LuaState 且 n 为待拷返回值个数,
  // 满足被调函数的 unsafe 前置条件, 此处不引入额外解引用。
  unsafe { call_epilog_c(l, nresults, n) }
}
