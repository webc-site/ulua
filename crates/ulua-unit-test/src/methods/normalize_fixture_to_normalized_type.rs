//! @interface-stub
use alloc::{string::String, sync::Arc};

use ulua_analysis::{
  functions::find_node_at_position_ast_query::find_node_at_position_source_module_position,
  records::normalized_type::NormalizedType,
};
use ulua_ast::{
  records::{ast_stat_type_alias::AstStatTypeAlias, position::Position},
  rtti::ast_node_as,
};
use ulua_common::FFlag;

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

    let ty = if !FFlag::DebugLuauForceOldSolver.get() {
      let source_module = self.base.get_main_source_module();
      assert!(!source_module.is_null(), "expected main source module");

      let node = unsafe {
        find_node_at_position_source_module_position(
          &*source_module,
          Position { line: 0, column: 5 },
        )
      };
      assert!(!node.is_null(), "expected type alias AST node");

      let alias = unsafe { ast_node_as::<AstStatTypeAlias>(node) };
      assert!(!alias.is_null(), "expected AstStatTypeAlias");

      let module = self.base.get_main_module(false);
      assert!(!module.is_null(), "expected main module");

      unsafe {
        (*module)
          .ast_resolved_types
          .find(&((*alias).type_ptr as *const _))
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
