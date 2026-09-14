use ulua_ast::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_type_or_pack::AstTypeOrPack, location::Location,
};
use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::interesting_edge_case::InterestingEdgeCase,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type::follow_type_id,
    get_identifier_of_base_var_type_checker_2::get_identifier_of_base_var_mut,
    get_type_alt_j::get_type_id,
  },
  records::{
    function_type::FunctionType,
    instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction,
    intersection_type::IntersectionType, type_checker_2::TypeChecker2,
    type_instantiation_count_mismatch::TypeInstantiationCountMismatch,
  },
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn check_type_instantiation(
    &mut self,
    base_function_expr: &AstExpr,
    fn_type: TypeId,
    location: &Location,
    type_arguments: AstArray<AstTypeOrPack>,
  ) {
    LUAU_ASSERT!(FFlag::LuauExplicitTypeInstantiationSupport.get());

    let fn_type_followed = follow_type_id(fn_type);
    let Some(ftv) = get_type_id::<FunctionType>(fn_type_followed) else {
      let mut interesting_edge_case = InterestingEdgeCase::None;

      // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
      let has_call = unsafe {
        find_metatable_entry(
          self.builtin_types,
          &mut (*self.module).errors,
          fn_type,
          "__call",
          *location,
        )
      }
      .is_some();

      if has_call {
        interesting_edge_case = InterestingEdgeCase::MetatableCall;
      } else if get_type_id::<IntersectionType>(follow_type_id(fn_type)).is_some() {
        interesting_edge_case = InterestingEdgeCase::Intersection;
      }

      self.report_error_type_error_data_location(
        InstantiateGenericsOnNonFunction {
          interesting_edge_case,
        }
        .into(),
        location,
      );
      return;
    };

    let mut type_count: usize = 0;
    let mut type_pack_count: usize = 0;

    for type_or_pack in type_arguments.iter() {
      if !type_or_pack.r#type.is_null() {
        type_count += 1;
      } else {
        LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
        type_pack_count += 1;
      }
    }

    let generics_len = ftv.generics.len();
    let generic_packs_len = ftv.generic_packs.len();

    if generics_len < type_count || generic_packs_len < type_pack_count {
      // SAFETY: get_identifier_of_base_var_mut 尚未引用化；expr 指向 AST arena。
      let function_name =
        get_identifier_of_base_var_mut(base_function_expr as *const AstExpr as *mut AstExpr);
      self.report_error_type_error_data_location(
        TypeInstantiationCountMismatch {
          function_name,
          function_type: fn_type,
          provided_types: type_count,
          maximum_types: generics_len,
          provided_type_packs: type_pack_count,
          maximum_type_packs: generic_packs_len,
        }
        .into(),
        location,
      );
    }
  }
}
