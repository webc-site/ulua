//! `lua_f_recordhit` — record a call-target hit in the caller's feedback vector.
//! C++ source: `VM/src/lfunc.cpp:225`
//!
//! Returns `true` if the inline threshold has not yet been reached for this
//! slot (caller should continue speculation), `false` otherwise.

use ulua_common::{fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::feedback_vector_slot_kind::FeedbackVectorSlotKind,
  records::{closure::Closure, lua_state::LuaState},
};

/// # Safety
/// `caller`/`target` 须为存活 `Closure`（cpp lfunc.cpp:225）：caller 必须为 Lua 闭包且其
/// `feedbackvec` 非空、`slotid < feedbackvecsize`（断言即该契约的 debug 兜底）；target 仅当
/// `is_c == 0` 时才读 `inner.l.p`，故分支前不解引用。`l` 须存活以读取内联回调。
pub unsafe fn lua_f_recordhit(
  l: *mut LuaState,
  caller: *mut Closure,
  target: *mut Closure,
  slotid: u32,
) -> bool {
  // Safety: 契约保证 caller/target 为存活闭包且 target 反馈向量非空，slotid 经 sizecode 上界检查后按槽读写
  unsafe {
    // cpp lfunc.cpp:225：回调缺失时不参与内联计数
    let Some(inline_fn) = (*(*l).global).ecb.inlinefunction else {
      return false;
    };

    LUAU_ASSERT!((*caller).is_c == 0);
    let callerp = (*caller).inner.l.p;

    if (*target).is_c != 0 {
      return false;
    }
    let targetp = (*target).inner.l.p;

    LUAU_ASSERT!(slotid < (*callerp).feedbackvecsize);
    let slot = &mut *(*callerp).feedbackvec.add(slotid as usize);
    LUAU_ASSERT!(slot.kind == FeedbackVectorSlotKind::CallTarget);

    if slot.data.call_target.proto == 0 {
      slot.data.call_target.proto = (*targetp).funid;
    }

    if slot.data.call_target.proto != (*targetp).funid {
      return false;
    }

    slot.data.call_target.hits += 1;

    if slot.data.call_target.hits as i32 >= fint::LuauInlineHitsThreshold.get() {
      inline_fn(l, caller, target, slot.data.call_target.pc);
      return false;
    }

    true
  }
}
