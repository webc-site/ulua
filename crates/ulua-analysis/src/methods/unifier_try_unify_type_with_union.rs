use alloc::string::{String, ToString};

use crate::{
  functions::{
    get_name_type::get_name, get_table_match_tag::get_table_match_tag,
    has_unification_too_complex::has_unification_too_complex, is_prim::is_nil,
  },
  records::{
    arena_handle::alias_ref, normalization_too_complex::NormalizationTooComplex,
    type_error::TypeError, unifier::Unifier, union_type::UnionType,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  /// Rust 形态（§2）：C++ `const UnionType* uv` → `&'static UnionType`，
  /// 判空与解引用收口在调用方的 `alias_opt` 门面，本方法无裸指针。
  pub fn unifier_try_unify_type_with_union(
    &mut self,
    sub_ty: TypeId,
    super_ty: TypeId,
    uv: &'static UnionType,
    cache_enabled: bool,
    is_function_call: bool,
  ) {
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
      // 门面化：match tag 的 arena 存活 SingletonType 裸指针经 alias_ref 折叠为
      // 共享借用（§2 收口点 records/arena_handle.rs），以下判等不再手写解引用。
      let sub_singleton = alias_ref(sub_match_tag.1);
      for (i, option) in uv.options.iter().enumerate() {
        if let Some(option_match_tag) = get_table_match_tag(*option)
          && option_match_tag.0 == sub_match_tag.0
          && *alias_ref(option_match_tag.1) == *sub_singleton
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
        if self
          .shared_state
          .get()
          .cached_unify
          .find(&(sub_ty, *ty))
          .is_some()
        {
          start_index = i;
          break;
        }
      }
    }

    // 从 start_index 起轮转尝试各 option
    for ty in uv
      .options
      .iter()
      .copied()
      .cycle()
      .skip(start_index)
      .take(uv.options.len())
    {
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
      // 归一化过于复杂时与 C++ 一致报错并整体返回
      let sub_norm = self.normalizer_mut().try_normalize(sub_ty);
      let super_norm = self.normalizer_mut().try_normalize(super_ty);
      let (Some(sub_norm), Some(super_norm)) = (sub_norm, super_norm) else {
        self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        );
        return;
      };
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
        // 手抄 TypeMismatch 构造收敛到 ext 骨架单点（context 仍先于 error 求值，
        // failed_option 直接透传，map(Arc::new) 已在 ext 内等价完成）
        self.unifier_report_type_mismatch_ext(
          super_ty,
          sub_ty,
          String::from(reason),
          failed_option,
        );
      }
    }
  }
}
