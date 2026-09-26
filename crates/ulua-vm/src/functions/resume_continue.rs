use core::ptr::addr_of;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    luau_execute::luau_execute, luau_finishop::luau_finishop, luau_poscall::luau_poscall,
  },
  macros::{
    curr_func::curr_func, lua_callinfo_handle::LUA_CALLINFO_HANDLE,
    lua_callinfo_opyield::LUA_CALLINFO_OPYIELD, scheduled_reentry::SCHEDULED_REENTRY,
  },
  records::{closure::CClosure, lua_state::LuaState},
};

/// 展开 Luau/C 混合栈、逐帧运行续体直至挂起/出错（cpp `resume_continue`）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `l` 须为存活协程状态，且 `resume_start`→驱动→本函数的成对协议已建立：ci 栈
/// 上仅含恢复流程构造的续体帧（C `cont` 或 Lua 主体），`base_ccalls == n_ccalls`
/// 由循环内 `LUAU_ASSERT!` 把守。
pub(crate) unsafe fn resume_continue(l: *mut LuaState) {
  unsafe {
    // unroll Luau/C combined stack, processing continuations
    while ((*l).status == LuaStatus::Ok as u8 || (*l).status == SCHEDULED_REENTRY as u8)
      && (*l).ci > (*l).base_ci
    {
      LUAU_ASSERT!((*l).base_ccalls == (*l).n_ccalls);

      (*l).status = LuaStatus::Ok as u8;

      let cl = curr_func!(l);

      if (*cl).is_c != 0 {
        // C continuation; we expect this to be followed by Lua continuations
        let c = addr_of!((*cl).inner.c).cast::<CClosure>();
        let cont_opt = (*c).cont;
        LUAU_ASSERT!(cont_opt.is_some());

        if let Some(cont) = cont_opt {
          // continuation can use non-protected calls again
          (*(*l).ci).flags &= !(LUA_CALLINFO_HANDLE as u32);

          let n = cont(l, 0);

          // continuation can break or yield again
          if (*l).status == LuaStatus::Break as u8 || (*l).status == LuaStatus::Yield as u8 {
            break;
          }

          if (*l).status == SCHEDULED_REENTRY as u8 {
            continue;
          }

          luau_poscall(l, (*l).top.offset(-(n as isize)));
        }
      } else {
        if ((*(*l).ci).flags & LUA_CALLINFO_OPYIELD as u32) != 0 {
          luau_finishop(l);
        }

        // Luau continuation; it terminates at the end of the stack or at another C continuation
        luau_execute(l);
      }
    }
  }
}
