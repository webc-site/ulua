// ConstraintGenerator::visit(const ScopePtr&, AstStatTypeFunction*) (ConstraintGenerator.cpp:2194-2271).
use alloc::{string::String, vec::Vec};
use core::{ffi::CStr, mem::take, ptr::null_mut};

use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns,
    as_mutable_type::as_mutable_type_id, checkpoint::checkpoint, follow_type::follow,
    for_each_constraint::for_each_constraint, get_mutable_type::get_mutable,
    get_type_alt_j::get as get_type,
  },
  records::{
    blocked_type::BlockedType, constraint::Constraint, constraint_generator::ConstraintGenerator,
    generalization_constraint::GeneralizationConstraint, reserved_identifier::ReservedIdentifier,
    scope::Scope, symbol::Symbol,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_variant::TypeVariant,
  },
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_type_function(
    &mut self,
    scope: &ScopePtr,
    function: *mut AstStatTypeFunction,
  ) -> ControlFlow {
    let function_ref = unsafe { &*function };

    // function->name == "typeof"
    let name_bytes = unsafe { CStr::from_ptr(function_ref.name.value) }.to_bytes();
    if name_bytes == b"typeof" {
      self.report_error(
        function_ref.base.base.location,
        TypeErrorData::ReservedIdentifier(ReservedIdentifier::new(String::from("typeof"))),
      );
    }

    let scope_it = self
      .ast_type_function_environment_scopes
      .find(&(function as *const AstStatTypeFunction))
      .cloned();
    LUAU_ASSERT!(scope_it.is_some());

    let environment_scope: ScopePtr = scope_it.unwrap().unwrap();

    let start_checkpoint = unsafe { checkpoint(self as *const _) };
    let sig = unsafe {
      self.check_function_signature(
        &environment_scope,
        null_mut(),
        function_ref.body,
        None,
        None,
      )
    };

    // Place this function as a child of the non-type function scope.
    unsafe {
      (*(scope.as_ref() as *const Scope as *mut Scope))
        .children
        .push(sig.signature_scope.as_ref() as *const Scope as *mut Scope);
    }
    self.interior_free_types.push(Default::default());
    self.check_function_body(&sig.body_scope, unsafe { &*function_ref.body });
    let end_checkpoint = unsafe { checkpoint(self as *const _) };

    let generalized_ty: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
    let gc: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
      &sig.signature_scope,
      function_ref.base.base.location,
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

    // generalized_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:2248
    // `getMutable<BlockedType>(generalizedType)->setOwner(gc)`
    let blocked = get_mutable::<BlockedType>(generalized_ty).unwrap();
    blocked.set_owner(gc as *const _);
    self.interior_free_types.pop();

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

    let existing_function_ty = unsafe {
      (*(environment_scope.as_ref() as *const Scope as *mut Scope))
        .lookup_symbol(Symbol::from_global(function_ref.name))
    };

    if existing_function_ty.is_none() {
      unsafe {
        (*self.ice).ice_string_location(
          "checkAliases did not populate type function name",
          &function_ref.name_location,
        );
      }
    }

    let unpacked_ty = follow(existing_function_ty.unwrap());

    // 对照 C++:2259 `if (auto bt = get<BlockedType>(unpackedTy); bt && nullptr == bt->getOwner())`
    if let Some(bt) = get_type::<BlockedType>(unpacked_ty)
      && bt.get_owner().is_null()
    {
      // SAFETY: unpacked_ty 是 arena 内的类型句柄，与 C++ asMutable 同契约。
      unsafe {
        (*as_mutable_type_id(unpacked_ty)).ty = TypeVariant::Bound(generalized_ty);
      }
    }

    ControlFlow::None
  }
}
