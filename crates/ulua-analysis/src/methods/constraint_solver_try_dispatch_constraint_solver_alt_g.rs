use core::{ffi::CStr, ptr::NonNull};

use ulua_ast::records::ast_name::AstName;
use ulua_common::FFlag;

use crate::{
  functions::{
    follow_type::follow_type_id, get_mutable_table_type::get_mutable_table_type,
    get_mutable_type::get_mutable_type_id, get_table_type::get_table_type,
    get_type_alt_j::get_type_id, occurs_check_type_utils::occurs_check_type_id_type_id,
    saturate_arguments::saturate_arguments, shallow_clone_clone_alt_b::shallow_clone,
  },
  records::{
    apply_type_function::ApplyTypeFunction,
    clone_state::CloneState,
    constraint::Constraint,
    constraint_solver::ConstraintSolver,
    generic_type_visitor::GenericTypeVisitorTrait,
    infinite_type_finder::InfiniteTypeFinder,
    instantiation_queuer::InstantiationQueuer,
    instantiation_queuer_deprecated::InstantiationQueuerDeprecated,
    instantiation_signature::InstantiationSignature,
    iterative_type_visitor::IterativeTypeVisitorTrait,
    metatable_type::MetatableType,
    occurs_check_failed::OccursCheckFailed,
    pending_expansion_type::PendingExpansionType,
    reduce_constraint::ReduceConstraint,
    table_type::TableType,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_function_instance_type::TypeFunctionInstanceType,
    unknown_symbol::{Context, UnknownSymbol},
  },
  type_aliases::{constraint_v::ConstraintV, type_error_data::TypeErrorData, type_id::TypeId},
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_type_alias_expansion_constraint_not_null_constraint(
    &mut self,
    c: &TypeAliasExpansionConstraint,
    constraint: *const Constraint,
  ) -> bool {
    let Some(petv) = get_type_id::<PendingExpansionType>(follow_type_id(c.target)) else {
      self.unblock_type_id_location(c.target, unsafe { (*constraint).location });
      return true;
    };

    let alias_name = ast_name_to_string(petv.name);
    let alias_prefix = petv.prefix.map(ast_name_to_string);
    let raw_type_arguments = petv.type_arguments.clone();
    let raw_pack_arguments = petv.pack_arguments.clone();

    let tf = unsafe {
      if let Some(prefix) = &alias_prefix {
        (*(*constraint).scope).lookup_imported_type(prefix, &alias_name)
      } else {
        (*(*constraint).scope).lookup_type(&alias_name)
      }
    };

    let Some(tf) = tf else {
      self.report_error_type_error_data_location(
        TypeErrorData::UnknownSymbol(UnknownSymbol::new(alias_name, Context::Type)),
        unsafe { &(*constraint).location },
      );
      bind_alias_expansion_result(self, c, constraint, unsafe {
        (*self.builtin_types).error_type
      });
      return true;
    };

    if get_type_id::<TypeFunctionInstanceType>(follow_type_id(tf.r#type())).is_some() {
      self.push_constraint(
        unsafe { NonNull::new_unchecked((*constraint).scope) },
        unsafe { (*constraint).location },
        ConstraintV::Reduce(ReduceConstraint { ty: tf.r#type() }),
      );
    }

    let lhs = follow_type_id(c.target);
    let rhs = tf.r#type();
    if occurs_check_type_id_type_id(lhs, rhs) {
      self.report_error_type_error_data_location(
        TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
        unsafe { &(*constraint).location },
      );
      bind_alias_expansion_result(self, c, constraint, unsafe {
        (*self.builtin_types).error_type
      });
      return true;
    }

    if tf.type_params().is_empty() && tf.type_pack_params().is_empty() {
      bind_alias_expansion_result(self, c, constraint, tf.r#type());
      return true;
    }

    let (type_arguments, pack_arguments) = unsafe {
      saturate_arguments(
        &mut *self.arena,
        &mut *self.builtin_types,
        &tf,
        &raw_type_arguments,
        &raw_pack_arguments,
      )
    };

    let same_types = type_arguments.len() == tf.type_params().len()
      && type_arguments
        .iter()
        .zip(tf.type_params())
        .all(|(arg, param)| *arg == param.ty);
    let same_packs = pack_arguments.len() == tf.type_pack_params().len()
      && pack_arguments
        .iter()
        .zip(tf.type_pack_params())
        .all(|(arg, param)| *arg == param.tp);

    if same_types && same_packs {
      bind_alias_expansion_result(self, c, constraint, tf.r#type());
      return true;
    }

    let signature = InstantiationSignature {
      fn_sig: tf.clone(),
      arguments: type_arguments.clone(),
      pack_arguments: pack_arguments.clone(),
    };

    if let Some(cached) = self.instantiated_aliases.find(&signature).copied() {
      bind_alias_expansion_result(self, c, constraint, cached);
      return true;
    }

    let mut itf =
      InfiniteTypeFinder::infinite_type_finder_infinite_type_finder(self, &signature, unsafe {
        NonNull::new_unchecked((*constraint).scope)
      });
    itf.run_type_id(tf.r#type());

    if itf.found_infinite_type {
      bind_alias_expansion_result(self, c, constraint, unsafe {
        (*self.builtin_types).error_type
      });
      unsafe {
        (*(*constraint).scope)
          .invalid_type_aliases
          .try_insert(alias_name.clone(), (*constraint).location);
      }
      return true;
    }

    let mut apply_type_function = ApplyTypeFunction::new(self.arena);
    for (i, ty) in type_arguments.iter().enumerate() {
      *apply_type_function
        .type_arguments
        .get_or_insert(tf.type_params()[i].ty) = *ty;
    }

    for (i, tp) in pack_arguments.iter().enumerate() {
      *apply_type_function
        .type_pack_arguments
        .get_or_insert(tf.type_pack_params()[i].tp) = *tp;
    }

    let Some(mut instantiated) = apply_type_function.substitute_type_id(tf.r#type()) else {
      bind_alias_expansion_result(self, c, constraint, unsafe {
        (*self.builtin_types).error_type
      });
      return true;
    };

    let mut target = follow_type_id(instantiated);

    if FFlag::LuauIterativeInstantiationQueuer.get() {
      let mut queuer = InstantiationQueuer::new(
        unsafe { NonNull::new_unchecked((*constraint).scope) },
        unsafe { &(*constraint).location },
        self as *mut ConstraintSolver,
      );
      queuer.run_type_id(target);
    } else {
      let mut queuer = InstantiationQueuerDeprecated::instantiation_queuer_deprecated_instantiation_queuer_deprecated(
                unsafe { NonNull::new_unchecked((*constraint).scope) },
                unsafe { &(*constraint).location },
                self as *mut ConstraintSolver,
            );
      queuer.traverse_type_id(target);
    }

    if unsafe { (*target).persistent || (*target).owning_arena != self.arena } {
      bind_alias_expansion_result(self, c, constraint, target);
      return true;
    }

    // 指针相等对应 C++ 的 tfTable == targetTable。
    let tf_table = get_table_type(tf.r#type()).map(|table| table as *const TableType);
    let target_table = get_table_type(target).map(|table| table as *const TableType);
    let needs_clone = follow_type_id(tf.r#type()) == target
      || (tf_table.is_some() && tf_table == target_table)
      || type_arguments.contains(&target);

    let mut table = get_mutable_table_type(target);
    if table.is_some() {
      if needs_clone {
        if get_type_id::<MetatableType>(target).is_some() {
          // SAFETY: builtin_types 在 solver 存活期内有效。
          let mut clone_state = unsafe { CloneState::new(&mut *self.builtin_types) };
          // SAFETY: arena 在 solver 存活期内有效。
          instantiated = unsafe { shallow_clone(target, &mut *self.arena, &mut clone_state, true) };
          let metatable = get_mutable_type_id::<MetatableType>(instantiated)
            .expect("shallow_clone 保留 MetatableType 变体");
          metatable.table = unsafe {
            shallow_clone(
              metatable.table(),
              // SAFETY: arena 在 solver 存活期内有效。
              &mut *self.arena,
              &mut clone_state,
              true,
            )
          };
          table = get_mutable_table_type(metatable.table());
        } else if get_type_id::<TableType>(target).is_some() {
          // SAFETY: builtin_types 在 solver 存活期内有效。
          let mut clone_state = unsafe { CloneState::new(&mut *self.builtin_types) };
          // SAFETY: arena 在 solver 存活期内有效。
          instantiated = unsafe { shallow_clone(target, &mut *self.arena, &mut clone_state, true) };
          table = get_mutable_table_type(instantiated);
        }

        target = follow_type_id(instantiated);
      }

      // C++ 对 table 直解引用；外层已确认 Some，重赋值来源同为 clone，必仍命中。
      let table = table.expect("target remains TableType");
      table.instantiated_type_params = type_arguments.clone();
      table.instantiated_type_pack_params = pack_arguments.clone();
      // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
      table.definition_location = unsafe { (*constraint).location };
      if let Some(module) = &self.module {
        table.definition_module_name = module.name.clone();
      }
    }

    bind_alias_expansion_result(self, c, constraint, target);
    self.instantiated_aliases.try_insert(signature, target);

    true
  }
}

fn bind_alias_expansion_result(
  solver: &mut ConstraintSolver,
  c: &TypeAliasExpansionConstraint,
  constraint: *const Constraint,
  result: TypeId,
) {
  let c_target = follow_type_id(c.target);

  if occurs_check_type_id_type_id(c_target, result) {
    solver.report_error_type_error_data_location(
      TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
      unsafe { &(*constraint).location },
    );
    unsafe {
      solver.bind_not_null_constraint_type_id_type_id(
        constraint,
        c_target,
        (*solver.builtin_types).error_type,
      )
    };
  } else {
    unsafe { solver.bind_not_null_constraint_type_id_type_id(constraint, c_target, result) };
  }
}

fn ast_name_to_string(name: AstName) -> String {
  if name.value.is_null() {
    String::new()
  } else {
    unsafe { CStr::from_ptr(name.value) }
      .to_string_lossy()
      .into_owned()
  }
}
