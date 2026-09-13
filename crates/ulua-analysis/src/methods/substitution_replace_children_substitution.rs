use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    extern_type::ExternType, function_type::FunctionType, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType,
    pending_expansion_type::PendingExpansionType, substitution::Substitution,
    table_type::TableType, type_function_instance_type::TypeFunctionInstanceType,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl Substitution {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn replace_children_type_id(&mut self, ty: TypeId) {
    unsafe {
      LUAU_ASSERT!(ty == (*self.base.log).follow_type_id(ty));
    }

    if self.base.ignore_children_type_id(ty) {
      return;
    }

    if unsafe { (*ty).owning_arena != self.arena } {
      return;
    }

    if let Some(ftv) = get_mutable_type_id::<FunctionType>(ty) {
      for generic in &mut ftv.generics {
        *generic = self.replace_type_id(*generic);
      }
      for generic_pack in &mut ftv.generic_packs {
        *generic_pack = self.replace_type_pack_id(*generic_pack);
      }
      ftv.arg_types = self.replace_type_pack_id(ftv.arg_types);
      ftv.ret_types = self.replace_type_pack_id(ftv.ret_types);
    } else if let Some(ttv) = get_mutable_type_id::<TableType>(ty) {
      LUAU_ASSERT!(ttv.bound_to.is_none());
      for prop in ttv.props.values_mut() {
        if let Some(read_ty) = prop.read_ty {
          prop.read_ty = Some(self.replace_type_id(read_ty));
        }
        if let Some(write_ty) = prop.write_ty {
          prop.write_ty = Some(self.replace_type_id(write_ty));
        }
      }
      if let Some(ref mut indexer) = ttv.indexer {
        indexer.index_type = self.replace_type_id(indexer.index_type);
        indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
      }
      for itp in &mut ttv.instantiated_type_params {
        *itp = self.replace_type_id(*itp);
      }
      for itp in &mut ttv.instantiated_type_pack_params {
        *itp = self.replace_type_pack_id(*itp);
      }
    } else if let Some(mtv) = get_mutable_type_id::<MetatableType>(ty) {
      mtv.table = self.replace_type_id(mtv.table);
      mtv.metatable = self.replace_type_id(mtv.metatable);
    } else if let Some(utv) = get_mutable_type_id::<UnionType>(ty) {
      for opt in &mut utv.options {
        *opt = self.replace_type_id(*opt);
      }
    } else if let Some(itv) = get_mutable_type_id::<IntersectionType>(ty) {
      for part in &mut itv.parts {
        *part = self.replace_type_id(*part);
      }
    } else if let Some(petv) = get_mutable_type_id::<PendingExpansionType>(ty) {
      for a in &mut petv.type_arguments {
        *a = self.replace_type_id(*a);
      }
      for a in &mut petv.pack_arguments {
        *a = self.replace_type_pack_id(*a);
      }
    } else if let Some(tfit) = get_mutable_type_id::<TypeFunctionInstanceType>(ty) {
      for a in &mut tfit.type_arguments {
        *a = self.replace_type_id(*a);
      }
      for a in &mut tfit.pack_arguments {
        *a = self.replace_type_pack_id(*a);
      }
    } else if let Some(etv) = get_mutable_type_id::<ExternType>(ty) {
      for prop in etv.props.values_mut() {
        if let Some(read_ty) = prop.read_ty {
          prop.read_ty = Some(self.replace_type_id(read_ty));
        }
        if let Some(write_ty) = prop.write_ty {
          prop.write_ty = Some(self.replace_type_id(write_ty));
        }
      }
      if let Some(ref mut parent) = etv.parent {
        *parent = self.replace_type_id(*parent);
      }
      if let Some(ref mut metatable) = etv.metatable {
        *metatable = self.replace_type_id(*metatable);
      }
      if let Some(ref mut indexer) = etv.indexer {
        indexer.index_type = self.replace_type_id(indexer.index_type);
        indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
      }
    } else if let Some(ntv) = get_mutable_type_id::<NegationType>(ty) {
      ntv.ty = self.replace_type_id(ntv.ty);
    }
  }
}
