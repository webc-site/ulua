use alloc::vec::Vec;
use core::{mem::take, ptr::null_mut};

use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::FFlag;

use crate::{
  enums::type_context::TypeContext,
  functions::{
    add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns,
    checkpoint::checkpoint, for_each_constraint::for_each_constraint,
    get_mutable_type::get_mutable, has_free_type::has_free_type,
  },
  records::{
    blocked_type::BlockedType, constraint::Constraint, constraint_generator::ConstraintGenerator,
    generalization_constraint::GeneralizationConstraint,
    in_conditional_context::InConditionalContext, inference::Inference, scope::Scope,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_scope_ptr_ast_expr_function_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    func: *mut AstExprFunction,
    expected_type: Option<TypeId>,
    generalize: bool,
  ) -> Inference {
    let _in_context =
      unsafe { InConditionalContext::new(&mut self.type_context, TypeContext::Default) };

    let start_checkpoint = unsafe { checkpoint(self as *const _) };
    let sig =
      unsafe { self.check_function_signature(scope, null_mut(), func, expected_type, None) };

    self.interior_free_types.push(Default::default());
    self.check_function_body(&sig.body_scope, unsafe { &*func });
    let end_checkpoint = unsafe { checkpoint(self as *const _) };

    let generalized_ty = unsafe { (*self.arena).add_type(BlockedType::default()) };
    let gc = self.add_constraint_scope_ptr_location_constraint_v(
      &sig.signature_scope,
      unsafe { (*func).base.base.location },
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type: generalized_ty,
        source_type: sig.signature,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: false,
      }),
    );

    unsafe {
      let signature_scope = sig.signature_scope.as_ref() as *const Scope as *mut Scope;
      (*signature_scope).interior_free_types = Some(take(
        &mut self.interior_free_types.last_mut().unwrap().types,
      ));
      (*signature_scope).interior_free_type_packs = Some(take(
        &mut self.interior_free_types.last_mut().unwrap().type_packs,
      ));
    }
    self.interior_free_types.pop();

    // generalized_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3432
    // `getMutable<BlockedType>(generalizedTy)->setOwner(gc)`
    let blocked = get_mutable::<BlockedType>(generalized_ty).unwrap();
    blocked.set_owner(gc as *const _);

    if FFlag::LuauConstraintGraph.get() {
      unsafe {
        add_all_as_dependencies_and_chain_returns(start_checkpoint, end_checkpoint, self, gc)
      };
    } else {
      let mut previous: *mut Constraint = null_mut();
      for_each_constraint(
        start_checkpoint,
        end_checkpoint,
        self,
        |constraint: *mut Constraint| {
          unsafe { (*gc).deprecated_dependencies.push(constraint) };

          if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c }
            && psc.returns
          {
            if !previous.is_null() {
              unsafe { (*constraint).deprecated_dependencies.push(previous) };
            }
            previous = constraint;
          }
        },
      );
    }

    if generalize && has_free_type(sig.signature) {
      Inference::inference_type_id_refinement_id(generalized_ty, null_mut())
    } else {
      Inference::inference_type_id_refinement_id(sig.signature, null_mut())
    }
  }
}
