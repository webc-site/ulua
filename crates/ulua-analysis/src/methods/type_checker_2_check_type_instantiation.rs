use core::ptr::from_ref;

use ulua_ast::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_type_or_pack::AstTypeOrPack, location::Location,
};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::interesting_edge_case::InterestingEdgeCase,
  functions::{
    find_metatable_entry::find_metatable_entry, follow_type,
    get_identifier_of_base_var_type_infer::get_identifier_of_base_var, get_type,
  },
  records::{
    arena_handle::Handle, function_type::FunctionType,
    instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction,
    intersection_type::IntersectionType, type_checker_2::TypeChecker2,
    type_instantiation_count_mismatch::TypeInstantiationCountMismatch,
  },
  type_aliases::type_id::TypeId,
};

impl TypeChecker2 {
  pub(crate) fn check_type_instantiation(
    &mut self,
    base_function_expr: &AstExpr,
    fn_type: TypeId,
    location: &Location,
    type_arguments: AstArray<AstTypeOrPack>,
  ) {
    LUAU_ASSERT!(fflag::LuauExplicitTypeInstantiationSupport.get());

    let fn_type_followed = follow_type::follow(fn_type);
    let Some(ftv) = get_type::get::<FunctionType>(fn_type_followed) else {
      let mut interesting_edge_case = InterestingEdgeCase::None;

      // SAFETY: self.module 与类型检查会话同寿（C++ 同契约）。
      let has_call = unsafe {
        find_metatable_entry(
          Handle::from_ptr(self.builtin_types.as_ptr()),
          &mut (*self.module).errors,
          fn_type,
          "__call",
          *location,
        )
      }
      .is_some();

      if has_call {
        interesting_edge_case = InterestingEdgeCase::MetatableCall;
      } else if get_type::get::<IntersectionType>(follow_type::follow(fn_type)).is_some() {
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
      // cpp：`if (type) ++typeCount; else { LUAU_ASSERT(typePack); ++typePackCount; }`
      // （TypeChecker2.cpp checkTypeInstantiation）——Error 形态即断言失败后仍按
      // pack 计数的一侧，保留同样的计数与上报顺序。
      match *type_or_pack {
        AstTypeOrPack::Type(_) => type_count += 1,
        AstTypeOrPack::Pack(_) => type_pack_count += 1,
        AstTypeOrPack::Error => {
          LUAU_ASSERT!(false);
          type_pack_count += 1;
        }
      }
    }

    let generics_len = ftv.generics.len();
    let generic_packs_len = ftv.generic_packs.len();

    if generics_len < type_count || generic_packs_len < type_pack_count {
      // SAFETY: get_identifier_of_base_var 尚未引用化；expr 指向 AST arena。
      let function_name = get_identifier_of_base_var(from_ref(base_function_expr).cast_mut());
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
