use alloc::string::String;

use ulua_ast::records::{ast_expr::AstExpr, ast_stat_assign::AstStatAssign};

use crate::records::json_encoder_fixture::JsonEncoderFixture;
impl JsonEncoderFixture {
  pub fn expect_parse_expr(&mut self, src: &str) -> *mut AstExpr {
    let mut s = String::from("a = ");
    s.push_str(src);
    let root = self.expect_parse(&s);
    let root_ref = unsafe {
      // Safety: expect_parse 失败即 panic，返回恒为 fixture arena 存活的 AstStatBlock 根节点（块地址不动）；&* 物化只读借用读 body，与 cpp 测试 root-> 解引用同形。
      &*root
    };

    ulua_common::LUAU_ASSERT!(!root_ref.body.is_empty());
    let stat = root_ref.body[0];

    // `body[0]` 已是 Node 句柄：下转走句柄上生命周期正确的 `try_as`，借用半径
    // 由 `root_ref` 借用供给（夹具 arena 只读存活至本帧末），不再锻造假 'static。
    let stat_assign = stat.try_as::<AstStatAssign>();
    ulua_common::LUAU_ASSERT!(stat_assign.is_some());
    let stat_assign = stat_assign.expect("源码 `a = ...` 首语句恒为 AstStatAssign");

    ulua_common::LUAU_ASSERT!(stat_assign.values.len() == 1);

    stat_assign.values[0]
  }
}
