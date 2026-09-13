use core::ptr::null_mut;

use ulua_ast::records::ast_expr_constant_bool::AstExprConstantBool;

use crate::{
  enums::polarity::Polarity,
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    constraint_generator::ConstraintGenerator, free_type::FreeType, inference::Inference,
    primitive_type_constraint::PrimitiveTypeConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  pub fn check_scope_ptr_ast_expr_constant_bool_optional_type_id_bool(
    &mut self,
    scope: &ScopePtr,
    bool_expr: &AstExprConstantBool,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> Inference {
    // SAFETY: builtin_types/arena 为会话级裸指针句柄，与 C++ 成员指针同契约。
    unsafe {
      let singleton_type = if bool_expr.value {
        (*self.builtin_types).true_type
      } else {
        (*self.builtin_types).false_type
      };
      if force_singleton {
        return Inference::inference_type_id_refinement_id(singleton_type, null_mut());
      }

      if self.large_table_depth > 0 {
        return Inference::inference_type_id_refinement_id(
          (*self.builtin_types).boolean_type,
          null_mut(),
        );
      }

      let free_ty = self.fresh_type(scope, Polarity::Positive);
      // fresh_type 刚分配的必是 FreeType，对照 C++ `getMutable<FreeType>(freeTy)->...` 直接解引用
      let ft = get_mutable_type_id::<FreeType>(free_ty).unwrap();
      ft.lower_bound = singleton_type;
      ft.upper_bound = (*self.builtin_types).boolean_type;

      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        bool_expr.base.base.location,
        ConstraintV::PrimitiveType(PrimitiveTypeConstraint {
          free_type: free_ty,
          expected_type,
          primitive_type: (*self.builtin_types).boolean_type,
        }),
      );
      Inference::inference_type_id_refinement_id(free_ty, null_mut())
    }
  }
}
