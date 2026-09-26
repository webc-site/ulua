use alloc::vec::Vec;
use core::ptr::{from_mut, null_mut};

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
    ast_expr_local::AstExprLocal, ast_expr_type_assertion::AstExprTypeAssertion,
    ast_local::AstLocal, ast_node::AstNode, ast_stat_assign::AstStatAssign,
    ast_stat_local::AstStatLocal, ast_type::AstType, ast_type_pack::AstTypePack,
    ast_visitor::AstVisitor,
  },
  rtti::ast_node_try_as_ptr,
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
    // SAFETY: 遍历入口保证 `node.func` 指向存活 AstExpr；ast_node_try_as_ptr 判空并按
    // class_index 分派，命中仅只读 name，无写方别名。
    let is_require = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(node.func) }
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
      // SAFETY: `v` 来自遍历期间存活的 AST 节点字段；ast_node_try_as_ptr 判空并按
      // class_index 分派，命中仅按值读出 `.local`（parser 对 AstExprLocal.local 恒置
      // 非空），不写该节点、与遍历借用无别名冲突。
      if let Some(expr_local) = unsafe { ast_node_try_as_ptr::<AstExprLocal>(v) } {
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
