use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    extern_type::ExternType, function_type::FunctionType, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType,
    pending_expansion_type::PendingExpansionType, table_type::TableType, tarjan::Tarjan,
    r#type::Type, type_function_instance_type::TypeFunctionInstanceType, union_type::UnionType,
  },
  type_aliases::{nominal_relation::NominalRelation, type_id::TypeId},
};
impl Tarjan {
  pub fn visit_children_type_id_i32(&mut self, ty: TypeId, _index: i32) {
    let mut ty = ty;
    unsafe {
      LUAU_ASSERT!(ty == (*self.log).follow_type_id(ty));
    }

    if self.ignore_children_visit_type_id(ty) {
      return;
    }

    let pty = unsafe { (*self.log).pending_type_id(ty) };
    if !pty.is_null() {
      ty = unsafe { &(*pty).pending as *const Type };
    }

    if let Some(ftv) = get_type_id::<FunctionType>(ty) {
      for generic in ftv.generics.iter() {
        self.visit_child_type_id(*generic);
      }
      for generic_pack in ftv.generic_packs.iter() {
        self.visit_child_type_pack_id(*generic_pack);
      }

      self.visit_child_type_pack_id(ftv.arg_types);
      self.visit_child_type_pack_id(ftv.ret_types);
      return;
    }

    if let Some(ttv) = get_type_id::<TableType>(ty) {
      LUAU_ASSERT!(ttv.bound_to.is_none());
      for prop in ttv.props.values() {
        self.visit_child_optional_ty(prop.read_ty);
        self.visit_child_optional_ty(prop.write_ty);
      }

      if let Some(ref indexer) = ttv.indexer {
        self.visit_child_type_id(indexer.index_type);
        self.visit_child_type_id(indexer.index_result_type);
      }

      for itp in ttv.instantiated_type_params.iter() {
        self.visit_child_type_id(*itp);
      }

      for itp in ttv.instantiated_type_pack_params.iter() {
        self.visit_child_type_pack_id(*itp);
      }
      return;
    }

    if let Some(mtv) = get_type_id::<MetatableType>(ty) {
      self.visit_child_type_id(mtv.table);
      self.visit_child_type_id(mtv.metatable);
      return;
    }

    if let Some(utv) = get_type_id::<UnionType>(ty) {
      for opt in utv.options.iter() {
        self.visit_child_type_id(*opt);
      }
      return;
    }

    if let Some(itv) = get_type_id::<IntersectionType>(ty) {
      for part in itv.parts.iter() {
        self.visit_child_type_id(*part);
      }
      return;
    }

    if let Some(petv) = get_type_id::<PendingExpansionType>(ty) {
      for a in petv.type_arguments.iter() {
        self.visit_child_type_id(*a);
      }
      for a in petv.pack_arguments.iter() {
        self.visit_child_type_pack_id(*a);
      }
      return;
    }

    if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty) {
      for a in tfit.type_arguments.iter() {
        self.visit_child_type_id(*a);
      }
      for a in tfit.pack_arguments.iter() {
        self.visit_child_type_pack_id(*a);
      }
      return;
    }

    if let Some(etv) = get_type_id::<ExternType>(ty) {
      for prop in etv.props.values() {
        if prop.read_ty.is_some() {
          self.visit_child_optional_ty(prop.read_ty);
        }
        if prop.write_ty.is_some() {
          self.visit_child_optional_ty(prop.write_ty);
        }
      }

      if let Some(parent) = etv.parent {
        self.visit_child_type_id(parent);
      }

      if let Some(metatable) = etv.metatable {
        self.visit_child_type_id(metatable);
      }

      if let Some(ref indexer) = etv.indexer {
        self.visit_child_type_id(indexer.index_type);
        self.visit_child_type_id(indexer.index_result_type);
      }

      if FFlag::DebugLuauUserDefinedClasses.get()
        && let Some(ref relation) = etv.relation
      {
        match relation {
          NominalRelation::V0(obj) => {
            self.visit_child_type_id(obj.ty);
          }
          NominalRelation::V1(klass) => {
            self.visit_child_type_id(klass.ty);
          }
        }
      }
      return;
    }

    if let Some(ntv) = get_type_id::<NegationType>(ty) {
      self.visit_child_type_id(ntv.ty);
    }
  }
}
