/// 为 cpp 刻意共享裸指针三件套（`build`/`function`/`stats`）的四个结构
/// （`IrLoweringX64`/`IrLoweringA64`/`IrRegAllocX64`/`IrRegAllocA64`）生成同名
/// 访问器家族：`function_mut`/`function_ref`/`build_mut`/`stats_mut`。
///
/// 统一契约（与各结构体文档登记一致）：三个指针由构造点（`lower_function`）以
/// 可变引用注入为裸指针，所指对象覆盖整个降级栈帧、比本对象长寿且与本对象无
/// 内存重叠；访问器即时派生借用、语句内消费，不长期持有。`stats` 可为空（顶层
/// 允许不采集统计），由 `stats_mut` 统一判空。宏只收口 4×4 份逐位重复的手写体，
/// 不引入新语义；各结构特有的混窗/闭包窗门面（`build_regs_mut`、`with_op_label`
/// 等）仍在各自文件手写。
#[macro_export]
macro_rules! shared_ptr_accessors {
  ($build_ty:ty) => {
    /// Safety:`function` 由构造点以 `&mut IrFunction` 注入为裸指针，所指对象覆盖
    /// 整个降级栈帧、比本对象长寿且与本对象无内存重叠；此处即时派生唯一可变
    /// 借用、语句内消费。
    #[inline]
    pub(crate) fn function_mut(&mut self) -> &mut IrFunction {
      unsafe { &mut *self.function }
    }

    /// 只读视图：供 `&self` 接收者的读取门面（操作数取值、判空前的字段读）使用。
    ///
    /// Safety:同 `function_mut`；仅派生共享借用。
    #[inline]
    pub(crate) fn function_ref(&self) -> &IrFunction {
      unsafe { &*self.function }
    }

    /// 发射视图访问器：`build` 的注入契约同 `function_mut`；同一调用语句内还需
    /// 其它视图时走各结构手写的混窗门面（`build_regs_mut` 等），不复用本访问器。
    ///
    /// Safety:见结构体注释；即时派生唯一可变借用、语句内消费。
    #[inline]
    pub(crate) fn build_mut(&mut self) -> &mut $build_ty {
      unsafe { &mut *self.build }
    }

    /// 统计指针可为空（顶层允许不采集统计），收敛判空 + 解引用样板。
    ///
    /// Safety:`stats` 非空时契约同 `function_mut`。
    #[inline]
    pub(crate) fn stats_mut(&mut self) -> Option<&mut LoweringStats> {
      if self.stats.is_null() {
        return None;
      }
      // Safety:同 `function_mut`。
      Some(unsafe { &mut *self.stats })
    }
  };
}

pub use shared_ptr_accessors;
