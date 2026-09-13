use core::{
  ffi::c_void,
  ptr::{NonNull, null_mut},
};

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap as CommonDenseHashMap;

use crate::{
  enums::polarity::Polarity,
  functions::{
    find_blocked_arg_types_in::find_blocked_arg_types_in, flatten_type_pack::flatten_type_pack_id,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id,
    get_type_alt_j::get_type_id, push_type_into::push_type_into, unwrap_group::unwrap_group,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver, constraint_v::ConstraintV,
    dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet,
    function_check_constraint::FunctionCheckConstraint, function_type::FunctionType,
    generic_type::GenericType, internal_error_reporter::InternalErrorReporter,
    push_type_constraint::PushTypeConstraint, scope::Scope, subtyping::Subtyping,
    unifier_2::Unifier2,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_function_check_constraint_not_null_constraint_bool(
    &mut self,
    c: &FunctionCheckConstraint,
    constraint: *const Constraint,
    force: bool,
  ) -> bool {
    let fn_ty = follow_type_id(c.fn_type);
    let args_pack = unsafe { follow_type_pack_id(c.args_pack) };

    if self.is_blocked_type_id(fn_ty) {
      return self.block_type_id_not_null_constraint(fn_ty, constraint);
    }

    if self.is_blocked_type_pack_id(args_pack) {
      return true;
    }

    let blocked_types = unsafe {
      find_blocked_arg_types_in(
        c.call_site,
        c.ast_types as *mut CommonDenseHashMap<*const AstExpr, TypeId>,
      )
    };
    for ty in &blocked_types {
      self.block_type_id_not_null_constraint(*ty, constraint);
    }
    if !blocked_types.is_empty() {
      return false;
    }

    let Some(ftv) = get_type_id::<FunctionType>(fn_ty) else {
      return true;
    };

    let mut replacements: DenseHashMap<TypeId, TypeId> = DenseHashMap::new(null_mut());
    let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> = DenseHashMap::new(null_mut());

    let mut generic_types_and_packs: DenseHashSet<*const c_void> = DenseHashSet::new(null_mut());

    let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
            NonNull::new(self.arena).unwrap(),
            NonNull::new(self.builtin_types).unwrap(),
            NonNull::new(constraint_scope(constraint)).unwrap(),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).unwrap(),
        );

    for generic in &ftv.generics {
      // We may see non-generic types here, for example when evaluating a
      // recursive function call.
      if let Some(gty) = get_type_id::<GenericType>(follow_type_id(*generic)) {
        let repl_ty = if gty.polarity == Polarity::Negative {
          unsafe { (*self.builtin_types).never_type }
        } else {
          unsafe { (*self.builtin_types).unknown_type }
        };
        replacements.try_insert(*generic, repl_ty);
        generic_types_and_packs.insert_mut(*generic as *const c_void);
      }
    }

    for generic_pack in &ftv.generic_packs {
      replacement_packs.try_insert(*generic_pack, unsafe {
        (*self.builtin_types).unknown_type_pack
      });
      generic_types_and_packs.insert_mut(*generic_pack as *const c_void);
    }

    let (expected_args, _) = flatten_type_pack_id(ftv.arg_types);
    let (arg_pack_head, _) = flatten_type_pack_id(args_pack);

    // If this is a self call, the types will have more elements than the AST call.
    // We don't attempt to perform bidirectional inference on the self type.
    let type_offset = if unsafe { (*c.call_site).self_ } {
      1
    } else {
      0
    };

    let mut subtyping = Subtyping::subtyping_owned(
      self.builtin_types,
      self.arena,
      self.normalizer,
      self.type_function_runtime,
      &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter,
    );

    let call_site_args_size = unsafe { (*c.call_site).args.size };
    let args_data = unsafe { (*c.call_site).args.data };

    for i in 0..call_site_args_size {
      if i + type_offset >= expected_args.len() || i + type_offset >= arg_pack_head.len() {
        break;
      }

      let expected_arg_ty = follow_type_id(expected_args[i + type_offset]);
      let expr = unsafe { unwrap_group(*args_data.add(i)) };

      let result = push_type_into(
        NonNull::new(c.ast_types as *mut CommonDenseHashMap<*const AstExpr, TypeId>).unwrap(),
        NonNull::new(c.ast_expected_types as *mut CommonDenseHashMap<*const AstExpr, TypeId>)
          .unwrap(),
        NonNull::new(self as *mut ConstraintSolver).unwrap(),
        NonNull::new(constraint as *mut Constraint).unwrap(),
        NonNull::new(&mut generic_types_and_packs as *mut DenseHashSet<*const c_void>).unwrap(),
        NonNull::new(&mut u2 as *mut Unifier2).unwrap(),
        NonNull::new(&mut subtyping as *mut Subtyping).unwrap(),
        expected_arg_ty,
        expr as *const AstExpr,
      );

      if !force && !result.incomplete_types.is_empty() {
        for incomplete in &result.incomplete_types {
          let addition = self.push_constraint(
            NonNull::new(constraint_scope(constraint)).unwrap(),
            unsafe { (*constraint).location },
            ConstraintV::PushType(PushTypeConstraint {
              expected_type: incomplete.expected_type,
              target_type: incomplete.target_type,
              ast_types: c.ast_types,
              ast_expected_types: c.ast_expected_types,
              expr: incomplete.expr,
            }),
          );
          self.inherit_blocks(constraint, addition.as_ptr());
        }
      }
    }

    let incomplete_subtypes = u2.incomplete_subtypes.clone();
    for c_item in incomplete_subtypes {
      let addition = self.push_constraint(
        NonNull::new(constraint_scope(constraint)).unwrap(),
        unsafe { (*constraint).location },
        c_item,
      );
      self.inherit_blocks(constraint, addition.as_ptr());
    }

    true
  }
}

fn constraint_scope(c: *const Constraint) -> *mut Scope {
  unsafe { (*c).scope }
}
