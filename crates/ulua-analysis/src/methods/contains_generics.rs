//! `contains_generics` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, vec::Vec};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    contains_generics::ContainsGenerics,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    iterative_type_visitor::{IterativeTypeVisitor, IterativeTypeVisitorTrait},
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ContainsGenerics {
  pub fn contains_generics_contains_generics(generics: *mut DenseHashSet<*const ()>) -> Self {
    ContainsGenerics {
      base: IterativeTypeVisitor {
        seen: DenseHashSet::default(),
        work_queue: Vec::new(),
        parent_cursor: -1,
        work_cursor: 0,
        visitor_name: String::from("ContainsGenerics"),
        skip_bound_types: true,
        visit_once: true,
      },
      generics,
      found: false,
    }
  }
}

impl IterativeTypeVisitorTrait for ContainsGenerics {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    &mut self.base
  }

  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    ContainsGenerics::visit_type_id(self, ty)
  }

  fn visit_type_id_generic_type(&mut self, ty: TypeId, gt: &GenericType) -> bool {
    ContainsGenerics::visit_type_id_generic_type(self, ty, gt)
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    ContainsGenerics::visit_type_id_type_function_instance_type(self, ty, tfit)
  }

  fn visit_type_pack_id_generic_type_pack(
    &mut self,
    tp: TypePackId,
    gtp: &GenericTypePack,
  ) -> bool {
    ContainsGenerics::visit_type_pack_id_generic_type_pack(self, tp, gtp)
  }
}
