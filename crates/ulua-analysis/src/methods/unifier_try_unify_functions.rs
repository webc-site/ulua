use alloc::format;
use core::{cmp::min, ptr::null_mut};

use ulua_common::fflag;

use crate::{
  functions::{
    finite::finite, get_mutable_type, has_unification_too_complex::has_unification_too_complex,
    size_type_pack::size,
  },
  records::{
    count_mismatch::CountMismatchContext, function_type::FunctionType,
    instantiation::Instantiation, unification_too_complex::UnificationTooComplex, unifier::Unifier,
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
      get_mutable_type::get_mutable::<FunctionType>(super_ty),
      get_mutable_type::get_mutable::<FunctionType>(sub_ty),
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
        Some(self.types),
        self.builtin_types,
        self.scope_ref().level,
        self.scope.as_ptr(),
      );

      if let Some(instantiated) = instantiation.substitute_type_id(sub_ty) {
        let Some(instantiated_function) =
          get_mutable_type::get_mutable::<FunctionType>(instantiated)
        else {
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
      self.unifier_report_type_mismatch_ext(
        super_ty,
        sub_ty,
        "different number of generic type parameters".into(),
        None,
      );
    }

    if num_generic_packs != sub_function.generic_packs.len() {
      num_generic_packs = min(num_generic_packs, sub_function.generic_packs.len());
      self.unifier_report_type_mismatch_ext(
        super_ty,
        sub_ty,
        "different number of generic type pack parameters".into(),
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
        self.unifier_report_type_mismatch_ext(
          super_ty,
          sub_ty,
          reason.clone(),
          inner_state.errors.first().cloned(),
        );
      } else if !inner_state.errors.is_empty() {
        self.unifier_report_type_mismatch_ext(
          super_ty,
          sub_ty,
          "".into(),
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
          // Safety: `size`/`finite` 为 `unsafe fn(TypePackId, *mut TxnLog)`；
          // `super_function.ret_types` 是存活 arena TypePackId 句柄（函数类型字段），
          // `null_mut()` 触发被调方 `log.is_null()` 的无-log follow 分支，等价于 C++
          // `size(tp, log=nullptr)` 默认形参；二者仅做只读遍历，不产生别名或写入。
          && size(super_function.ret_types, None) == 1
          // Safety: 同上一条——ret_types 存活句柄、null log 走无 txn-log 只读分支。
          && unsafe { finite(super_function.ret_types, null_mut()) }
        {
          self.unifier_report_type_mismatch_ext(
            super_ty,
            sub_ty,
            "Return type is not compatible.".into(),
            inner_state.errors.first().cloned(),
          );
        } else if !inner_state.errors.is_empty() && inner_state.first_pack_error_pos.is_some() {
          let reason = format!(
            "Return #{} type is not compatible.",
            inner_state
              .first_pack_error_pos
              .expect("外层 173 行 is_some() 判定已成立")
          );
          self.unifier_report_type_mismatch_ext(
            super_ty,
            sub_ty,
            reason.clone(),
            inner_state.errors.first().cloned(),
          );
        } else if !inner_state.errors.is_empty() {
          self.unifier_report_type_mismatch_ext(
            super_ty,
            sub_ty,
            "".into(),
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
    super_function = get_mutable_type::get_mutable::<FunctionType>(super_ty)
      .expect("入口 let-else 已证 FunctionType，cpp 重取后同样免判直 deref（见上注释）");
    sub_function = get_mutable_type::get_mutable::<FunctionType>(sub_ty)
      .expect("入口 let-else 已证 FunctionType，cpp 重取后同样免判直 deref（见上注释）");

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
}
