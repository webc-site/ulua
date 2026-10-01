use ulua_ast::{records::ast_type::AstType, rtti::AstNodePtr};

use crate::records::ast_type_visitor_tracking_withs::AstTypeVisitorTrackingWiths;

impl AstTypeVisitorTrackingWiths {
  pub fn visit(&mut self, n: *mut AstType) -> bool {
    // 基类上转收口在 `AstNodePtr::as_ast_node` 门面（repr(C) 基址重合），免手写 `as`。
    self.base.visit(n.as_ast_node())
  }
}
