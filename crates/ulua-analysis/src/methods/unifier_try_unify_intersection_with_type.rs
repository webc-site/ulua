use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use crate::{
  functions::has_unification_too_complex::has_unification_too_complex,
  records::{
    intersection_type::IntersectionType, normalization_too_complex::NormalizationTooComplex,
    txn_log::TxnLog, type_error::TypeError, unifier::Unifier,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

impl Unifier {
  /// Rust 形态（§2）：C++ `const IntersectionType* uv` → `&'static IntersectionType`
  /// （`uv` 由调用方对 `txn_log_get_mutable`/`get_type_id` 快照经 `alias_opt` 门面
  /// 折叠得到，指向 arena bump 分块中地址稳定的节点，全函数只读 `parts`）。
  pub fn unifier_try_unify_intersection_with_type(
    &mut self,
    sub_ty: TypeId,
    uv: &'static IntersectionType,
    super_ty: TypeId,
    cache_enabled: bool,
    is_function_call: bool,
  ) {
    let mut found = false;
    let mut errors_suppressed = false;
    let mut unification_too_complex: Option<TypeError> = None;
    let mut start_index = 0usize;

    if cache_enabled {
      for (i, ty) in uv.parts.iter().enumerate() {
        // `self.shared_state` 为 `Handle` 单例句柄（契约见 records/arena_handle.rs），
        // 此处仅共享借用做 `cached_unify` 只读查找；`*ty` 是对 `parts`
        // 元素引用的安全解引用。
        if self
          .shared_state
          .get()
          .cached_unify
          .find(&(*ty, super_ty))
          .is_some()
        {
          start_index = i;
          break;
        }
      }
    }

    let mut logs: Vec<TxnLog> = Vec::new();

    // 从首个缓存命中处轮转遍历各 part
    for ty in uv
      .parts
      .iter()
      .copied()
      .cycle()
      .skip(start_index)
      .take(uv.parts.len())
    {
      let mut inner_state = self.unifier_make_child_unifier();
      inner_state.normalize = false;
      inner_state.try_unify_type_id_type_id_bool_bool_literal_properties(
        ty,
        super_ty,
        is_function_call,
        false,
        None,
      );

      if inner_state.errors.is_empty() {
        found = true;
        errors_suppressed = inner_state.failure;
        if inner_state.failure {
          logs.push(inner_state.log);
        } else {
          errors_suppressed = false;
          self.log.concat(inner_state.log);
          break;
        }
      } else if let Some(e) = has_unification_too_complex(&inner_state.errors) {
        unification_too_complex = Some(e);
      }
    }

    if errors_suppressed && !logs.is_empty() {
      self.log.concat(logs.remove(0));
    }

    if let Some(e) = unification_too_complex {
      self.report_error_type_error(e);
    } else if !found && self.normalize {
      // A & B <: T 即使 A </: T 且 B </: T 也可能成立（如 string? & number? <: nil），
      // 借助归一化处理；归一化过于复杂时与 C++ 一致报错
      match (
        self.normalizer_mut().try_normalize(sub_ty),
        self.normalizer_mut().try_normalize(super_ty),
      ) {
        (Some(sub_norm), Some(super_norm)) => self.unifier_try_unify_normalized_types(
          sub_ty,
          super_ty,
          &sub_norm,
          &super_norm,
          "none of the intersection parts are compatible".to_string(),
          None,
        ),
        _ => self.report_error_location_type_error_data(
          self.location,
          TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
        ),
      }
    } else if !found {
      self.unifier_report_type_mismatch_ext(
        super_ty,
        sub_ty,
        String::from("none of the intersection parts are compatible"),
        None,
      );
    } else if errors_suppressed {
      self.failure = true;
    }
  }
}
