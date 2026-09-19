use alloc::{format, string::String, sync::Arc};
use core::{cmp::min, ptr::null_mut};

use ulua_common::fflag;

use crate::{
  functions::{
    finite::finite, get_mutable_type::get_mutable_type_id,
    has_unification_too_complex::has_unification_too_complex, size_type_pack::size,
  },
  records::{
    count_mismatch::CountMismatchContext, function_type::FunctionType,
    instantiation::Instantiation, type_error::TypeError, type_mismatch::TypeMismatch,
    unification_too_complex::UnificationTooComplex, unifier::Unifier,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};
impl Unifier {
  /// `void Unifier::tryUnifyFunctions(TypeId subTy, TypeId superTy, bool isFunctionCall)`
  pub fn unifier_try_unify_functions(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    is_function_call: bool,
  ) {
    // C++：`if (!superFunction || !subFunction) ice(...)`。一次 let-else 绑定取代
    // 其后 30 余处 `as_ref().unwrap()`，判定与 ICE 文案逐条对齐。
    let (Some(mut super_function), Some(mut sub_function)) = (
      get_mutable_type_id::<FunctionType>(super_ty),
      get_mutable_type_id::<FunctionType>(sub_ty),
    ) else {
      self.ice_string("passed non-function types to unifyFunction");
      return;
    };

    let mut num_generics = super_function.generics.len();
    let mut num_generic_packs = super_function.generic_packs.len();

    let should_instantiate = (num_generics == 0 && !sub_function.generics.is_empty())
      || (num_generic_packs == 0 && !sub_function.generic_packs.is_empty());

    if fflag::LuauInstantiateInSubtyping.get() && should_instantiate {
      let mut instantiation = Instantiation::instantiation_new(
        &self.log as *const _,
        self.types,
        self.builtin_types,
        unsafe { (*self.scope).level },
        self.scope,
      );

      if let Some(instantiated) = instantiation.substitute_type_id(sub_ty) {
        let Some(instantiated_function) = get_mutable_type_id::<FunctionType>(instantiated) else {
          self.ice_string(
            "instantiation made a function type into a non-function type in unifyFunction",
          );
          return;
        };
        sub_function = instantiated_function;

        num_generics = min(super_function.generics.len(), sub_function.generics.len());
        num_generic_packs = min(
          super_function.generic_packs.len(),
          sub_function.generic_packs.len(),
        );
      } else {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
        );
      }
    } else if num_generics != sub_function.generics.len() {
      num_generics = min(num_generics, sub_function.generics.len());
      self.report_function_type_mismatch(
        super_ty,
        sub_ty,
        "different number of generic type parameters",
        None,
      );
    }

    if num_generic_packs != sub_function.generic_packs.len() {
      num_generic_packs = min(num_generic_packs, sub_function.generic_packs.len());
      self.report_function_type_mismatch(
        super_ty,
        sub_ty,
        "different number of generic type pack parameters",
        None,
      );
    }

    for (&super_generic, &sub_generic) in super_function
      .generics
      .iter()
      .zip(sub_function.generics.iter())
      .take(num_generics)
    {
      self
        .log
        .push_seen_type_id_type_id(super_generic, sub_generic);
    }

    for (&super_pack, &sub_pack) in super_function
      .generic_packs
      .iter()
      .zip(sub_function.generic_packs.iter())
      .take(num_generic_packs)
    {
      self
        .log
        .push_seen_type_pack_id_type_pack_id(super_pack, sub_pack);
    }

    let context = self.ctx;

    if !is_function_call {
      let mut inner_state = self.unifier_make_child_unifier();

      inner_state.ctx = CountMismatchContext::Arg;
      inner_state.try_unify_type_pack_id_type_pack_id_bool(
        super_function.arg_types,
        sub_function.arg_types,
        is_function_call,
      );

      let reported = !inner_state.errors.is_empty();

      if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        self.report_error_type_error(e);
      } else if !inner_state.errors.is_empty()
        && let Some(first_pack_error_pos) = inner_state.first_pack_error_pos
      {
        let reason = format!("Argument #{} type is not compatible.", first_pack_error_pos);
        self.report_function_type_mismatch(
          super_ty,
          sub_ty,
          &reason,
          inner_state.errors.first().cloned(),
        );
      } else if !inner_state.errors.is_empty() {
        self.report_function_type_mismatch(
          super_ty,
          sub_ty,
          "",
          inner_state.errors.first().cloned(),
        );
      }

      inner_state.ctx = CountMismatchContext::FunctionResult;
      inner_state.try_unify_type_pack_id_type_pack_id_bool(
        sub_function.ret_types,
        super_function.ret_types,
        false,
      );

      if !reported {
        if let Some(e) = has_unification_too_complex(&inner_state.errors) {
          self.report_error_type_error(e);
        } else if !inner_state.errors.is_empty()
          && unsafe { size(super_function.ret_types, null_mut()) } == 1
          && unsafe { finite(super_function.ret_types, null_mut()) }
        {
          self.report_function_type_mismatch(
            super_ty,
            sub_ty,
            "Return type is not compatible.",
            inner_state.errors.first().cloned(),
          );
        } else if !inner_state.errors.is_empty() && inner_state.first_pack_error_pos.is_some() {
          let reason = format!(
            "Return #{} type is not compatible.",
            inner_state.first_pack_error_pos.unwrap()
          );
          self.report_function_type_mismatch(
            super_ty,
            sub_ty,
            &reason,
            inner_state.errors.first().cloned(),
          );
        } else if !inner_state.errors.is_empty() {
          self.report_function_type_mismatch(
            super_ty,
            sub_ty,
            "",
            inner_state.errors.first().cloned(),
          );
        }
      }

      self.log.concat(inner_state.log);
    } else {
      self.ctx = CountMismatchContext::Arg;
      self.try_unify_type_pack_id_type_pack_id_bool(
        super_function.arg_types,
        sub_function.arg_types,
        is_function_call,
      );

      self.ctx = CountMismatchContext::FunctionResult;
      self.try_unify_type_pack_id_type_pack_id_bool(
        sub_function.ret_types,
        super_function.ret_types,
        false,
      );
    }

    // unification 过程可能改变 sub/super 的绑定，C++ 在此重新 getMutable。
    // C++ 取回后不做 null 判定直接解引用，故 None 仍是「立即终止」的 panic，
    // 与原 `.as_ref().unwrap()` 行为一致。
    super_function = get_mutable_type_id::<FunctionType>(super_ty).unwrap();
    sub_function = get_mutable_type_id::<FunctionType>(sub_ty).unwrap();

    self.ctx = context;

    for i in (0..num_generic_packs).rev() {
      self.log.pop_seen_type_pack_id_type_pack_id(
        super_function.generic_packs[i],
        sub_function.generic_packs[i],
      );
    }

    for i in (0..num_generics).rev() {
      self
        .log
        .pop_seen_type_id_type_id(super_function.generics[i], sub_function.generics[i]);
    }
  }

  fn report_function_type_mismatch(
    &mut self,
    super_ty: TypeId,
    sub_ty: TypeId,
    reason: &str,
    err: Option<TypeError>,
  ) {
    let context = self.unifier_mismatch_context();
    self.report_error_location_type_error_data(
      self.location,
      TypeErrorData::TypeMismatch(TypeMismatch {
        wanted_type: super_ty,
        given_type: sub_ty,
        reason: String::from(reason),
        error: err.map(Arc::new),
        context,
      }),
    );
  }
}
