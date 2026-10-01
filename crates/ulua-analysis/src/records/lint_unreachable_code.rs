use ulua_ast::{
  enums::ast_stat_ref::AstStatRef,
  records::{
    ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction, ast_stat::AstStat,
    ast_stat_block::AstStatBlock, ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor,
    ast_stat_for_in::AstStatForIn, ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_while::AstStatWhile, ast_visitor::AstVisitor,
  },
  rtti::{ast_node_is, ast_node_try_as},
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  enums::status::Status,
  functions::{does_call_error::does_call_error, emit_warning::emit_warning},
  records::{
    arena_handle::alias_ref, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
};

#[derive(Debug, Clone)]
pub struct LintUnreachableCode<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintUnreachableCode<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    self.analyze(&node.body.base);
    true
  }
}

// —— 原 methods/lint_unreachable_code_analyze.rs ——
impl<'ctx> LintUnreachableCode<'ctx> {
  /// cpp `LintUnreachableCode::analyze(AstStat* node)` 的引用形态：可空性由
  /// 调用方的 `Option` 折叠（elsebody / 根节点），本函数体内全部下转都经
  /// 生命周期正确的 [`ast_node_try_as`]，借用半径即入参 `node` 的借用。
  pub(crate) fn analyze(&mut self, node: &AstStat) -> Status {
    // 下转命中 AstStatBlock 时给出块引用（class_index 判型，纯只读）。
    if let Some(block) = ast_node_try_as::<AstStatBlock>(node) {
      let body = &block.body;
      for (i, si) in body.iter().enumerate() {
        let step = self.analyze(si);
        if step != Status::Unknown {
          if i + 1 == body.len() {
            return step;
          }
          let next = body.at(i + 1).get();
          if step == Status::Error
            && ast_node_is::<AstStatExpr>(si)
            && ast_node_is::<AstStatReturn>(next)
            && i + 2 == body.len()
          {
            return Status::Error;
          }
          // 句柄为 Copy，先局部化，使 emit_warning 的 `&mut` 重建不与其他借用冲突。
          let mut ctx = self.context;
          emit_warning(
            ctx.get(),
            Code::UnreachableCode,
            next.base.location,
            format_args!(
              "Unreachable code (previous statement always {}s)",
              Into::<&'static str>::into(step)
            ),
          );
          return step;
        }
      }
      return Status::Unknown;
    }
    // thenbody 按 parser 不变量非空（Node 句柄承载），elsebody 可为 null
    // （cpp `elsebody ? analyze(elsebody) : Unknown` 的同款判空，折叠为
    // Option::map_or）。
    if let Some(stat) = ast_node_try_as::<AstStatIf>(node) {
      let ifs = self.analyze(&stat.thenbody.base);
      let elses = stat
        .elsebody
        .get()
        .map_or(Status::Unknown, |elsebody| self.analyze(elsebody));
      return min_status(ifs, elses);
    }
    // while/repeat/for/forin 同构——判型命中后只在共享引用上读 body 句柄。
    if let Some(stat) = ast_node_try_as::<AstStatWhile>(node) {
      self.analyze(&stat.body.base);
      return Status::Unknown;
    }
    if let Some(stat) = ast_node_try_as::<AstStatRepeat>(node) {
      self.analyze(&stat.body.base);
      return Status::Unknown;
    }
    match node.as_stat_ref() {
      AstStatRef::Break(_) => return Status::Break,
      AstStatRef::Continue(_) => return Status::Continue,
      AstStatRef::Return(_) => return Status::Return,
      _ => {}
    }
    // expr 是 AstStatExpr 恒非空的实参槽位（parser 构造保证，Node 承载）。
    if let Some(stat) = ast_node_try_as::<AstStatExpr>(node) {
      if let Some(call) = ast_node_try_as::<AstExprCall>(stat.expr.get())
        && does_call_error(call)
      {
        return Status::Error;
      }
      return Status::Unknown;
    }
    if let Some(stat) = ast_node_try_as::<AstStatFor>(node) {
      self.analyze(&stat.body.base);
      return Status::Unknown;
    }
    if let Some(stat) = ast_node_try_as::<AstStatForIn>(node) {
      self.analyze(&stat.body.base);
      return Status::Unknown;
    }
    Status::Unknown
  }
}
fn min_status(lhs: Status, rhs: Status) -> Status {
  if status_rank(lhs) <= status_rank(rhs) {
    lhs
  } else {
    rhs
  }
}
fn status_rank(status: Status) -> u8 {
  match status {
    Status::Unknown => 0,
    Status::Continue => 1,
    Status::Break => 2,
    Status::Return => 3,
    Status::Error => 4,
  }
}

// —— 原 methods/lint_unreachable_code_process.rs ——
impl<'ctx> LintUnreachableCode<'ctx> {
  pub fn process(context: &mut LintContext) {
    let root = context.root;
    let mut pass = LintUnreachableCode {
      context: LintContextHandle::from_ref(context),
    };
    // `LintContext::root` 仍是 records 引用化波次前的裸指针字段：null 即 cpp
    // 的空根（analyze 直接短路 Unknown）；非 null 出自贯穿整趟 lint pass 存活
    // 的 arena 树，`alias_ref` 是 analysis 侧该契约的既有单点门面。
    if let Some(root) = (!root.is_null()).then(|| alias_ref(root)) {
      pass.analyze(root);
    }
    // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
    // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
    unsafe {
      ast_stat_visit(root, &mut pass);
    }
  }
}

// —— 原 methods/lint_unreachable_code_visit.rs ——
impl<'ctx> LintUnreachableCode<'ctx> {
  /// cpp `visit(AstExprFunction*)`：`node` 为分析期存活的函数节点共享借用；
  /// `body` 句柄经基类字段链升为 `&AstStat` 交给引用形态的 `analyze`。
  pub fn visit(&mut self, node: &AstExprFunction) -> bool {
    self.analyze(&node.body.base);
    true
  }
}
