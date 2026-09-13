use alloc::{
  string::{String, ToString},
  sync::Arc,
};

use crate::{
  functions::{
    get_name_type::get_name, get_table_match_tag::get_table_match_tag,
    has_unification_too_complex::has_unification_too_complex, is_nil::is_nil,
  },
  records::{
    type_error::TypeError, type_mismatch::TypeMismatch, unifier::Unifier, union_type::UnionType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn unifier_try_unify_type_with_union(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    uv: *const UnionType,
    cache_enabled: bool,
    is_function_call: bool,
  ) {
    let uv = unsafe { &*uv };
    let mut found = false;
    let mut errors_suppressed = false;
    let mut unification_too_complex: Option<TypeError> = None;
    let mut failed_option_count = 0usize;
    let mut failed_option: Option<TypeError> = None;
    let mut found_heuristic = false;
    let mut start_index = 0usize;

    if let Some(sub_name) = get_name(sub_ty) {
      for (i, option) in uv.options.iter().enumerate() {
        if let Some(option_name) = get_name(*option)
          && option_name == sub_name
        {
          found_heuristic = true;
          start_index = i;
          break;
        }
      }
    }

    if let Some(sub_match_tag) = get_table_match_tag(sub_ty) {
      for (i, option) in uv.options.iter().enumerate() {
        if let Some(option_match_tag) = get_table_match_tag(*option)
          && option_match_tag.0 == sub_match_tag.0
          && unsafe { *option_match_tag.1 == *sub_match_tag.1 }
        {
          found_heuristic = true;
          start_index = i;
          break;
        }
      }
    }

    if !found_heuristic {
      for (i, ty) in uv.options.iter().enumerate() {
        if sub_ty == *ty {
          found_heuristic = true;
          start_index = i;
          break;
        }
      }
    }

    if !found_heuristic && cache_enabled {
      for (i, ty) in uv.options.iter().enumerate() {
        if unsafe {
          (*self.shared_state)
            .cached_unify
            .find(&(sub_ty, *ty))
            .is_some()
        } {
          start_index = i;
          break;
        }
      }
    }

    // 从 start_index 起轮转尝试各 option
    for (i, _) in uv.options.iter().enumerate() {
      let ty = uv.options[(i + start_index) % uv.options.len()];
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state.normalize = false;
      inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
        sub_ty,
        ty,
        is_function_call,
        false,
        None,
      );

      if !inner_state.failure {
        found = true;
        self.log.concat(inner_state.log);
        break;
      } else if inner_state.errors.is_empty() {
        errors_suppressed = true;
      } else if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        unification_too_complex = Some(e);
      } else if !is_nil(ty) {
        failed_option_count += 1;
        if failed_option.is_none() {
          failed_option = inner_state.errors.first().cloned();
        }
      }
    }

    if let Some(e) = unification_too_complex {
      self.report_error_type_error(e);
    } else if !found && self.normalize {
      let mut inner_state = self.unifier_make_child_unifier();
      let sub_norm = unsafe { (*self.normalizer).normalize(sub_ty) };
      let super_norm = unsafe { (*self.normalizer).normalize(super_ty) };
      if (failed_option_count == 1 || found_heuristic) && failed_option.is_some() {
        inner_state.unifier_try_unify_normalized_types(
          sub_ty,
          super_ty,
          &sub_norm,
          &super_norm,
          "None of the union options are compatible. For example:".to_string(),
          failed_option,
        );
      } else {
        inner_state.unifier_try_unify_normalized_types(
          sub_ty,
          super_ty,
          &sub_norm,
          &super_norm,
          "none of the union options are compatible".to_string(),
          None,
        );
      }

      if !inner_state.failure {
        self.log.concat(inner_state.log);
      } else if errors_suppressed || inner_state.errors.is_empty() {
        self.failure = true;
      } else {
        self.report_error_type_error(inner_state.errors.remove(0));
      }
    } else if !found {
      if errors_suppressed {
        self.failure = true;
      } else {
        let reason = if (failed_option_count == 1 || found_heuristic) && failed_option.is_some() {
          "None of the union options are compatible. For example:"
        } else {
          "none of the union options are compatible"
        };
        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::TypeMismatch(TypeMismatch {
            wanted_type: super_ty,
            given_type: sub_ty,
            reason: String::from(reason),
            error: failed_option.map(Arc::new),
            context,
          }),
        );
      }
    }
  }
}
