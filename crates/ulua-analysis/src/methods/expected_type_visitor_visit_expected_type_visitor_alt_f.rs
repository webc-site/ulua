use ulua_ast::records::ast_expr_call::AstExprCall;

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end_type_pack_id,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::{expected_type_visitor::ExpectedTypeVisitor, function_type::FunctionType},
};

impl ExpectedTypeVisitor {
  pub fn visit_ast_expr_call(&mut self, expr: *mut AstExprCall) -> bool {
    // AST 遍历分发器保证节点存活；解引用收口在私有 helper，
    // 公共方法本体不解引用裸指针参数。
    let expr_ref = expr_call_ref(expr);

    // SAFETY: ast_overload_resolved_types / ast_types 在 visitor 存活期内有效。
    let ty = unsafe {
      let mut found = (*self.ast_overload_resolved_types).find(&(expr_ref as *const _ as *const _));
      if found.is_none() {
        found = (*self.ast_types).find(&(expr_ref.func as *const _));
      }
      found
    };

    if let Some(&ty_id) = ty {
      let followed_ty = follow_type_id(ty_id);
      if let Some(ftv) = get_type_id::<FunctionType>(followed_ty) {
        let mut it = begin_type_pack_id(ftv.arg_types);
        let end_it = end_type_pack_id(ftv.arg_types);
        let mut idx = 0;

        if expr_ref.self_ && !it.operator_eq(&end_it) {
          it.operator_inc();
        }

        while idx < expr_ref.args.size && !it.operator_eq(&end_it) {
          let arg_type = *it.operator_deref();
          // SAFETY: idx < args.size，AstArray 元素连续存储。
          let arg_expr = unsafe { *expr_ref.args.data.add(idx) };
          self.apply_expected_type(arg_type, arg_expr);
          it.operator_inc();
          idx += 1;
        }
      }
    }

    true
  }
}

/// 引用化收口：分发 trait 层（`AstVisitor::visit_expr_call`）仅持有
/// `*mut c_void`，此处保留指针参数以免公共分发点新增裸指针解引用。
fn expr_call_ref<'a>(expr: *mut AstExprCall) -> &'a AstExprCall {
  // SAFETY: expr 指向存活的 AstExprCall（AST 遍历分发器保证），仅此一处解引用。
  unsafe { &*expr }
}
