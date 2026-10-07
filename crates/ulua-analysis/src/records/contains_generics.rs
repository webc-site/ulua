use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    arena_handle::alias_ref, generic_type::GenericType, generic_type_pack::GenericTypePack,
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
    // 键 `ty as *const ()` 只做身份比较，不解引用；写的是 `self.found` 这个
    // 独立 bool，与集合无别名关系。
    let set = alias_ref(self.generics);
    let key = ty as *const ();
    self.found |= set.contains(&key);
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
    // 键 `tp as *const ()` 仅作身份比较，本函数不解引用它；写目标 `self.found`
    // 与集合互不重叠。
    let set = alias_ref(self.generics);
    let key = tp as *const ();
    self.found |= set.contains(&key);
    !self.found
  }
}
