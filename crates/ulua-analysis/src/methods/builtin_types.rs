//! `builtin_types` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::boxed::Box;
use core::{mem::replace, ptr::NonNull};

use ulua_common::fflag;

use crate::{
  functions::unfreeze::unfreeze,
  records::{arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl BuiltinTypes {
  pub fn any_type_pack(&self) -> TypePackId {
    self.any_type_pack
  }
}

// 构造期 arena 可变句柄唯一入口：C++
// `NotNull<TypeArena> arena{builtinTypes->arena.get()}`（`GlobalTypes.cpp`
// 构造序列惯用法）的 Rust 形态 chokepoint。

impl BuiltinTypes {
  /// 将 `self.arena` 的可变借用物化为 `Handle<TypeArena>`，替代原先散布于
  /// GlobalTypes 构造序列（`make_string_metatable`、本构造器的
  /// unfreeze/freeze 两点）的双重转铸 shim
  /// `&mut *(&mut *NonNull<BuiltinTypes>.as_mut().arena as *mut TypeArena)`，
  /// 把该族的解引用收拢到本函数单点。
  ///
  /// 调用序契约（正确性，非内存安全）：`builtin_types` 须为 C++ `NotNull<BuiltinTypes>`
  /// 接线语义下的会话句柄——由 GlobalTypes/Frontend 布线指向存活 BuiltinTypes 单例、
  /// 比返回句柄物化的任何借用长寿；其 `arena` 字段为 Box 独占堆分配、地址稳定；
  /// 返回句柄物化的借用存续期间不得另有存活可变别名指向同一 arena（构造序列
  /// 单线程执行）。句柄形态的存活前提由本 crate 的 `NonNull`/`Handle` 会话接线
  /// 不变量承载（同 `alias_nn*` safe 门面族），解引用 unsafe 收口在体内窄块。
  pub(crate) fn arena_handle(builtin_types: NonNull<BuiltinTypes>) -> Handle<TypeArena> {
    // Safety: 非空与存活契约即函数头的会话接线不变量；经裸指针 place 直取
    // `arena` 字段，可变借用只覆盖 Box 独占的 TypeArena 堆块字节，与原调用点
    // `&mut *builtin_types.as_mut().arena` 的转铸形态逐项同构。
    unsafe { Handle::from_mut(&mut (*builtin_types.as_ptr()).arena) }
  }
}

impl BuiltinTypes {
  pub fn boolean_type(&self) -> TypeId {
    self.boolean_type
  }
}

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

impl BuiltinTypes {
  pub fn empty_table_type(&self) -> TypeId {
    self.empty_table_type
  }
}

impl BuiltinTypes {
  pub fn error_recovery_type(&self, guess: TypeId) -> TypeId {
    guess
  }
}

impl BuiltinTypes {
  pub fn error_recovery_type_pack(&self, guess: TypePackId) -> TypePackId {
    guess
  }
}

impl BuiltinTypes {
  pub fn number_type(&self) -> TypeId {
    self.number_type
  }
}

impl BuiltinTypes {
  pub fn string_type(&self) -> TypeId {
    self.string_type
  }
}
