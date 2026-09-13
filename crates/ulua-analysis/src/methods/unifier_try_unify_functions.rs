use alloc::{format, string::String, sync::Arc};
use core::{cmp::min, ptr::null_mut};

use ulua_common::FFlag;

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
    let mut super_function = get_mutable_type_id::<FunctionType>(super_ty);
    let mut sub_function = get_mutable_type_id::<FunctionType>(sub_ty);

    if super_function.is_none() || sub_function.is_none() {
      self.ice_string("passed non-function types to unifyFunction");
      return;
    }

    let mut num_generics = super_function.as_ref().unwrap().generics.len();
    let mut num_generic_packs = super_function.as_ref().unwrap().generic_packs.len();

    let should_instantiate = (num_generics == 0
      && !sub_function.as_ref().unwrap().generics.is_empty())
      || (num_generic_packs == 0 && !sub_function.as_ref().unwrap().generic_packs.is_empty());

    if FFlag::LuauInstantiateInSubtyping.get() && should_instantiate {
      let mut instantiation = Instantiation::instantiation_new(
        &self.log as *const _,
        self.types,
        self.builtin_types,
        unsafe { (*self.scope).level },
        self.scope,
      );

      if let Some(instantiated) = instantiation.substitute_type_id(sub_ty) {
        sub_function = get_mutable_type_id::<FunctionType>(instantiated);
        if sub_function.is_none() {
          self.ice_string(
            "instantiation made a function type into a non-function type in unifyFunction",
          );
          return;
        }

        num_generics = min(
          super_function.as_ref().unwrap().generics.len(),
          sub_function.as_ref().unwrap().generics.len(),
        );
        num_generic_packs = min(
          super_function.as_ref().unwrap().generic_packs.len(),
          sub_function.as_ref().unwrap().generic_packs.len(),
        );
      } else {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
        );
      }
    } else if num_generics != sub_function.as_ref().unwrap().generics.len() {
      num_generics = min(num_generics, sub_function.as_ref().unwrap().generics.len());
      self.report_function_type_mismatch(
        super_ty,
        sub_ty,
        "different number of generic type parameters",
        None,
      );
    }

    if num_generic_packs != sub_function.as_ref().unwrap().generic_packs.len() {
      num_generic_packs = min(
        num_generic_packs,
        sub_function.as_ref().unwrap().generic_packs.len(),
      );
      self.report_function_type_mismatch(
        super_ty,
        sub_ty,
        "different number of generic type pack parameters",
        None,
      );
    }

    for i in 0..num_generics {
      let super_generics = &super_function.as_ref().unwrap().generics;
      let sub_generics = &sub_function.as_ref().unwrap().generics;
      self
        .log
        .push_seen_type_id_type_id(super_generics[i], sub_generics[i]);
    }

    for i in 0..num_generic_packs {
      let super_generic_packs = &super_function.as_ref().unwrap().generic_packs;
      let sub_generic_packs = &sub_function.as_ref().unwrap().generic_packs;
      self
        .log
        .push_seen_type_pack_id_type_pack_id(super_generic_packs[i], sub_generic_packs[i]);
    }

    let context = self.ctx;

    if !is_function_call {
      let mut inner_state = self.unifier_make_child_unifier();

      inner_state.ctx = CountMismatchContext::Arg;
      inner_state.try_unify_type_pack_id_type_pack_id_bool(
        super_function.as_ref().unwrap().arg_types,
        sub_function.as_ref().unwrap().arg_types,
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
        sub_function.as_ref().unwrap().ret_types,
        super_function.as_ref().unwrap().ret_types,
        false,
      );

      if !reported {
        if let Some(e) = has_unification_too_complex(&inner_state.errors) {
          self.report_error_type_error(e);
        } else if !inner_state.errors.is_empty()
          && unsafe { size(super_function.as_ref().unwrap().ret_types, null_mut()) } == 1
          && unsafe { finite(super_function.as_ref().unwrap().ret_types, null_mut()) }
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
        super_function.as_ref().unwrap().arg_types,
        sub_function.as_ref().unwrap().arg_types,
        is_function_call,
      );

      self.ctx = CountMismatchContext::FunctionResult;
      self.try_unify_type_pack_id_type_pack_id_bool(
        sub_function.as_ref().unwrap().ret_types,
        super_function.as_ref().unwrap().ret_types,
        false,
      );
    }

    // unification 过程可能改变 sub/super 的绑定，C++ 在此重新 getMutable
    super_function = get_mutable_type_id::<FunctionType>(super_ty);
    sub_function = get_mutable_type_id::<FunctionType>(sub_ty);

    self.ctx = context;

    for i in (0..num_generic_packs).rev() {
      let super_generic_packs = &super_function.as_ref().unwrap().generic_packs;
      let sub_generic_packs = &sub_function.as_ref().unwrap().generic_packs;
      self
        .log
        .pop_seen_type_pack_id_type_pack_id(super_generic_packs[i], sub_generic_packs[i]);
    }

    for i in (0..num_generics).rev() {
      let super_generics = &super_function.as_ref().unwrap().generics;
      let sub_generics = &sub_function.as_ref().unwrap().generics;
      self
        .log
        .pop_seen_type_id_type_id(super_generics[i], sub_generics[i]);
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
