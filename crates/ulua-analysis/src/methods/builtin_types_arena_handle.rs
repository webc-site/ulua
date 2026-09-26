//! 构造期 arena 可变句柄唯一入口：C++
//! `NotNull<TypeArena> arena{builtinTypes->arena.get()}`（`GlobalTypes.cpp`
//! 构造序列惯用法）的 Rust 形态 chokepoint。

use core::ptr::NonNull;

use crate::records::{arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena};

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
