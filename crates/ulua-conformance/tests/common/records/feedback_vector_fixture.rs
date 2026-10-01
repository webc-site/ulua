use core::ptr::null_mut;

use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{fflag, fint};
use ulua_vm::{
  functions::lua_newstate::lua_newstate,
  records::{closure::Closure, lua_state::LuaState, proto::Proto},
};

use crate::common::{
  functions::alloc::alloc as luau_alloc,
  records::state_ref::StateRef,
  type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
};

/// cpp `FeedbackVector.test.cpp` 每个 `TEST_F` 在 fixture 构造时统一打开的三条
/// call-feedback 旗标：`LuauEmitCallFeedback` / `LuauCallFeedback` +
/// `LuauInlineHitsThreshold = 2`（阈值降到 2 让 call feedback 在两次调用后恰好封槽）。
/// 收进本结构后，8 个同构用例不再各写一遍三行 `ScopedFastFlag::new` + 空行。
struct CallFeedbackFlags {
  _emit_call_fb: ScopedFastFlag,
  _call_fb: ScopedFastFlag,
  _inline_threshold: ScopedFastInt,
}

impl CallFeedbackFlags {
  fn new() -> Self {
    Self {
      _emit_call_fb: ScopedFastFlag::new(&fflag::LuauEmitCallFeedback, true),
      _call_fb: ScopedFastFlag::new(&fflag::LuauCallFeedback, true),
      _inline_threshold: ScopedFastInt::new(&fint::LuauInlineHitsThreshold, 2),
    }
  }
}

pub struct FeedbackVectorFixture<'a> {
  pub bcb: BytecodeBuilder<'a>,
  pub l: StateRef,
  pub on_inline: Option<
    unsafe extern "C-unwind" fn(*mut LuaState, *mut Closure, *mut Closure, u32) -> *mut Proto,
  >,
  /// 声明在 `l` 之后：结构字段按声明顺序 Drop，故先 `lua_close`（`l`）再弹旗标守卫，
  /// 与原用例「先声明三条旗标、后建状态、逆序析构」的收尾顺序一致。
  _call_feedback_flags: CallFeedbackFlags,
}

impl<'a> FeedbackVectorFixture<'a> {
  pub fn new() -> Self {
    // FFI: c-API 要求 NULL
    let state = unsafe { lua_newstate(Some(luau_alloc), null_mut()) };
    let l = StateRef::new(state).expect("lua_newstate failed");

    Self {
      bcb: BytecodeBuilder::new(None),
      l,
      on_inline: None,
      _call_feedback_flags: CallFeedbackFlags::new(),
    }
  }

  pub fn lua_state(&self) -> *mut LuaState {
    self.l.as_ptr()
  }
}

impl Default for FeedbackVectorFixture<'_> {
  fn default() -> Self {
    Self::new()
  }
}
