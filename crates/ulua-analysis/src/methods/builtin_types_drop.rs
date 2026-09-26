use alloc::boxed::Box;
use core::mem::replace;

use ulua_common::fflag;

use crate::{
  functions::unfreeze::unfreeze,
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
};
impl Drop for BuiltinTypes {
  fn drop(&mut self) {
    // thread-local override 而非全局 set/restore（cpp 直写 `FValue::value`，多线程
    // 下互相踩踏全局标志）：unfreeze 经 get() 读到本线程 override，单线程语义与
    // cpp 一致，并行使用者互不干扰。
    fflag::DebugLuauFreezeArena.push_test_override(self.debug_freeze_arena);

    unfreeze(&mut self.arena);
    let arena = replace(&mut self.arena, Box::new(TypeArena::default()));
    drop(arena);

    fflag::DebugLuauFreezeArena.pop_test_override();
  }
}
