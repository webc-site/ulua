use alloc::string::String;
use core::{ptr::null_mut, slice::from_raw_parts, str::from_utf8};

use ulua_ast::records::ast_expr_constant_string::AstExprConstantString;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    constraint_generator::ConstraintGenerator, free_type::FreeType, inference::Inference,
    primitive_type_constraint::PrimitiveTypeConstraint, singleton_type::SingletonType,
    string_singleton::StringSingleton,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, singleton_variant::SingletonVariant,
    type_id::TypeId,
  },
};
impl ConstraintGenerator {
  pub fn check_scope_ptr_ast_expr_constant_string_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    string: &AstExprConstantString,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> Inference {
    // SAFETY: builtin_types/arena 为会话级裸指针句柄；string_value 指向 AST 常量池，
    // 生命周期与模块 AST 同寿，与 C++ `std::string_view(value.data, value.size)` 同契约。
    unsafe {
      let string_value = string.value;
      let string_data = string_value.data;
      let string_size = string_value.size;
      let string_bytes = from_raw_parts(string_data as *const u8, string_size);
      let string_str = from_utf8(string_bytes).unwrap_or("");
      let string_singleton = StringSingleton::new(String::from(string_str));
      let singleton_type = SingletonType::new(SingletonVariant::V1(string_singleton));

      if force_singleton {
        return Inference::inference_type_id_refinement_id(
          (*self.arena).add_type(singleton_type),
          null_mut(),
        );
      }

      if self.large_table_depth > 0 {
        return Inference::inference_type_id_refinement_id(
          (*self.builtin_types).string_type,
          null_mut(),
        );
      }

      let free_ty = self.fresh_type(scope, Polarity::Positive);
      // fresh_type 刚分配的必是 FreeType，对照 C++ `getMutable<FreeType>(freeTy)` 后的 LUAU_ASSERT
      let ft = get_mutable_type_id::<FreeType>(free_ty);
      LUAU_ASSERT!(ft.is_some());
      let ft = ft.unwrap();
      ft.lower_bound = (*self.arena).add_type(singleton_type);
      ft.upper_bound = (*self.builtin_types).string_type;

      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        string.base.base.location,
        ConstraintV::PrimitiveType(PrimitiveTypeConstraint {
          free_type: free_ty,
          expected_type,
          primitive_type: (*self.builtin_types).string_type,
        }),
      );
      Inference::inference_type_id_refinement_id(free_ty, null_mut())
    }
  }
}
