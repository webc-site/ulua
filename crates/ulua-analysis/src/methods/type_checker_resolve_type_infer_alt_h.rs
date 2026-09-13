/// `kTypeofRootTag` (Type.h:1257).
use alloc::boxed::Box;

use crate::{
  functions::{
    add_refinement::add_refinement, follow_type::follow_type_id, get_type_alt_j::get_type_id,
    has_tag_type_alt_b::has_tag, is_boolean::is_boolean, is_buffer::is_buffer,
    is_integer::is_integer, is_nil::is_nil, is_number::is_number,
    is_overloaded_function::is_overloaded_function, is_string::is_string,
    is_table_intersection::is_table_intersection, is_thread::is_thread,
    is_undecidable::is_undecidable,
  },
  records::{
    extern_type::ExternType, function_type::FunctionType, is_a_predicate::IsAPredicate,
    metatable_type::MetatableType, table_type::TableType, type_checker::TypeChecker,
    type_guard_predicate::TypeGuardPredicate, unknown_type::UnknownType,
  },
  type_aliases::{
    l_value::LValue, refinement_map::RefinementMap, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_id_predicate::TypeIdPredicate,
  },
};
const K_TYPEOF_ROOT_TAG: &str = "typeofRoot";

fn is_table_like(ty: TypeId) -> bool {
  is_table_intersection(ty)
    || get_type_id::<TableType>(ty).is_some()
    || get_type_id::<MetatableType>(ty).is_some()
}

fn is_function_like(ty: TypeId) -> bool {
  is_overloaded_function(ty) || get_type_id::<FunctionType>(ty).is_some()
}

fn is_userdata_like(ty: TypeId) -> bool {
  get_type_id::<ExternType>(ty).is_some()
}

impl TypeChecker {
  /// C++ helper lambda `refine` inside `resolve(const TypeGuardPredicate&, ...)`.
  fn type_guard_refine(
    &mut self,
    lvalue: &LValue,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
    f: fn(TypeId) -> bool,
    maps_to: Option<TypeId>,
  ) {
    let predicate: TypeIdPredicate = Box::new(move |ty: TypeId| -> Option<TypeId> {
      if sense && get_type_id::<UnknownType>(ty).is_some() {
        return maps_to.or(Some(ty));
      }

      if f(ty) == sense {
        return Some(ty);
      }

      if is_undecidable(ty) {
        return maps_to.or(Some(ty));
      }

      None
    });

    self.refine_l_value(lvalue, refis, scope, predicate);
  }

  pub fn resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    typeguard_p: &TypeGuardPredicate,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
  ) {
    // Rewrite the predicate 'type(foo) == "vector"' to be 'typeof(foo) == "Vector3"'.
    // They're exactly identical.
    if !typeguard_p.is_typeof && typeguard_p.kind == "vector" {
      return self.resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
        &TypeGuardPredicate {
          lvalue: typeguard_p.lvalue.clone(),
          location: typeguard_p.location,
          kind: "Vector3".to_string(),
          is_typeof: true,
        },
        refis,
        scope,
        sense,
      );
    }

    let ty = self.resolve_l_value_refinement_map_scope_ptr_l_value(
      refis,
      scope.clone(),
      &typeguard_p.lvalue,
    );
    if ty.is_none() {
      return;
    }

    // In certain cases, the value may actually be nil, but Luau doesn't know about it.
    // So we whitelist this.
    if sense && typeguard_p.kind == "nil" {
      add_refinement(refis, &typeguard_p.lvalue, self.nil_type);
      return;
    }

    // Note: "vector" never happens here at this point.
    let kind = typeguard_p.kind.as_str();
    if kind == "nil" {
      // This can still happen when sense is false!
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_nil,
        Some(self.nil_type),
      );
    } else if kind == "string" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_string,
        Some(self.string_type),
      );
    } else if kind == "number" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_number,
        Some(self.number_type),
      );
    } else if kind == "integer" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_integer,
        Some(self.integer_type),
      );
    } else if kind == "boolean" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_boolean,
        Some(self.boolean_type),
      );
    } else if kind == "thread" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_thread,
        Some(self.thread_type),
      );
    } else if kind == "buffer" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_buffer,
        Some(self.buffer_type),
      );
    } else if kind == "table" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_table_like,
        None,
      );
    } else if kind == "function" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_function_like,
        None,
      );
    } else if kind == "userdata" {
      return self.type_guard_refine(
        &typeguard_p.lvalue,
        refis,
        scope,
        sense,
        is_userdata_like,
        None,
      );
    }

    if !typeguard_p.is_typeof {
      let err = self.error_recovery_type_scope_ptr(&scope);
      add_refinement(refis, &typeguard_p.lvalue, err);
      return;
    }

    let global_scope = unsafe { &**self.global_scope };
    let type_fun = global_scope.lookup_type(&typeguard_p.kind);
    let type_fun = match type_fun {
      Some(tf) if tf.type_params().is_empty() && tf.type_pack_params().is_empty() => tf,
      _ => {
        let err = self.error_recovery_type_scope_ptr(&scope);
        add_refinement(refis, &typeguard_p.lvalue, err);
        return;
      }
    };

    let resolved = follow_type_id(type_fun.r#type());
    let extern_type_builtin = unsafe { (*self.builtin_types).extern_type };

    // You cannot refine to the top class type.
    if resolved == extern_type_builtin {
      let err = self.error_recovery_type_scope_ptr(&scope);
      add_refinement(refis, &typeguard_p.lvalue, err);
      return;
    }

    // We're only interested in the root type of any extern type.
    let is_root_extern_type = get_type_id::<ExternType>(resolved).is_some_and(|etv| {
      etv.parent == Some(extern_type_builtin) || has_tag(resolved, K_TYPEOF_ROOT_TAG)
    });
    if !is_root_extern_type {
      let err = self.error_recovery_type_scope_ptr(&scope);
      add_refinement(refis, &typeguard_p.lvalue, err);
      return;
    }

    // Until type filtering functions are broken out, we rewrite this to be the same as using IsA.
    self.resolve_is_a_predicate_refinement_map_scope_ptr_bool(
      &IsAPredicate {
        lvalue: typeguard_p.lvalue.clone(),
        location: typeguard_p.location,
        ty: resolved,
      },
      refis,
      scope,
      sense,
    );
  }
}
