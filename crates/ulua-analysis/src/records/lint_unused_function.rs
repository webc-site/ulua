use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_name::AstName,
    ast_stat_function::AstStatFunction, ast_visitor::AstVisitor, location::Location,
  },
  rtti::ast_node_try_as_ptr,
  visit::{ast_expr_visit, ast_stat_visit},
};
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};
#[derive(Debug, Clone, Default)]
pub struct Global {
  pub(crate) location: Location,
  pub(crate) function: bool,
  pub(crate) used: bool,
}

impl DenseDefault for Global {
  fn dense_default() -> Self {
    Self::default()
  }
}

#[derive(Debug, Clone)]
pub struct LintUnusedFunction<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
  pub(crate) globals: DenseHashMap<AstName, Global>,
}

impl<'ctx> LintUnusedFunction<'ctx> {
  pub fn process(context: &'ctx mut LintContext) {
    let root = context.root;
    let mut pass = Self {
      context: LintContextHandle::from_ref(context),
      globals: DenseHashMap::default(),
    };
    // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
    // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
    unsafe { ast_stat_visit(root, &mut pass) };
    pass.report();
  }

  pub fn report(&mut self) {
    let mut handle = self.context;
    for (name, global) in self.globals.iter() {
      if global.function && !global.used {
        // AstName::as_bytes 容忍 null（空名 → 空切片，直接跳过）。
        let name_bytes = name.as_bytes();
        if !name_bytes.is_empty() && name_bytes[0] != b'_' {
          emit_warning(
            handle.get(),
            Code::FunctionUnused,
            global.location,
            format_args!(
              "Function '{}' is never used; prefix with '_' to silence",
              name.as_str_or_empty()
            ),
          );
        }
      }
    }
  }

  pub(crate) fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    // SAFETY: 遍历入口保证节点存活；name/func 已句柄化为 Node（非空由类型层
    // 承载），`as_ptr` 桥交仍以指针形态消费的判型/分发门面。
    if let Some(expr) = unsafe { ast_node_try_as_ptr::<AstExprGlobal>(node.name.as_ptr()) } {
      let g = self.globals.get_or_insert(expr.name);
      g.function = true;
      g.location = expr.base.base.location;
      unsafe {
        ast_expr_visit(node.func.cast::<AstExpr>().as_ptr(), self);
      }
      return false;
    }
    true
  }

  pub(crate) fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    let name = node.name;
    let g = self.globals.get_or_insert(name);
    g.used = true;
    true
  }
}

impl<'ctx> AstVisitor for LintUnusedFunction<'ctx> {
  fn visit_stat_function(&mut self, node: &mut AstStatFunction) -> bool {
    self.visit_stat_function(node)
  }

  fn visit_expr_global(&mut self, node: &mut AstExprGlobal) -> bool {
    self.visit_expr_global(node)
  }
}
