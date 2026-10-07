use alloc::vec::Vec;
use core::ptr::{from_mut, null_mut};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
  ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion, ast_local::AstLocal,
  ast_node::AstNode, ast_stat_assign::AstStatAssign, ast_stat_local::AstStatLocal,
  ast_type::AstType, ast_type_pack::AstTypePack, ast_visitor::AstVisitor, node_handle::OptNode,
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::{
  records::{
    file_resolver::FileResolver, module_info::ModuleInfo, require_trace_result::RequireTraceResult,
  },
  type_aliases::module_name_type::ModuleName,
};
impl DenseDefault for ModuleInfo {
  fn dense_default() -> Self {
    Self {
      name: ModuleName::new(),
      optional: false,
    }
  }
}

pub struct RequireTracer<'a> {
  pub(crate) result: *mut RequireTraceResult,
  /// C++ `FileResolver*`：以 trait object 引用承载，`trace_requires` 作用域内
  /// 由调用方保证存活。`dyn` 保留：`FileResolver` 实现方集合运行期开放。
  pub(crate) file_resolver: &'a mut dyn FileResolver,
  pub(crate) current_module_name: ModuleName,
  pub(crate) locals: DenseHashMap<*mut AstLocal, *mut AstExpr>,
  pub(crate) work: Vec<*mut AstNode>,
  pub(crate) require_calls: Vec<*mut AstExprCall>,
}

impl AstVisitor for RequireTracer<'_> {
  fn visit_expr_type_assertion(&mut self, _node: &mut AstExprTypeAssertion) -> bool {
    false
  }

  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    // `func` 槽为 records 引用化波次前的裸指针字段：局部句柄折叠判空后，
    // 判型走生命周期正确的 `try_as`，name 只读借用止于本表达式。
    let func = OptNode::from_ptr(node.func);
    let is_require = func
      .try_as::<AstExprGlobal>()
      .is_some_and(|global| global.name.as_str_or_empty() == "require");
    if is_require && node.args.size >= 1 {
      self.require_calls.push(from_mut(node));
    }
    true
  }

  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    // zip 在较短一侧结束，等价于 min(vars.size, values.size)
    for (&local, &expr) in node
      .vars
      .as_slice()
      .iter()
      .zip(node.values.as_slice().iter())
    {
      *self.locals.get_or_insert(local) = expr;
    }
    true
  }

  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    for &v in node.vars.as_slice() {
      // `v` 来自遍历期间存活的 AST 节点字段：局部句柄折叠判空，判型走
      // 生命周期正确的 `try_as`；命中仅按值读出 `.local` 身份指针（parser 对
      // AstExprLocal.local 恒置非空），不写该节点、与遍历借用无别名冲突。
      let v_slot = OptNode::from_ptr(v);
      if let Some(expr_local) = v_slot.try_as::<AstExprLocal>() {
        // local 槽已句柄化恒非空；locals 键值为既有裸指针形态，经 as_ptr 桥接。
        let local = expr_local.local.as_ptr();
        *self.locals.get_or_insert(local) = null_mut();
      }
    }
    true
  }

  fn visit_type(&mut self, _node: &mut AstType) -> bool {
    true
  }

  fn visit_type_pack(&mut self, _node: &mut AstTypePack) -> bool {
    true
  }
}
