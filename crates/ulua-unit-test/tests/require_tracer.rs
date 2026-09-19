//! 对照 `cpp/tests/RequireTracer.test.cpp`（TEST_SUITE "RequireTracerTest"）。

use std::ptr::from_ref;

use ulua_analysis::{
  functions::trace_requires::trace_requires,
  records::{require_trace_result::RequireTraceResult, type_check_limits::TypeCheckLimits},
};
use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
  ast_expr_index_name::AstExprIndexName, ast_stat_block::AstStatBlock,
  ast_stat_function::AstStatFunction, ast_stat_local::AstStatLocal,
  ast_type_pack_explicit::AstTypePackExplicit, ast_type_typeof::AstTypeTypeof,
};
use ulua_unit_test::{
  functions::ast_node_ref::{NodePtr, PtrRef, as_node_at, deref_at},
  methods::{
    require_tracer_fixture_parse::require_tracer_fixture_parse,
    require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture,
  },
  records::require_tracer_fixture::RequireTracerFixture,
};

/// cpp 各用例传入 `traceRequires` 的模块名。
const MODULE_NAME: &str = "ModuleName";

/// cpp `RequireTracerFixture`：arena、根块与 trace 结果同属一个作用域。
///
/// 根块以裸指针留存，避开"引用自持借用"的结构；`block()` 把它重新收窄为借用，
/// 存活期由 arena 字段担保，与 `&self` 同生命周期。
struct TraceFixture {
  /// 只为持有 arena（Allocator + AstNameTable）而存在：`root` 与 `result` 的
  /// 指针键都落在它内，字段存活期即借用有效期。
  _arena: RequireTracerFixture,
  root: *const AstStatBlock,
  result: RequireTraceResult,
}

impl TraceFixture {
  /// cpp 用例体：`parse(src)` 后立刻 `traceRequires(...)`。
  fn new(source: &str) -> Self {
    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = require_tracer_fixture_parse(&mut fixture.names, &mut fixture.allocator, source);

    let result = trace_requires(
      &mut fixture.file_resolver,
      block,
      MODULE_NAME.to_string(),
      &TypeCheckLimits::default(),
    );

    // 先固化为裸指针，结束 block 对 fixture 字段的借用，再移入 self。
    let root = from_ref(block);

    Self {
      _arena: fixture,
      root,
      result,
    }
  }

  fn block(&self) -> &AstStatBlock {
    // arena 由 self._arena 持有，root 必为非空。
    self.root.as_ref_opt().expect("根块必须存在")
  }

  /// cpp `REQUIRE(result.exprs.contains(e)) && CHECK_EQ(..., result.exprs[e].name)`：
  /// 未登记的 expr 直接判定用例失败。
  fn module_of<T>(&self, expr: *mut T) -> &str {
    self
      .result
      .exprs
      .find(&expr.cast())
      .expect("expr 未出现在 trace 结果中")
      .name
      .as_str()
  }
}

#[test]
fn trace_local() {
  let t = TraceFixture::new(
    r"
      local m = workspace.Foo.Bar.Baz
      require(m)
  ",
  );
  assert!(!t.result.exprs.empty());

  let loc = as_node_at::<AstStatLocal, _>(&t.block().body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!(1, loc.vars.size);
  assert_eq!(1, loc.values.size);

  // cpp：`value = value->expr->as<AstExprIndexName>()`，逐层回看索引链。
  let mut expr = loc.values.as_slice()[0];
  let mut value = expr
    .as_node::<AstExprIndexName>()
    .expect("values[0] 应为索引名");
  assert_eq!("workspace/Foo/Bar/Baz", t.module_of(expr));

  expr = value.expr;
  value = expr.as_node::<AstExprIndexName>().expect("expr 应为索引名");
  assert_eq!("workspace/Foo/Bar", t.module_of(expr));

  expr = value.expr;
  value = expr.as_node::<AstExprIndexName>().expect("expr 应为索引名");
  assert_eq!("workspace/Foo", t.module_of(expr));

  expr = value.expr;
  assert!(expr.as_node::<AstExprGlobal>().is_some(), "链尾应为全局名");
  assert_eq!("workspace", t.module_of(expr));
}

#[test]
fn trace_transitive_local() {
  let t = TraceFixture::new(
    r"
      local m = workspace.Foo.Bar.Baz
      local n = m.Quux
      require(n)
  ",
  );
  assert_eq!(3, t.block().body.size);

  let local = as_node_at::<AstStatLocal, _>(&t.block().body, 1).expect("body[1] 应为 AstStatLocal");
  assert_eq!(1, local.vars.size);

  let value = local.values.as_slice()[0];
  assert_eq!("workspace/Foo/Bar/Baz/Quux", t.module_of(value));
}

#[test]
fn trace_function_arguments() {
  let t = TraceFixture::new(
    r"
      local M = require(workspace.Game.Thing)
  ",
  );
  assert_eq!(1, t.block().body.size);

  let local = as_node_at::<AstStatLocal, _>(&t.block().body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!(1, local.vars.size);
  assert_eq!(1, local.values.size);

  let call = as_node_at::<AstExprCall, _>(&local.values, 0).expect("values[0] 应为调用");
  assert_eq!(1, call.args.size);

  assert_eq!("workspace/Game/Thing", t.module_of(call.args.as_slice()[0]));
}

#[test]
fn follow_typeof() {
  let t = TraceFixture::new(
    r"
      local R: typeof(require(workspace.CoolThing).UsefulObject)
  ",
  );
  assert_eq!(1, t.block().body.size);

  let local = as_node_at::<AstStatLocal, _>(&t.block().body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!(1, local.vars.size);

  let annotation = deref_at(&local.vars, 0)
    .expect("vars[0] 应为 AstLocal")
    .annotation;
  let typeof_annotation = annotation
    .as_node::<AstTypeTypeof>()
    .expect("annotation 应为 typeof");
  let index_name = typeof_annotation
    .expr
    .as_node::<AstExprIndexName>()
    .expect("typeof.expr 应为索引名");
  assert_eq!(Some("UsefulObject"), index_name.index.as_str());

  let call = index_name
    .expr
    .as_node::<AstExprCall>()
    .expect("expr 应为调用");
  assert_eq!(1, call.args.size);

  assert_eq!("workspace/CoolThing", t.module_of(call.args.as_slice()[0]));
}

#[test]
fn follow_typeof_in_return_type() {
  let t = TraceFixture::new(
    r"
      function foo(): typeof(require(workspace.CoolThing).UsefulObject)
      end
  ",
  );
  assert_eq!(1, t.block().body.size);

  let func = as_node_at::<AstStatFunction, _>(&t.block().body, 0).expect("body[0] 应为函数声明");

  let return_annotation = func
    .func
    .as_ref_opt()
    .expect("函数表达式必须存在")
    .return_annotation;
  let tp = return_annotation
    .as_node::<AstTypePackExplicit>()
    .expect("返回值应为显式 type pack");
  assert_eq!(1, tp.type_list.types.size);

  let typeof_annotation =
    as_node_at::<AstTypeTypeof, _>(&tp.type_list.types, 0).expect("types[0] 应为 typeof");
  let index_name = typeof_annotation
    .expr
    .as_node::<AstExprIndexName>()
    .expect("typeof.expr 应为索引名");
  assert_eq!(Some("UsefulObject"), index_name.index.as_str());

  let call = index_name
    .expr
    .as_node::<AstExprCall>()
    .expect("expr 应为调用");
  assert_eq!(1, call.args.size);

  assert_eq!("workspace/CoolThing", t.module_of(call.args.as_slice()[0]));
}

#[test]
fn follow_string_indexexpr() {
  let t = TraceFixture::new(
    r#"
      local R = game["Test"]
      require(R)
  "#,
  );
  assert_eq!(2, t.block().body.size);

  let local = as_node_at::<AstStatLocal, _>(&t.block().body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!("game/Test", t.module_of(local.values.as_slice()[0]));
}

#[test]
fn follow_group() {
  let t = TraceFixture::new(
    r"
      local R = (((game).Test))
      require(R)
  ",
  );
  assert_eq!(2, t.block().body.size);

  let local = as_node_at::<AstStatLocal, _>(&t.block().body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!("game/Test", t.module_of(local.values.as_slice()[0]));
}

#[test]
fn follow_type_annotation() {
  let t = TraceFixture::new(
    r"
      local R = game.Test :: (typeof(game.Redirect))
      require(R)
  ",
  );
  assert_eq!(2, t.block().body.size);

  let local = as_node_at::<AstStatLocal, _>(&t.block().body, 0).expect("body[0] 应为 AstStatLocal");
  assert_eq!("game/Redirect", t.module_of(local.values.as_slice()[0]));
}

#[test]
fn follow_type_annotation_2() {
  let t = TraceFixture::new(
    r"
      local R = game.Test :: (typeof(game.Redirect))
      local N = R.Nested
      require(N)
  ",
  );
  assert_eq!(3, t.block().body.size);

  let local = as_node_at::<AstStatLocal, _>(&t.block().body, 1).expect("body[1] 应为 AstStatLocal");
  assert_eq!(
    "game/Redirect/Nested",
    t.module_of(local.values.as_slice()[0])
  );
}
