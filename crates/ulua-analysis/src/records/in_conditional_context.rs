use crate::{enums::type_context::TypeContext, records::arena_handle::Handle};

/// RAII 守卫：构造时把宿主的 `TypeContext` 字段翻转为新值，`drop` 时写回旧值。
/// 字段以 [`Handle`] 别名持有（目标为宿主 `&mut` 独占的字段地址，守卫生命周期
/// 严格短于宿主借用，单线程串行），守卫不占用借用系统资源。对应 C++
/// `InConditionalContext`（`cpp/Analysis/include/Luau/TypeUtils.h:45`）。
#[derive(Debug)]
pub struct InConditionalContext {
  pub(crate) type_context: Handle<TypeContext>,
  pub(crate) old_value: TypeContext,
}
