use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::{
  enums::control_flow::ControlFlow,
  functions::{flatten_type_pack::flatten_type_pack_id, get_mutable_type::get_mutable_type_id},
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};

impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_assign(
    &mut self,
    scope: &ScopePtr,
    assign: *mut AstStatAssign,
  ) -> ControlFlow {
    let result_pack = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
        scope,
        unsafe { (*assign).values },
        &[],
      )
      .tp;
    let mut value_types = Vec::new();
    let (head, _) = flatten_type_pack_id(result_pack);

    unsafe {
      let vars_size = (*assign).vars.size;
      if head.len() >= vars_size {
        value_types.extend_from_slice(&head[..vars_size]);
      } else {
        for _ in 0..vars_size {
          value_types.push((*self.arena).add_type(BlockedType::default()));
        }
        let uc = self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          (*assign).base.base.location,
          ConstraintV::Unpack(UnpackConstraint {
            result_pack: value_types.clone(),
            source_pack: result_pack,
          }),
        );
        for t in &value_types {
          get_mutable_type_id::<BlockedType>(*t)
            .as_mut()
            .unwrap()
            .set_owner(uc as *const _);
        }
      }
      for (i, &vt) in value_types.iter().enumerate() {
        self.visit_l_value_scope_ptr_ast_expr_type_id(scope, *(*assign).vars.data.add(i), vt);
      }
    }
    ControlFlow::None
  }
}
