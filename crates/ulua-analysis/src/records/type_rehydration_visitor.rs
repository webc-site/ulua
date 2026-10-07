use std::collections::BTreeMap;

use ulua_ast::records::allocator::Allocator;

use crate::{
  records::{
    arena_handle::alias, type_rehydration_options::TypeRehydrationOptions, visit_key::VisitKey,
  },
  type_aliases::synthetic_names::SyntheticNames,
};

#[derive(Debug)]
pub struct TypeRehydrationVisitor {
  pub(crate) seen: BTreeMap<VisitKey, i32>,
  pub(crate) count: i32,
  pub(crate) allocator: *mut Allocator,
  pub(crate) synthetic_names: *mut SyntheticNames,
  pub(crate) options: TypeRehydrationOptions,
}

impl TypeRehydrationVisitor {
  /// `self.allocator` 的统一读写入口：构造期由 TypeAttacher 交出的 arena 指针，
  /// 比本 visitor 长寿；`alias` 按 C++ 模型物化独占借用（单线程串行、借用窗口内
  /// 无并存别名），unsafe 不再渗进业务调用点；接收者为 `&self`
  /// 与本 visitor 多数方法一致（原 `unsafe { &mut *self.allocator }` 写穿惯用法同构）。返回借用与原裸指针解引用同构、
  /// 不经 `self` 生命周期，保持既有调用点的借用语义不变。
  #[inline]
  pub(crate) fn allocator_mut(&self) -> &'static mut Allocator {
    alias(self.allocator)
  }

  /// `self.synthetic_names` 的统一读写入口，与 [`Self::allocator_mut`] 同一
  /// 收口手法：指针构造期由 `&mut ta.synthetic_names` 存入（见 type_attacher
  /// 调用链），TypeAttacher 比 visitor 活得久；业务侧（`rehydrate_generic`）
  /// 不再直接 `alias` 裸字段。
  #[inline]
  pub(crate) fn synthetic_names_mut(&self) -> &'static mut SyntheticNames {
    alias(self.synthetic_names)
  }
}
