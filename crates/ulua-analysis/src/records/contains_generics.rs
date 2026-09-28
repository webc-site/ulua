use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack,
    iterative_type_visitor::IterativeTypeVisitor,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone)]
pub struct ContainsGenerics {
  pub base: IterativeTypeVisitor,
  pub generics: *mut DenseHashSet<*const ()>,
  pub found: bool,
}

impl ContainsGenerics {
  pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    !self.found
  }

  pub fn visit_type_id_generic_type(&mut self, ty: TypeId, _gt: &GenericType) -> bool {
    // Safety: `generics` 由 `contains_generics_contains_generics` 原样保存调用方传入的
    // `*mut DenseHashSet`；该集合的源头一律经 `NonNull` 接线（如
    // `bidirectional_type_pusher` 构造形参 `NonNull<DenseHashSet<*const ()>>`，其实参在
    // `constraint_solver_try_dispatch` 里由 `NonNull::new(&mut set).unwrap()` 取得），故非空、
    // 对齐且在遍历结束前存活。此处只建只读借用并调 `contains`（不改集合），写的是
    // `self.found` 这个独立 bool，与集合无别名关系；键 `ty as *const ()` 只做身份
    // 比较，不解引用。
    unsafe {
      let set = &*self.generics;
      let key = ty as *const ();
      self.found |= set.contains(&key);
    }
    true
  }

  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    _ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    !self.found
  }

  pub fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    _gtp: &GenericTypePack,
  ) -> bool {
    // Safety: 同 `visit_type_id_generic_type` —— `generics` 是构造期从 `not_null` 命名
    // 契约的调用方接线的存活 `DenseHashSet`，只读取（`contains`）；`tp as *const ()`
    // 仅作身份比较的键，本函数不解引用它。写目标 `self.found` 与集合互不重叠。
    unsafe {
      let set = &*self.generics;
      let key = tp as *const ();
      self.found |= set.contains(&key);
    }
    !self.found
  }
}
