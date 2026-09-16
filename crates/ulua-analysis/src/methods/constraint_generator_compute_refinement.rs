use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use core::mem::ManuallyDrop;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::table_state::TableState,
  functions::{
    contains_subscripted_definition::contains_subscripted_definition, follow_type::follow_type_id,
    get_type_alt_j::get_type_id,
  },
  records::{
    conjunction_refinement::Conjunction, constraint_generator::ConstraintGenerator,
    disjunction_refinement::Disjunction, equivalence::Equivalence, negation_refinement::Negation,
    negation_type::NegationType, property_type::Property, proposition_refinement::Proposition,
    refinement_partition::RefinementPartition, scope::Scope, table_type::TableType,
    variadic::Variadic,
  },
  type_aliases::{
    constraint_v::ConstraintV,
    def_id_def::DefId,
    name_type::Name,
    refinement_context::RefinementContext,
    refinement_id_refinement::RefinementId,
    refinement_refinement::{Refinement, RefinementMember},
  },
};
impl ConstraintGenerator {
  // ConstraintGenerator::computeRefinement(const ScopePtr&, Location, RefinementId,
  //     RefinementContext*, bool sense, bool eq, std::vector<ConstraintV>*)
  // (ConstraintGenerator.cpp:565).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn compute_refinement(
    &mut self,
    scope: *mut Scope,
    location: Location,
    refinement: RefinementId,
    refis: *mut RefinementContext,
    sense: bool,
    eq: bool,
    constraints: *mut Vec<ConstraintV>,
  ) {
    if refinement.is_null() {
      return;
    }

    let refinement_ref: &Refinement = unsafe { &*refinement };

    if let Some(variadic) = <Variadic as RefinementMember>::get_if(refinement_ref) {
      for refi in variadic.refinements.clone() {
        unsafe { self.compute_refinement(scope, location, refi, refis, sense, eq, constraints) };
      }
    } else if let Some(negation) = <Negation as RefinementMember>::get_if(refinement_ref) {
      unsafe {
        self.compute_refinement(
          scope,
          location,
          negation.refinement,
          refis,
          !sense,
          eq,
          constraints,
        )
      };
    } else if let Some(conjunction) = <Conjunction as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (conjunction.lhs, conjunction.rhs);
      let mut lhs_refis = RefinementContext::default();
      let mut rhs_refis = RefinementContext::default();

      let lhs_target: *mut RefinementContext = if sense { refis } else { &mut lhs_refis };
      unsafe { self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints) };
      let rhs_target: *mut RefinementContext = if sense { refis } else { &mut rhs_refis };
      unsafe { self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints) };

      if !sense {
        let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });
        unsafe {
          self.union_refinements(&sp, location, &lhs_refis, &rhs_refis, refis, constraints)
        };
      }
    } else if let Some(disjunction) = <Disjunction as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (disjunction.lhs, disjunction.rhs);
      let mut lhs_refis = RefinementContext::default();
      let mut rhs_refis = RefinementContext::default();

      let lhs_target: *mut RefinementContext = if sense { &mut lhs_refis } else { refis };
      unsafe { self.compute_refinement(scope, location, lhs, lhs_target, sense, eq, constraints) };
      let rhs_target: *mut RefinementContext = if sense { &mut rhs_refis } else { refis };
      unsafe { self.compute_refinement(scope, location, rhs, rhs_target, sense, eq, constraints) };

      if sense {
        let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });
        unsafe {
          self.union_refinements(&sp, location, &lhs_refis, &rhs_refis, refis, constraints)
        };
      }
    } else if let Some(equivalence) = <Equivalence as RefinementMember>::get_if(refinement_ref) {
      let (lhs, rhs) = (equivalence.lhs, equivalence.rhs);
      unsafe { self.compute_refinement(scope, location, lhs, refis, sense, true, constraints) };
      unsafe { self.compute_refinement(scope, location, rhs, refis, sense, true, constraints) };
    } else if let Some(proposition) = <Proposition as RefinementMember>::get_if(refinement_ref) {
      let mut discriminant_ty = proposition.discriminant_ty;
      let prop_key = proposition.key;
      let implicit_from_call = proposition.implicit_from_call;

      // if we have a negative sense, then we need to negate the discriminant
      // 对照 C++：`if (auto nt = get<NegationType>(follow(discriminantTy))) discriminantTy = nt->ty;`
      if !sense {
        if let Some(nt) = get_type_id::<NegationType>(follow_type_id(discriminant_ty)) {
          discriminant_ty = nt.ty;
        } else {
          discriminant_ty = unsafe {
            (*self.arena).add_type(NegationType {
              ty: discriminant_ty,
            })
          };
        }
      }

      if eq {
        let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });
        let singleton_func = unsafe { &(*self.builtin_types).type_functions.singleton_func };
        discriminant_ty = self.create_type_function_instance(
          singleton_func,
          alloc::vec![discriminant_ty],
          alloc::vec![],
          &sp,
          location,
        );
      }

      let mut key = prop_key;
      while !key.is_null() {
        let key_def = unsafe { (*key).def } as DefId;

        unsafe {
          (*refis).insert(key_def, RefinementPartition::default());
          (*refis)
            .get_mut(&key_def)
            .unwrap()
            .discriminant_types
            .push(discriminant_ty);
        }

        // Reached leaf node
        let prop_name = unsafe { (*key).prop_name.clone() };
        let prop_name = match prop_name {
          Some(n) => n,
          None => break,
        };

        let mut props: BTreeMap<Name, Property> = BTreeMap::new();
        props.insert(prop_name, Property::readonly(discriminant_ty));

        let next_discriminant_ty = unsafe {
          let tt = TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &props,
            None,
            (*scope).level,
            scope,
            TableState::Sealed,
          );
          (*self.arena).add_type(tt)
        };

        discriminant_ty = next_discriminant_ty;

        key = unsafe { (*key).parent };
      }

      // When the top-level expression is `t[x]`, we want to refine it into `nil`, not `never`.
      let prop_def = unsafe { (*prop_key).def } as DefId;
      LUAU_ASSERT!(unsafe { (*refis).get(&prop_def) }.is_some());
      unsafe {
        (*refis).get_mut(&prop_def).unwrap().should_append_nil_type =
          (sense || !eq) && contains_subscripted_definition(prop_def) && !implicit_from_call;
      }
    }
  }
}
