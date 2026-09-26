//! GlobalTypes 自指针 `builtin_types` 的唯一解引用 chokepoint。
//!
//! 手法对齐 `BuiltinTypes::arena_handle`：收 `NonNull` 入参的静态转铸函数
//! 承载 `# Safety` 契约（构造期尚未有 `Self` 可借用），实例方法仅转发到它。

use core::ptr::NonNull;

use crate::records::{builtin_types::BuiltinTypes, global_types::GlobalTypes};

impl GlobalTypes {
  /// 由 `builtin_types` 句柄取共享引用。
  ///
  /// # Safety
  /// `builtin_types` 须满足 C++ `NotNull<BuiltinTypes>` 接线契约：非空、对齐，
  /// 指向比本 `GlobalTypes` 长寿的单例（`Frontend` 自持字段或其宿主 Box，
  /// 由 `GlobalTypes::new` 入参 / `Frontend::wire_self_pointers` 落位后布线）。
  /// 返回借用生命周期刻意不绑定 `&self`/入参，与迁移前各调用点
  /// `unsafe { ptr.as_ref() }` 的借用检查行为完全同构；单线程序列化驱动
  ///（lib.rs 不变量 1）下借用期内无并存可变别名（BuiltinTypes 单例字段
  /// 构造后只读，arena 改写一律经 `arena_handle` 契约）。
  pub(crate) unsafe fn builtin_types_of<'a>(
    builtin_types: NonNull<BuiltinTypes>,
  ) -> &'a BuiltinTypes {
    // Safety: 契约即本函数 `# Safety` 前提。
    unsafe { builtin_types.as_ref() }
  }

  /// 实例形态：解引用本对象自指针的受控读取（契约见
  /// [`GlobalTypes::builtin_types_of`]；未布线窗口内调用属上游构造序契约违例）。
  pub(crate) fn builtin_types_ref<'a>(&self) -> &'a BuiltinTypes {
    // Safety: `self.builtin_types` 由 `GlobalTypes::new` 入参（`&mut` 引用转铸
    // Some）接线、`Frontend::wire_self_pointers` 落位后重布线为存活单例地址。
    // `None` 仅理论不可达（入参 `&mut` 恒非空），仍显式 panic 拦为契约违例，
    // 满足 `builtin_types_of` 的非空前提后才入 unsafe 解引用。
    unsafe { Self::builtin_types_of(self.builtins_handle()) }
  }

  /// 供 `builtin_types_ref` 复用的取址 helper：`None`（未接线，理论不可达——
  /// `GlobalTypes::new` 入参 `&mut` 恒转 `Some`）明确 panic 而非静默解引用。
  pub(crate) fn builtins_handle(&self) -> NonNull<BuiltinTypes> {
    self
      .builtin_types
      .expect("GlobalTypes::builtin_types 尚未接线（构造序契约违例）")
  }
}
