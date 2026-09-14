use ulua_vm::{
  macros::{lua_multret::LUA_MULTRET, setnilvalue::setnilvalue, setobj_2_s::setobj2s},
  records::lua_state::lua_State,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn call_epilog_c(l: *mut lua_State, nresults: i32, n: i32) {
  unsafe {
    // ci 是当前 CallInfo，cip 是父帧
    let ci = (*l).ci;
    let cip = ci.offset(-1);

    // 返回值拷回父帧栈（只到 nresults），其余填 nil
    // MULTRET 时 nresults 为 -1，i != 0 条件有意永不生效
    let mut res: StkId = (*ci).func;
    let mut vali: StkId = (*l).top.offset(-n as isize);
    let valend: StkId = (*l).top;

    let mut i: i32 = nresults;
    while i != 0 && vali < valend {
      setobj2s!(l, res, vali);
      res = res.add(1);
      vali = vali.add(1);
      i -= 1;
    }

    while i > 0 {
      setnilvalue!(res);
      res = res.add(1);
      i -= 1;
    }

    // 弹出栈帧
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
#[unsafe(export_name = "ulua_callEpilogC")]
pub unsafe extern "C-unwind" fn call_epilog_c_export(l: *mut lua_State, nresults: i32, n: i32) {
  unsafe { call_epilog_c(l, nresults, n) }
}
