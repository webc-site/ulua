//! `luaF_recordhit` — record a call-target hit in the caller's feedback vector.
//! C++ source: `VM/src/lfunc.cpp:225`
//!
//! Returns `true` if the inline threshold has not yet been reached for this
//! slot (caller should continue speculation), `false` otherwise.

use ulua_common::{FInt, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::feedback_vector_slot_kind::FeedbackVectorSlotKind,
  records::{closure::Closure, lua_state::lua_State},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaF_recordhit")]
pub unsafe fn lua_f_recordhit(
  l: *mut lua_State,
  caller: *mut Closure,
  target: *mut Closure,
  slotid: u32,
) -> bool {
  unsafe {
    if (*(*l).global).ecb.inlinefunction.is_none() {
      return false;
    }

    LUAU_ASSERT!((*caller).is_c == 0);
    let callerp = (*(core::ptr::addr_of!((*caller).inner.l))).p;

    if (*target).is_c != 0 {
      return false;
    }
    let targetp = (*(core::ptr::addr_of!((*target).inner.l))).p;

    LUAU_ASSERT!(slotid < (*callerp).feedbackvecsize);
    let slot = (*callerp).feedbackvec.add(slotid as usize);
    LUAU_ASSERT!((*slot).kind == FeedbackVectorSlotKind::CallTarget);

    if (*slot).data.call_target.proto == 0 {
      (*slot).data.call_target.proto = (*targetp).funid;
    }

    if (*slot).data.call_target.proto != (*targetp).funid {
      return false;
    }

    (*slot).data.call_target.hits += 1;

    if (*slot).data.call_target.hits as i32 >= FInt::LuauInlineHitsThreshold.get() {
      if let Some(inline_fn) = (*(*l).global).ecb.inlinefunction {
        inline_fn(l, caller, target, (*slot).data.call_target.pc);
      }
      return false;
    }

    true
  }
}

pub use lua_f_recordhit as luaF_recordhit;
