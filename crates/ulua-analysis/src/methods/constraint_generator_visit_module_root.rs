use alloc::{sync::Arc, vec::Vec};

use ulua_ast::records::{ast_node::AstNode, ast_stat_block::AstStatBlock, location::Location};
use ulua_common::{
  FFlag,
  macros::{luau_assert::LUAU_ASSERT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE},
};

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  functions::{
    add_all_as_dependencies::add_all_as_dependencies, as_mutable_type::as_mutable_type_id,
    checkpoint::checkpoint, follow_type::follow_type_id, for_each_constraint::for_each_constraint,
    get_mutable_type::get_mutable, get_type_alt_j::get_type_id,
  },
  records::{
    blocked_type::BlockedType, constraint::Constraint, constraint_generator::ConstraintGenerator,
    function_type::FunctionType, generalization_constraint::GeneralizationConstraint,
    interior_free_types::InteriorFreeTypes, module::Module,
    pack_subtype_constraint::PackSubtypeConstraint, scope::Scope,
    simplify_constraint::SimplifyConstraint,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId, type_variant::TypeVariant,
  },
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_module_root(&mut self, block: *mut AstStatBlock) {
    LUAU_TIMETRACE_SCOPE!("ConstraintGenerator::visitModuleRoot", "Typechecking");

    LUAU_ASSERT!(self.scopes.is_empty());
    LUAU_ASSERT!(self.root_scope.is_null());

    let scope: ScopePtr = Arc::new(Scope::new(self.global_scope.as_ref().unwrap(), 0));
    self.root_scope = scope.as_ref() as *const Scope as *mut Scope;
    self
      .scopes
      .push((unsafe { (*block).base.base.location }, scope.clone()));
    unsafe {
      (*self.root_scope).location = (*block).base.base.location;
    }
    if let Some(module) = &self.module {
      let module_ptr = Arc::as_ptr(module) as *mut Module;
      unsafe {
        *(*module_ptr)
          .ast_scopes
          .get_or_insert(block as *const AstNode) = scope.as_ref() as *const Scope as *mut Scope;
      }
    }

    self.interior_free_types.push(InteriorFreeTypes::default());

    let local_type_function_scope: ScopePtr =
      Arc::new(Scope::new(self.type_function_scope.as_ref().unwrap(), 0));
    unsafe {
      let lhs = local_type_function_scope.as_ref() as *const Scope as *mut Scope;
      (*lhs).location = (*block).base.base.location;
    }
    unsafe {
      (*self.type_function_runtime).root_scope = local_type_function_scope;
    }

    let return_type = self.fresh_type_pack(&scope, Polarity::Positive);
    unsafe {
      (*self.root_scope).return_type = return_type;
    }
    let module_fn_ty = unsafe {
      (*self.arena).add_type(FunctionType::function_type_new(
        (*self.builtin_types).any_type_pack,
        return_type,
        None,
        false,
      ))
    };

    unsafe { self.prepopulate_global_scope(&scope, block) };

    let start = unsafe { checkpoint(self) };

    let cf = unsafe { self.visit_block_without_child_scope(self.root_scope, block) };
    if cf == ControlFlow::None {
      let empty_type_pack = unsafe { (*self.builtin_types).empty_type_pack };
      self.add_constraint_scope_ptr_location_constraint_v(
        &scope,
        unsafe { (*block).base.base.location },
        ConstraintV::PackSubtype(PackSubtypeConstraint {
          sub_pack: empty_type_pack,
          super_pack: return_type,
          returns: false,
        }),
      );
    }

    let end = unsafe { checkpoint(self) };

    let result = unsafe { (*self.arena).add_type(BlockedType::default()) };
    let gen_constraint = self.add_constraint_scope_ptr_location_constraint_v(
      &scope,
      unsafe { (*block).base.base.location },
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type: result,
        source_type: module_fn_ty,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: true,
      }),
    );

    unsafe {
      (*self.root_scope).interior_free_types =
        Some(self.interior_free_types.last().unwrap().types.clone());
      (*self.root_scope).interior_free_type_packs =
        Some(self.interior_free_types.last().unwrap().type_packs.clone());
    }

    // result 刚由 add_type(BlockedType) 分配，必命中；对照 C++:422
    // `getMutable<BlockedType>(result)->setOwner(genConstraint)`
    let blocked = get_mutable::<BlockedType>(result).unwrap();
    blocked.set_owner(gen_constraint);

    if FFlag::LuauConstraintGraph.get() {
      unsafe { add_all_as_dependencies(start, end, self, gen_constraint) };
    } else {
      for_each_constraint(start, end, self, |c: *mut Constraint| unsafe {
        (*gen_constraint).deprecated_dependencies.push(c);
      });
    }

    self.interior_free_types.pop();

    self.fill_in_inferred_bindings(&scope, block);

    if !self.logger.is_null() {
      unsafe {
        (*self.logger).capture_generation_module(self.module.clone().unwrap());
      }
    }

    let local_types_pairs: Vec<(TypeId, Vec<TypeId>)> = self
      .local_types
      .iter()
      .map(|(ty, domain)| (*ty, domain.order.clone()))
      .collect();
    for (ty, domain) in local_types_pairs {
      // FIXME: This isn't the most efficient thing.
      let mut domain_ty = unsafe { (*self.builtin_types).never_type };
      for d in domain {
        let d_followed = follow_type_id(d);
        if d_followed == ty {
          continue;
        }
        domain_ty = self.simplify_union(scope.clone(), Location::default(), domain_ty, d_followed);
      }

      LUAU_ASSERT!(get_type_id::<BlockedType>(ty).is_some());
      unsafe {
        (*as_mutable_type_id(ty)).ty = TypeVariant::Bound(domain_ty);
      }
    }

    let unions_to_simplify = self.unions_to_simplify.clone();
    for ty in unions_to_simplify {
      self.add_constraint_scope_ptr_location_constraint_v(
        &scope,
        unsafe { (*block).base.base.location },
        ConstraintV::Simplify(SimplifyConstraint { ty }),
      );
    }
  }
}
