use alloc::{string::String, vec::Vec};
use core::{cmp::min, ffi::CStr, ptr::eq};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_table::AstExprTable, ast_local::AstLocal,
    ast_node::AstNode, ast_stat_local::AstStatLocal,
  },
  rtti::ast_node_as,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  functions::{
    add_all_as_dependencies::add_all_as_dependencies, checkpoint::checkpoint,
    flatten_type_pack::flatten_type_pack_id, for_each_constraint::for_each_constraint,
    get_mutable_type::get_mutable, match_require::match_require,
    match_set_metatable::match_set_metatable,
  },
  records::{
    binding::Binding,
    blocked_type::BlockedType,
    constraint::Constraint,
    constraint_generator::{ConstraintGenerator, InferredBinding},
    name_constraint::NameConstraint,
    pack_subtype_constraint::PackSubtypeConstraint,
    scope::Scope,
    symbol::Symbol,
    type_fun::TypeFun,
    type_ids::TypeIds,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_local(
    &mut self,
    scope: &ScopePtr,
    stat_local: *mut AstStatLocal,
  ) -> ControlFlow {
    let stat_local_ref = unsafe { &*stat_local };
    let scope_raw = scope.as_ref() as *const Scope as *mut Scope;

    let mut annotated_types = Vec::with_capacity(stat_local_ref.vars.size);
    let mut has_annotation = false;

    let mut expected_types = Vec::with_capacity(stat_local_ref.vars.size);

    let mut assignees = Vec::with_capacity(stat_local_ref.vars.size);

    let mut first_value_type = None;

    for i in 0..stat_local_ref.vars.size {
      let local: *mut AstLocal = unsafe { *stat_local_ref.vars.data.add(i) };
      let location = unsafe { (*local).location };

      let assignee = unsafe { (*self.arena).add_type(BlockedType::default()) };
      self.local_types.try_insert(assignee, TypeIds::new());
      assignees.push(assignee);

      if first_value_type.is_none() {
        first_value_type = Some(assignee);
      }

      if !unsafe { (*local).annotation }.is_null() {
        has_annotation = true;
        let annotation_ty = self.resolve_type(
          scope_raw,
          unsafe { (*local).annotation },
          false,
          false,
          Polarity::Positive,
        );
        annotated_types.push(annotation_ty);
        expected_types.push(Some(annotation_ty));
        unsafe {
          (*scope_raw).bindings.insert(
            Symbol::from_local(local),
            Binding {
              type_id: annotation_ty,
              location,
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            },
          );
        }
      } else {
        annotated_types.push(unsafe { (*self.builtin_types).unknown_type });
        expected_types.push(None);
        unsafe {
          (*scope_raw).bindings.insert(
            Symbol::from_local(local),
            Binding {
              type_id: (*self.builtin_types).unknown_type,
              location,
              deprecated: false,
              deprecated_suggestion: String::new(),
              documentation_symbol: None,
            },
          );
        }

        let mut types = TypeIds::new();
        types.insert_type_id(assignee);
        self.inferred_bindings.try_insert(
          Symbol::from_local(local),
          InferredBinding {
            scope: scope_raw,
            location,
            types,
          },
        );
      }

      let def = unsafe { (*self.dfg).get_def_local(local) };
      unsafe {
        *(*scope_raw).lvalue_types.get_or_insert(def) = assignee;
      }
    }

    let start = unsafe { checkpoint(self as *const ConstraintGenerator) };
    let rvalue_pack = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
        scope,
        stat_local_ref.values,
        &expected_types,
      )
      .tp;
    let end = unsafe { checkpoint(self as *const ConstraintGenerator) };

    let mut deferred_types = Vec::new();
    let (head, tail) = flatten_type_pack_id(rvalue_pack);
    let mut fresh_blocked_types: Vec<&'static mut BlockedType> = Vec::new();

    for i in 0..stat_local_ref.vars.size {
      LUAU_ASSERT!(get_mutable::<BlockedType>(assignees[i]).is_some());
      let local_domain = self
        .local_types
        .find_mut(&assignees[i])
        .expect("local assignee domain should exist");

      let local = unsafe { *stat_local_ref.vars.data.add(i) };
      if !unsafe { (*local).annotation }.is_null() {
        local_domain.insert_type_id(annotated_types[i]);
        if i >= head.len() && tail.is_some() {
          deferred_types.push(annotated_types[i]);
        }
      } else if i < head.len() {
        local_domain.insert_type_id(head[i]);
      } else if tail.is_some() {
        let deferred = unsafe { (*self.arena).add_type(BlockedType::default()) };
        deferred_types.push(deferred);
        local_domain.insert_type_id(deferred);
        // deferred 刚由 add_type(BlockedType) 分配，必命中；对照 C++:1509
        fresh_blocked_types.push(get_mutable::<BlockedType>(deferred).unwrap());
      } else {
        local_domain.insert_type_id(unsafe { (*self.builtin_types).nil_type });
      }
    }

    if has_annotation {
      let annotated_pack = unsafe {
        (*self.arena).add_type_pack_vector_type_id_optional_type_pack_id(annotated_types, None)
      };
      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        stat_local_ref.base.base.location,
        ConstraintV::PackSubtype(PackSubtypeConstraint {
          sub_pack: rvalue_pack,
          super_pack: annotated_pack,
          returns: false,
        }),
      );
    }

    if !deferred_types.is_empty() {
      LUAU_ASSERT!(tail.is_some());
      let uc = self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        stat_local_ref.base.base.location,
        ConstraintV::Unpack(UnpackConstraint {
          result_pack: deferred_types,
          source_pack: tail.unwrap(),
        }),
      );

      if FFlag::LuauConstraintGraph.get() {
        unsafe { add_all_as_dependencies(start, end, self, uc) };
      } else {
        for_each_constraint(start, end, self, |run_before: *mut Constraint| unsafe {
          (*uc).deprecated_dependencies.push(run_before);
        });
      }

      // 对照 C++:1546 `for (BlockedType* bt : freshBlockedTypes) bt->setOwner(uc);`
      for bt in fresh_blocked_types {
        bt.set_owner(uc);
      }
    }

    if stat_local_ref.vars.size == 1
      && stat_local_ref.values.size == 1
      && let Some(first_value_type) = first_value_type
      && eq(scope_raw, self.root_scope)
      && !has_annotation
    {
      let var = unsafe { *stat_local_ref.vars.data };
      let value = unsafe { *stat_local_ref.values.data };
      let should_name = unsafe {
        let node = value as *mut AstNode;
        (*node).is::<AstExprTable>()
          || ast_node_as::<AstExprCall>(node)
            .as_ref()
            .is_some_and(match_set_metatable)
      };

      if should_name {
        let name = unsafe {
          CStr::from_ptr((*var).name.value)
            .to_string_lossy()
            .into_owned()
        };
        self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          unsafe { (*value).base.location },
          ConstraintV::Name(NameConstraint {
            named_type: first_value_type,
            name,
            synthetic: true,
            type_parameters: Vec::new(),
            type_pack_parameters: Vec::new(),
          }),
        );
      }
    }

    if stat_local_ref.values.size > 0 {
      for i in 0..min(stat_local_ref.values.size, stat_local_ref.vars.size) {
        let value = unsafe { *stat_local_ref.values.data.add(i) };
        let call = unsafe { ast_node_as::<AstExprCall>(value as *mut AstNode).as_ref() };
        let Some(call) = call else {
          continue;
        };
        let Some(require) = match_require(call) else {
          continue;
        };

        let module_name = self.module.as_ref().unwrap().name.clone();
        let module_info = unsafe {
          ((*self.module_resolver).vtable.resolve_module_info)(
            self.module_resolver,
            &module_name,
            require as *const _,
          )
        };
        let Some(module_info) = module_info else {
          continue;
        };

        let required_module = unsafe {
          ((*self.module_resolver).vtable.get_module)(self.module_resolver, &module_info.name)
        };
        let Some(required_module) = required_module else {
          continue;
        };

        let local = unsafe { *stat_local_ref.vars.data.add(i) };
        let name = unsafe {
          CStr::from_ptr((*local).name.value)
            .to_string_lossy()
            .into_owned()
        };
        unsafe {
          (*scope_raw)
            .imported_type_bindings
            .insert(name.clone(), required_module.exported_type_bindings.clone());
          (*scope_raw)
            .imported_modules
            .insert(name.clone(), module_info.name.clone());
        }

        for cycle in &self.require_cycles {
          if cycle.path.is_empty() || cycle.path[0] != module_info.name {
            continue;
          }

          unsafe {
            if let Some(bindings) = (*scope_raw).imported_type_bindings.get_mut(&name) {
              for tf in bindings.values_mut() {
                *tf = TypeFun {
                  type_params: Vec::new(),
                  type_pack_params: Vec::new(),
                  r#type: (*self.builtin_types).any_type,
                  definition_location: None,
                };
              }
            }
          }
        }
      }
    }

    ControlFlow::None
  }
}
