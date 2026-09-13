//! C++ `std::pair<TypeIds, TypePackIds> Constraint::getMaybeMutatedTypes() const`
//! (Constraint.cpp:93-211).
//!
//! This file also carries the `GenericTypeVisitorTrait` impl for
//! `ReferenceCountInitializer`: the per-`visit` node files declare the
//! overrides as inherent methods (the established decomposition for this
//! record), and this impl is the trait wiring that lets `traverse(...)`
//! dispatch into them — the same `FindCyclicTypes` precedent, but kept local
//! to a `getMaybeMutatedTypes` call site so the override node files stay
//! untouched.
use core::ffi::c_void;
/// Trait wiring for the `ReferenceCountInitializer` overrides (declared as
/// inherent methods in the sibling `..._visit_constraint*` node files). C++
/// `ReferenceCountInitializer : TypeOnceVisitor` (Constraint.h:384).
use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    assign_index_constraint::AssignIndexConstraint,
    assign_prop_constraint::AssignPropConstraint,
    blocked_type::BlockedType,
    blocked_type_pack::BlockedTypePack,
    constraint::Constraint,
    equality_constraint::EqualityConstraint,
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_call_constraint::FunctionCallConstraint,
    function_check_constraint::FunctionCheckConstraint,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    has_indexer_constraint::HasIndexerConstraint,
    has_prop_constraint::HasPropConstraint,
    iterable_constraint::IterableConstraint,
    name_constraint::NameConstraint,
    pack_subtype_constraint::PackSubtypeConstraint,
    pending_expansion_type::PendingExpansionType,
    primitive_type_constraint::PrimitiveTypeConstraint,
    push_function_type_constraint::PushFunctionTypeConstraint,
    push_type_constraint::PushTypeConstraint,
    reduce_pack_constraint::ReducePackConstraint,
    reference_count_initializer::ReferenceCountInitializer,
    subtype_constraint::SubtypeConstraint,
    table_type::TableType,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_function_instance_type::TypeFunctionInstanceType,
    type_ids::TypeIds,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{
    constraint_v::ConstraintVMember, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_ids::TypePackIds,
  },
};
impl GenericTypeVisitorTrait for ReferenceCountInitializer {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
    ReferenceCountInitializer::visit_type_id_free_type(self, ty, ftv)
  }

  fn visit_type_id_blocked_type(&mut self, ty: TypeId, btv: &BlockedType) -> bool {
    ReferenceCountInitializer::visit_type_id_blocked_type(self, ty, btv)
  }

  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    ReferenceCountInitializer::visit_type_id_pending_expansion_type(self, ty, petv)
  }

  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    ReferenceCountInitializer::visit_type_id_table_type(self, ty, ttv)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    ReferenceCountInitializer::visit_type_id_extern_type(self, ty, etv)
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    ReferenceCountInitializer::visit_type_id_type_function_instance_type(self, ty, tfit)
  }

  fn visit_type_pack_id_blocked_type_pack(
    &mut self,
    tp: TypePackId,
    btp: &BlockedTypePack,
  ) -> bool {
    ReferenceCountInitializer::visit_type_pack_id_blocked_type_pack(self, tp, btp)
  }

  fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, ftp: &FreeTypePack) -> bool {
    ReferenceCountInitializer::visit_type_pack_id_free_type_pack(self, tp, ftp)
  }
}

impl Constraint {
  pub fn get_maybe_mutated_types(&self) -> (TypeIds, TypePackIds) {
    // For the purpose of this function and reference counting in general, we are only considering
    // mutations that affect the _bounds_ of the free type, and not something that may bind the free
    // type itself to a new type. As such, `ReduceConstraint` and `GeneralizationConstraint` have no
    // contribution to the output set here.

    let mut types = TypeIds::new();

    // NOTE: In the future we'd like to track references to type packs, so we're
    // adding this local, but we do not modify it.
    let mut type_packs: TypePackIds = TypePackIds::new(null());

    let mut rci =
      ReferenceCountInitializer::reference_count_initializer_reference_count_initializer(
        &mut types,
        &mut type_packs,
      );

    if let Some(ec) = EqualityConstraint::get_if(&self.c) {
      rci.traverse_type_id(ec.result_type);
      rci.traverse_type_id(ec.assignment_type);
    } else if let Some(sc) = SubtypeConstraint::get_if(&self.c) {
      rci.traverse_type_id(sc.sub_type);
      rci.traverse_type_id(sc.super_type);
    } else if let Some(psc) = PackSubtypeConstraint::get_if(&self.c) {
      rci.traverse_type_pack_id(psc.sub_pack);
      rci.traverse_type_pack_id(psc.super_pack);
    } else if let Some(itc) = IterableConstraint::get_if(&self.c) {
      for &ty in itc.variables.iter() {
        rci.traverse_type_id(ty);
      }
      // `IterableConstraints` should not mutate `iterator`.
    } else if let Some(nc) = NameConstraint::get_if(&self.c) {
      rci.traverse_type_id(nc.named_type);
    } else if let Some(taec) = TypeAliasExpansionConstraint::get_if(&self.c) {
      rci.traverse_type_id(taec.target);
    } else if let Some(fchc) = FunctionCheckConstraint::get_if(&self.c) {
      rci.traverse_type_pack_id(fchc.args_pack);
    } else if let Some(fcc) = FunctionCallConstraint::get_if(&self.c) {
      rci.traverse_into_type_functions = false;
      rci.traverse_type_id(fcc.fn_type);
      rci.traverse_type_pack_id(fcc.args_pack);
      rci.traverse_into_type_functions = true;
    } else if let Some(ptc) = PrimitiveTypeConstraint::get_if(&self.c) {
      rci.traverse_type_id(ptc.free_type);
    } else if let Some(hpc) = HasPropConstraint::get_if(&self.c) {
      rci.traverse_type_id(hpc.result_type);
      rci.traverse_type_id(hpc.subject_type);
    } else if let Some(hic) = HasIndexerConstraint::get_if(&self.c) {
      rci.traverse_type_id(hic.subject_type);
      rci.traverse_type_id(hic.result_type);
      // `HasIndexerConstraint` should not mutate `index_type`.
    } else if let Some(apc) = AssignPropConstraint::get_if(&self.c) {
      rci.traverse_type_id(apc.lhs_type);
      rci.traverse_type_id(apc.rhs_type);
    } else if let Some(aic) = AssignIndexConstraint::get_if(&self.c) {
      rci.traverse_type_id(aic.lhs_type);
      rci.traverse_type_id(aic.index_type);
      rci.traverse_type_id(aic.rhs_type);
    } else if let Some(uc) = UnpackConstraint::get_if(&self.c) {
      for &ty in uc.result_pack.iter() {
        rci.traverse_type_id(ty);
      }
      // Consider:
      //
      //  function set(dictionary, key, value)
      //      local new = table.clone(dictionary)
      //      new[key] = value
      //      return new
      //  end
      //
      // In this case, we would expect `dictionary` to be inferred as
      // something like `{ [T]: K }` for some generic `T` and `K`.
      // However, in order to avoid eagerly generalizing dictionary,
      // we need to track that it may be mutated by the line:
      //
      //  new[key] = value
      //
      // ... this implies that `UnpackConstraint` can mutate both
      // it's LHS and RHS operands. LHS directly, and RHS by proxy.
      rci.traverse_type_pack_id(uc.source_pack);
    } else if let Some(rpc) = ReducePackConstraint::get_if(&self.c) {
      rci.traverse_type_pack_id(rpc.tp);
    } else if let Some(pftc) = PushFunctionTypeConstraint::get_if(&self.c) {
      rci.traverse_type_id(pftc.function_type);
    } else if let Some(ptc) = PushTypeConstraint::get_if(&self.c) {
      rci.traverse_type_id(ptc.target_type);
    }

    (types, type_packs)
  }
}
