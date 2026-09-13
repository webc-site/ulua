use core::ptr::NonNull;

use crate::{
  enums::unify_result::UnifyResult,
  records::{
    any_type::AnyType, builtin_types::BuiltinTypes, table_type::TableType, unifier_2::Unifier2,
  },
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn unify_any_type_table_type(
    &mut self,
    _sub_any: &AnyType,
    super_table: &TableType,
  ) -> UnifyResult {
    let builtin_types_ptr: NonNull<BuiltinTypes> = self.builtin_types;
    let builtin_types_ref: &BuiltinTypes = unsafe { builtin_types_ptr.as_ref() };
    let any_type_id: TypeId = builtin_types_ref.any_type;

    for prop in super_table.props.values() {
      if let Some(read_ty) = prop.read_ty {
        let _ = self.unify_type_id_type_id(any_type_id, read_ty);
      }

      if let Some(write_ty) = prop.write_ty {
        let _ = self.unify_type_id_type_id(write_ty, any_type_id);
      }
    }

    if let Some(indexer) = &super_table.indexer {
      let _ = self.unify_type_id_type_id(any_type_id, indexer.index_type);
      let _ = self.unify_type_id_type_id(any_type_id, indexer.index_result_type);
    }

    UnifyResult::Ok
  }
}
