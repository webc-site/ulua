use core::ptr::addr_of_mut;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_c_barrierback::lua_c_barrierback, resume_error::resume_error},
  macros::{api_check::api_check, isblack::isblack, luai_maxccalls::LUAI_MAXCCALLS},
  records::{gc_object::GCObject, lua_state::LuaState},
};

/// NUL 结尾字节串（`*const c_char` 契约调用点 `.as_ptr().cast()`；§10 不引入 `CStr`/`c"…"`）。
const ERR_NOT_SUSPENDED: &[u8] = b"cannot resume non-suspended coroutine\0";
const ERR_C_STACK_OVERFLOW: &[u8] = b"C stack overflow\0";

/// 协程恢复的入口校验与 ccount 建档（cpp `resume_start`，与 `resume_finish` 成对）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `l` 须为待恢复协程状态且栈上实参不少于 `nargs`（由本函数 `api_check!` 核验）；
/// `from` 为发起恢复的外层协程，允许为空（主状态直接恢复无外层），非空时须存活。
pub(crate) unsafe fn resume_start(l: *mut LuaState, from: *mut LuaState, nargs: i32) -> i32 {
  unsafe {
    api_check!(l, nargs >= 0);
    api_check!(l, (*l).top.offset_from((*l).base) >= nargs as isize);

    if (*l).status != LuaStatus::Yield as u8
      && (*l).status != LuaStatus::Break as u8
      && ((*l).status != 0 || (*l).ci != (*l).base_ci)
    {
      return resume_error(l, ERR_NOT_SUSPENDED.as_ptr().cast(), nargs);
    }

    (*l).n_ccalls = if !from.is_null() { (*from).n_ccalls } else { 0 };
    if (*l).n_ccalls as i32 >= LUAI_MAXCCALLS {
      return resume_error(l, ERR_C_STACK_OVERFLOW.as_ptr().cast(), nargs);
    }

    (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
    (*l).base_ccalls = (*l).n_ccalls;
    (*l).isactive = true;

    let o = l as *mut GCObject;
    if isblack!(o) {
      lua_c_barrierback(l, o, addr_of_mut!((*l).gclist));
    }

    LuaStatus::Ok as i32
  }
}
