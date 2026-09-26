use alloc::{string::String, sync::Arc};

use ulua_analysis::{
  functions::find_node_at_position_ast_query::find_node_at_position_source_module_position,
  records::normalized_type::NormalizedType,
};
use ulua_ast::{
  records::{ast_stat_type_alias::AstStatTypeAlias, position::Position},
  rtti::ast_node_try_as_ptr,
};
use ulua_common::fflag;

use crate::records::normalize_fixture::NormalizeFixture;
impl NormalizeFixture {
  pub fn to_normalized_type(
    &mut self,
    annotation: &str,
    expected_errors: usize,
  ) -> Option<Arc<NormalizedType>> {
    self.get_frontend();

    self
      .normalizer
      .as_mut()
      .expect("NormalizeFixture normalizer")
      .clear_caches();

    let mut source = String::from("type _Res = ");
    source.push_str(annotation);
    let result = self
      .base
      .check_string_optional_frontend_options(&source, None);
    self.base.validate_errors(&result.errors);
    assert_eq!(
      expected_errors,
      result.errors.len(),
      "{}",
      self.base.get_errors(&result)
    );

    let ty = if !fflag::DebugLuauForceOldSolver.get() {
      let source_module = self
        .base
        .get_main_source_module()
        .expect("expected main source module");

      let node = find_node_at_position_source_module_position(
        source_module.get(),
        Position { line: 0, column: 5 },
      );
      assert!(!node.is_null(), "expected type alias AST node");

      // 判型 + 下转 + 判空一步折叠为 Option（cpp `node->as<AstStatTypeAlias>()`），
      // 替代 `ast_node_as` 直调核心 + `!is_null` 断言 + `(*alias)` 裸解引用。
      let alias = unsafe { ast_node_try_as_ptr::<AstStatTypeAlias>(node) }
        .expect("expected AstStatTypeAlias");

      let module = self.base.get_main_module(false);
      assert!(!module.is_null(), "expected main module");

      // Safety: 上方断言 module 非空（resolver 保有的存活 Module）；alias.type_ptr 为别名语句在 arena 内存活类型节点指针（cpp 同款身份 map 键，签名涟漪归 B1），ast_resolved_types.find 只读查询。
      unsafe {
        (*module)
          .ast_resolved_types
          .find(&(alias.type_ptr as *const _))
          .copied()
      }
    } else {
      self.base.lookup_type(&String::from("_Res"))
    };

    let ty = ty.expect("expected resolved _Res type");
    self
      .normalizer
      .as_mut()
      .expect("NormalizeFixture normalizer")
      .try_normalize(ty)
  }
}
