// ConstraintGenerator::visit(const ScopePtr&, AstStatForIn*) (ConstraintGenerator.cpp:1587-1691).
use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_ast::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_node::AstNode, ast_stat_for_in::AstStatForIn,
  location::Location,
};
use ulua_common::{FFlag, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  functions::{
    add_all_as_reverse_dependencies::add_all_as_reverse_dependencies, checkpoint::checkpoint,
    for_each_constraint::for_each_constraint, get_mutable_type::get_mutable,
  },
  records::{
    binding::Binding, blocked_type::BlockedType, constraint::Constraint,
    constraint_generator::ConstraintGenerator, iterable_constraint::IterableConstraint,
    module::Module, reduce_constraint::ReduceConstraint, scope::Scope,
    subtype_constraint::SubtypeConstraint, symbol::Symbol, type_function::TypeFunction,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId, type_pack_id::TypePackId,
  },
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_for_in(
    &mut self,
    scope: &ScopePtr,
    for_in: *mut AstStatForIn,
  ) -> ControlFlow {
    let for_in_ref = unsafe { &*for_in };

    let loop_scope: ScopePtr = unsafe {
      self.child_scope(
        &for_in_ref.base.base as *const AstNode as *mut AstNode,
        scope,
      )
    };
    let loop_scope_raw = loop_scope.as_ref() as *const Scope as *mut Scope;

    let values: AstArray<*mut AstExpr> = for_in_ref.values;
    let iterator: TypePackId = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(scope, values, &Vec::new())
      .tp;

    let mut variable_types: Vec<TypeId> = Vec::with_capacity(for_in_ref.vars.size);

    for &var in for_in_ref.vars.as_slice() {
      let loop_var: TypeId = unsafe { (*self.arena).add_type(BlockedType::default()) };
      variable_types.push(loop_var);

      if FFlag::LuauPropagateTypeAnnotationsInForInLoops.get() {
        let def = unsafe { (*self.dfg).get_def_local(var) };

        if !unsafe { (*var).annotation }.is_null() {
          let annotation_ty = self.resolve_type(
            loop_scope.as_ref() as *const Scope as *mut Scope,
            unsafe { (*var).annotation },
            /* in_type_arguments */ false,
            /* replace_error_with_fresh */ false,
            Polarity::Positive,
          );
          unsafe {
            (*loop_scope_raw).bindings.insert(
              Symbol::from_local(var),
              Binding {
                type_id: annotation_ty,
                location: (*var).location,
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
              },
            );
          }
          self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            unsafe { (*var).location },
            ConstraintV::Subtype(SubtypeConstraint {
              sub_type: loop_var,
              super_type: annotation_ty,
            }),
          );
          unsafe {
            *(*loop_scope_raw).lvalue_types.get_or_insert(def) = annotation_ty;
          }
        } else {
          unsafe {
            (*loop_scope_raw).bindings.insert(
              Symbol::from_local(var),
              Binding {
                type_id: loop_var,
                location: (*var).location,
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
              },
            );
            *(*loop_scope_raw).lvalue_types.get_or_insert(def) = loop_var;
          }
        }
      } else {
        if !unsafe { (*var).annotation }.is_null() {
          let annotation_ty = self.resolve_type(
            loop_scope.as_ref() as *const Scope as *mut Scope,
            unsafe { (*var).annotation },
            /* in_type_arguments */ false,
            /* replace_error_with_fresh */ false,
            Polarity::Positive,
          );
          unsafe {
            (*loop_scope_raw).bindings.insert(
              Symbol::from_local(var),
              Binding {
                type_id: annotation_ty,
                location: (*var).location,
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
              },
            );
          }
          self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            unsafe { (*var).location },
            ConstraintV::Subtype(SubtypeConstraint {
              sub_type: loop_var,
              super_type: annotation_ty,
            }),
          );
        } else {
          unsafe {
            (*loop_scope_raw).bindings.insert(
              Symbol::from_local(var),
              Binding {
                type_id: loop_var,
                location: (*var).location,
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
              },
            );
          }
        }

        let def = unsafe { (*self.dfg).get_def_local(var) };
        unsafe {
          *(*loop_scope_raw).lvalue_types.get_or_insert(def) = loop_var;
        }
      }
    }

    let next_ast_fragment = unsafe { *values.data.add(0) } as *const AstNode;
    let ast_for_in_next_types: *mut DenseHashMap<*const AstNode, TypeId> = {
      let module = self.module.as_ref().unwrap();
      let module_ptr = Arc::as_ptr(module) as *mut Module;
      unsafe { &mut (*module_ptr).ast_for_in_next_types as *mut _ }
    };

    // C++ getLocation(forIn->values): span from first expr begin to last expr end.
    let values_location = {
      let first = unsafe { *values.data.add(0) };
      let last = unsafe { *values.data.add(values.size - 1) };
      Location {
        begin: unsafe { (*first).base.location.begin },
        end: unsafe { (*last).base.location.end },
      }
    };

    let iterable: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
      &loop_scope,
      values_location,
      ConstraintV::Iterable(IterableConstraint {
        iterator,
        variables: variable_types.clone(),
        next_ast_fragment,
        ast_for_in_next_types,
      }),
    );

    // Add an intersection ReduceConstraint for the key variable to denote that it can't be nil
    let key_var = unsafe { *for_in_ref.vars.data.add(0) };
    let key_def = unsafe { (*self.dfg).get_def_local(key_var) };
    let loop_var: TypeId = unsafe { *(*loop_scope_raw).lvalue_types.get_or_insert(key_def) };

    let intersection_ty: TypeId = {
      let intersect_func: &TypeFunction =
        unsafe { &(*self.builtin_types).type_functions.intersect_func };
      let not_nil_ty = unsafe { (*self.builtin_types).not_nil_type };
      self.create_type_function_instance(
        intersect_func,
        alloc::vec![loop_var, not_nil_ty],
        Vec::new(),
        &loop_scope,
        unsafe { (*key_var).location },
      )
    };

    unsafe {
      (*loop_scope_raw).bindings.insert(
        Symbol::from_local(key_var),
        Binding {
          type_id: intersection_ty,
          location: (*key_var).location,
          deprecated: false,
          deprecated_suggestion: String::new(),
          documentation_symbol: None,
        },
      );
      *(*loop_scope_raw).lvalue_types.get_or_insert(key_def) = intersection_ty;
    }

    let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
      &loop_scope,
      unsafe { (*key_var).location },
      ConstraintV::Reduce(ReduceConstraint {
        ty: intersection_ty,
      }),
    );
    if FFlag::LuauConstraintGraph.get() {
      unsafe { (*self.cgraph).add_dependency_of_constraint_constraint(&mut *iterable, &mut *c) };
    } else {
      unsafe { (*c).deprecated_dependencies.push(iterable) };
    }

    for var in &variable_types {
      // variable_types 均来自 add_type(BlockedType)，必命中；对照 C++:1700-1704
      // `BlockedType* bt = getMutable<BlockedType>(var); LUAU_ASSERT(bt); bt->setOwner(iterable);`
      let bt = get_mutable::<BlockedType>(*var).expect("variableTypes 元素应为 BlockedType");
      bt.set_owner(iterable as *const _);
    }

    let start = unsafe { checkpoint(self as *const _) };
    self.visit_scope_ptr_ast_stat_block(&loop_scope, for_in_ref.body);
    let end = unsafe { checkpoint(self as *const _) };

    unsafe {
      (*(scope.as_ref() as *const Scope as *mut Scope)).inherit_assignments(&loop_scope);
    }

    // This iter constraint must dispatch first.
    if FFlag::LuauConstraintGraph.get() {
      unsafe { add_all_as_reverse_dependencies(start, end, self, iterable) };
    } else {
      for_each_constraint(start, end, self, |run_later: *mut Constraint| {
        unsafe { (*run_later).deprecated_dependencies.push(iterable) };
      });
    }

    ControlFlow::None
  }
}
