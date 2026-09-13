use alloc::string::String;

use ulua_ast::{
  records::{ast_expr::AstExpr, ast_node::AstNode, ast_stat_assign::AstStatAssign},
  rtti::ast_node_as,
};

use crate::records::json_encoder_fixture::JsonEncoderFixture;
impl JsonEncoderFixture {
  pub fn expect_parse_expr(&mut self, src: &str) -> *mut AstExpr {
    let mut s = String::from("a = ");
    s.push_str(src);
    let root = self.expect_parse(&s);
    let root_ref = unsafe { &*root };

    ulua_common::LUAU_ASSERT!(root_ref.body.size > 0);
    let stat = unsafe { *root_ref.body.data.add(0) };

    let stat_assign = unsafe { ast_node_as::<AstStatAssign>(stat as *mut AstNode) };
    ulua_common::LUAU_ASSERT!(!stat_assign.is_null());

    let stat_assign_ref = unsafe { &*stat_assign };
    ulua_common::LUAU_ASSERT!(stat_assign_ref.values.size == 1);

    unsafe { *stat_assign_ref.values.data.add(0) }
  }
}
