use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
  ast_stat_return::AstStatReturn,
};

use crate::{
  enums::value_context::ValueContext,
  functions::extend_type_pack::extend_type_pack,
  records::{type_checker_2::TypeChecker2, type_pack::TypePack},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `ret` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_return(&mut self, ret: *mut AstStatReturn) {
    unsafe {
      let location = (*ret).base.base.location;

      let scope_ptr = self.find_innermost_scope(location);
      let expected_ret_type = (*scope_ptr).return_type;

      let list = (*ret).list;
      if list.size == 0 {
        let empty_type_pack = (*self.builtin_types).empty_type_pack;
        self.test_is_subtype_type_pack_id_type_pack_id_location(
          empty_type_pack,
          expected_ret_type,
          location,
        );
        return;
      }

      let builtin_types = self.builtin_types;
      // C++: extendTypePack(module->internalTypes, builtinTypes, expectedRetType, ret->list.size)
      let extended_pack = extend_type_pack(
        &mut (*self.module).internal_types,
        builtin_types,
        expected_ret_type,
        list.size,
        Vec::new(),
      );

      let mut is_subtype = true;
      let mut actual_head: Vec<TypeId> = Vec::new();
      let mut actual_tail: Option<TypePackId> = None;

      let head = &extended_pack.head;
      let head_len = head.len();

      // 最后一个元素单独处理（last_expr），此处遍历前面的元素并与 head 对齐
      for (idx, &expr) in list.as_slice().iter().take(list.size - 1).enumerate() {
        if let Some(&expected_ty) = head.get(idx) {
          let subtype_result = self.test_literal_or_ast_type_is_subtype(expr, expected_ty);
          is_subtype &= subtype_result;
          actual_head.push(expected_ty);
        } else {
          // SAFETY: expr 指向 AST arena 节点。
          let ty = self.lookup_type(&*expr);
          actual_head.push(ty);
        }
      }

      let last_idx = list.size - 1;
      let last_expr = *list.data.add(last_idx);

      if head_len < list.size
        || (*(last_expr as *mut AstNode)).is::<AstExprCall>()
        || (*(last_expr as *mut AstNode)).is::<AstExprVarargs>()
      {
        actual_tail = Some(self.lookup_pack(last_expr));
      } else {
        let last_expected_ty = head[last_idx];
        let subtype_result = self.test_literal_or_ast_type_is_subtype(last_expr, last_expected_ty);
        is_subtype &= subtype_result;
        actual_head.push(last_expected_ty);
      }

      if is_subtype {
        let reconstructed_ret_type = (*self.module).internal_types.add_type_pack_t(TypePack {
          head: actual_head,
          tail: actual_tail,
        });
        self.test_is_subtype_type_pack_id_type_pack_id_location(
          reconstructed_ret_type,
          expected_ret_type,
          location,
        );
      }

      for &expr in list.as_slice() {
        self.visit_ast_expr_value_context(expr, ValueContext::RValue);
      }
    }
  }
}
