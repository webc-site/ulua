use core::cell::UnsafeCell;

use crate::records::lint_context::LintContext;

/// lint pass 期间宿主 `LintContext` 的**非空**写句柄：把 lint 族唯一的 `&mut`
/// 重建收口在这一个点（`review.md` §2 第 5 条：`&UnsafeCell` 最小封装，unsafe
/// 不渗透到业务逻辑）。record 字段存句柄而非裸自指针，构造即接线，不存在
/// null 哨兵的构造期悬置窗口。
#[derive(Debug, Clone, Copy)]
pub(crate) struct LintContextHandle<'ctx>(&'ctx UnsafeCell<LintContext>);

impl<'ctx> LintContextHandle<'ctx> {
  /// 由宿主 `&mut LintContext` 构造；句柄存活期内源头借用让渡给本句柄。
  pub(crate) fn from_ref(context: &'ctx mut LintContext) -> Self {
    Self(UnsafeCell::from_mut(context))
  }

  /// 调用点零 unsafe 的写访问。
  pub(crate) fn get(&mut self) -> &mut LintContext {
    // SAFETY: 本句柄只能由 `from_ref` 从 `&'ctx mut LintContext` 构造，非空且
    // 指向存活 'ctx 的宿主；`get` 取 `&mut self`（句柄本体），故同一句柄副本上
    // 的两次可变重建在借用检查下互斥串行。lint 族由 `functions/lint.rs` 逐
    // flag 串行驱动：每次 `process` 内单线程遍历 AST，visitor 经 `&mut self`
    // 独占 pass 状态，宿主 `LintContext` 在该 pass 期间无其它活动借用（源头
    // `&mut` 已整体让渡给句柄），重建出的 `&mut` 借用半径止于各调用语句/局部
    // 作用域，不并存第二指向同一宿主的引用。
    unsafe { &mut *self.0.get() }
  }
}
