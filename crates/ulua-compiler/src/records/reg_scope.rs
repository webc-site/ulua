//! Source: `Compiler/src/Compiler.cpp`

use crate::records::compiler::Compiler;

/// RAII 寄存器栈守卫。**禁止** `Copy`/`Clone`：cpp `~RegScope` 在作用域退出时
/// 执行 `regTop = oldTop`，由下方 `Drop` 复现。旧实现曾 derive `Copy`，静默
/// 丢失回卷——兄弟子表达式之间 `reg_top` 不再恢复，触发
/// `reg_scope_top` 的 `assert!(top <= reg_top)`。
///
/// 挂账(b) 终判（批次13）：**保留 `*mut Compiler` 裸资源形态，论点成立**。论证：
/// 1. `&'a mut Compiler`（或 `PhantomData` 等价）不可行——守卫存活期间调用方必须
///    继续以 `&mut self` 编译子表达式（`let rs = self.reg_scope();
///    self.compile_expr(..)`），且 `compile_l_value`/`compile_l_value_index`/
///    `compile_expr_auto` 以 `&mut RegScope` 形参作寄存器作用域见证（守卫不持
///    真实借用是这套 witness 协议成立的前提）。若守卫带生命周期，其存活期使 `'a`
///    存续、与宿主全部后续 `&mut self` 调用互斥，整个 compile_* 方法树（30+
///    构造点、含嵌套守卫）须闭包化重构才能编译；cpp scope-guard 惯用法在此必须
///    越过借用检查器。
/// 2. `Cell` 内部可变不可行——`reg_top` 是 `Compiler` 公开字段、全树读写，改造
///    面远超守卫；且守卫嵌套次序仍是运行时约定，unsafe 只是搬家。
/// 3. 手动回卷（返回 old_top 由调用方恢复）不可行——`CompileError::raise` 发散
///    路径下无人恢复，恰在最需要回卷时报错逃逸，丢失 RAII 保证。
///
/// 收口面已最小化：字段 `pub(crate)`，全 crate 仅 `reg_scope`/`reg_scope_top`
/// 两个构造点（见其文档：裸地址契约）；非 Copy/Clone；`Drop` 回写的独占无别名
/// 证明见下。登记于 `docs/UNSAFE-AUDIT.md` §6。
#[derive(Debug)]
pub(crate) struct RegScope {
  pub(crate) self_: *mut Compiler,
  pub(crate) old_top: u32,
}

impl Drop for RegScope {
  fn drop(&mut self) {
    // 对应 cpp `~RegScope() { self->regTop = oldTop; }`
    // Safety: self_ 由 reg_scope/reg_scope_top 从 &mut self 现身取得
    // （`self as *mut _`），非空对齐；RegScope 生命周期严格嵌套于该可变
    // 借用内且 Drop 先于借用结束，此刻无任何存活的 Compiler 借用/引用，
    // 回写 reg_top 独占无别名冲突。
    unsafe {
      (*self.self_).reg_top = self.old_top;
    }
  }
}
