use crate::{
  enums::type_context::TypeContext,
  records::{arena_handle::Handle, in_conditional_context::InConditionalContext},
};

impl InConditionalContext {
  /// C++ `InConditionalContext::InConditionalContext(TypeContext*, TypeContext)`
  /// (`cpp/Analysis/include/Luau/TypeUtils.h:45`)。
  ///
  /// 安全函数：`c` 由构造方以 `&mut` 独占借出的 `TypeContext` 字段引用，
  /// 内部仅存 [`Handle`] 别名（构造语句结束即释放借用），守卫生命周期内
  /// 宿主可照常进行其它 `&mut self` 调用——与 C++ 持 `TypeContext*` 同构。
  #[inline]
  pub fn new(c: &mut TypeContext, new_value: TypeContext) -> Self {
    let type_context = Handle::from_mut(c);
    let old_value = *type_context.get();
    *type_context.get_mut() = new_value;
    Self {
      type_context,
      old_value,
    }
  }
}

impl Drop for InConditionalContext {
  fn drop(&mut self) {
    // 不变量由构造维持：目标为宿主 `TypeContext` 字段，所有调用点均以作用域
    // 守卫（`let _g = InConditionalContext::new(...)`）使用，Self 严格先于宿主
    // 释放；写回旧值即恢复构造前状态。
    *self.type_context.get_mut() = self.old_value;
  }
}
