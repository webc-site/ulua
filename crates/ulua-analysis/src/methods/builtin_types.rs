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
  /// 把该族的解引用 unsafe 收拢到本函数单点。
  ///
  /// # Safety
  /// `builtin_types` 须满足 C++ `NotNull<BuiltinTypes>` 接线契约：恒非空、
  /// 对齐，指向比经返回句柄物化的任何借用长寿的活 BuiltinTypes 单例；其
  /// `arena` 字段为 Box 独占堆分配、地址稳定；返回句柄物化的借用存续期间
  /// 不得另有存活可变别名指向同一 arena（构造序列单线程执行）。
  pub(crate) unsafe fn arena_handle(builtin_types: NonNull<BuiltinTypes>) -> Handle<TypeArena> {
    // Safety: 非空与存活契约即本函数 `# Safety` 前提；经裸指针 place 直取
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
