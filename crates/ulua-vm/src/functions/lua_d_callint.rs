use core::ptr::addr_of;

use crate::{
  functions::{lua_d_check_cstack::lua_d_check_cstack, performcall::performcall},
  macros::{
    isyielded::isyielded, lua_c_check_gc::lua_c_check_gc, lua_multret::LUA_MULTRET,
    luai_maxccalls::LUAI_MAXCCALLS, restoreci::restoreci, restorestack::restorestack,
    saveci::saveci, savestack::savestack,
  },
  records::{closure::CClosure, lua_state::LuaState},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_d_callint(
  l: *mut LuaState,
  func: StkId,
  nresults: i32,
  preparereentry: bool,
) {
  unsafe {
    (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
    if (*l).n_ccalls as i32 >= LUAI_MAXCCALLS {
      lua_d_check_cstack(l);
    }

    let mut fromyieldableccall = false;

    if (*l).ci != (*l).base_ci {
      let ccl = (*(*(*l).ci).func).as_closure_ptr();
      let cc = addr_of!((*ccl).inner.c).cast::<CClosure>();
      if (*ccl).is_c != 0 && (*cc).cont.is_some() {
        fromyieldableccall = true;
        (*l).base_ccalls = (*l).base_ccalls.wrapping_add(1);
      }
    }

    let funcoffset = savestack!(l, func);
    let cioffset = saveci!(l, (*l).ci);

    performcall(l, func, nresults, preparereentry);

    let yielded = isyielded(&*l);

    // r12-w7a2 收编（票面特别裁决位）：performcall 恢复点之后的两处
    // `restorestack!(l, funcoffset)` 裸重派生收为一次等价窗绑定——
    // restorestack_slot 为纯偏移算术、无副作用；绑定点与下面两个消费点之间仅有
    // 标量场读写（base_ccalls），无任何栈操作，逐位同值。跨恢复点时序不重排：
    // 两分支体的写入次序、`nresults == LUA_MULTRET ? 0 : nresults` 的求值形态
    // 与恢复动作本体（callerci->top / l->top 场写）全部原位保留。
    let funcslot = restorestack!(l, funcoffset);

    if fromyieldableccall {
      (*l).base_ccalls = (*l).base_ccalls.wrapping_sub(1);

      if yielded {
        let callerci = restoreci!(l, cioffset);
        (*callerci).top = funcslot.add(if nresults != LUA_MULTRET {
          nresults as usize
        } else {
          0
        });
      }
    }

    if nresults != LUA_MULTRET && !yielded {
      (*l).top = funcslot.add(nresults as usize);
    }

    (*l).n_ccalls = (*l).n_ccalls.wrapping_sub(1);
    lua_c_check_gc!(l);
  }
}
