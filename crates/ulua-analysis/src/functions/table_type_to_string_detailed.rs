//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1448:table_type_to_string_detailed`
//! Source: `Analysis/src/ToString.cpp:1448-1481` (hand-ported)

use alloc::format;

use crate::{
  enums::ignore_synthetic_name::IgnoreSyntheticName,
  functions::can_use_type_name_in_scope::can_use_type_name_in_scope,
  records::{
    table_type::TableType, to_string_result::ToStringResult, to_string_span::ToStringSpan,
    type_stringifier::TypeStringifier,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

/// C++ `static void tableTypeToStringDetailed(...)`.
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn table_type_to_string_detailed(
  ty: TypeId,
  ttv: *const TableType,
  ignore_synthetic_name: IgnoreSyntheticName,
  result: &mut ToStringResult,
  scope: &Option<ScopePtr>,
  name_to_use: &str,
  tvs: &mut TypeStringifier,
) {
  unsafe {
    if ignore_synthetic_name == IgnoreSyntheticName::No && (*ttv).synthetic_name.is_some() {
      result.invalid = true;
    }

    // If scope is provided, add module name and check visibility
    if let (Some(name), Some(scope)) = (&(*ttv).name, scope) {
      let (success, module_name) = can_use_type_name_in_scope(scope.clone(), name);

      if !success {
        result.invalid = true;
      }

      if let Some(module_name) = module_name {
        result.name = format!("{}.", module_name);
      }
    }

    let start_pos = result.name.len();
    result.name.push_str(name_to_use);
    let end_pos = result.name.len();

    if end_pos > start_pos {
      result.type_spans.push(ToStringSpan {
        start_pos,
        end_pos,
        r#type: ty,
      });
    }

    tvs.stringify_vector_type_id_vector_type_pack_id(
      &(*ttv).instantiated_type_params,
      &(*ttv).instantiated_type_pack_params,
    );
  }
}
