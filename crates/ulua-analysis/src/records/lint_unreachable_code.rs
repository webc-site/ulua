use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction, ast_stat::AstStat,
    ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak, ast_stat_continue::AstStatContinue,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_if::AstStatIf, ast_stat_repeat::AstStatRepeat, ast_stat_return::AstStatReturn,
    ast_stat_while::AstStatWhile, ast_visitor::AstVisitor,
  },
  rtti::{AstNodePtr, ast_node_is_ptr, ast_node_try_as_ptr},
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  enums::status::Status,
  functions::{does_call_error::does_call_error, emit_warning::emit_warning},
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintUnreachableCode<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintUnreachableCode<'ctx> {
  fn visit_expr_function(&mut self, node: &mut AstExprFunction) -> bool {
    let body = node.body;
    self.analyze(body.cast::<AstStat>().as_ptr());
    true
  }
}

// —— 原 methods/lint_unreachable_code_analyze.rs ——
impl<'ctx> LintUnreachableCode<'ctx> {
  /// cpp `LintUnreachableCode::analyze(AstStat* node)` 的直译。
  pub fn analyze(&mut self, node: *mut AstStat) -> Status {
    if node.is_null() {
      return Status::Unknown;
    }
    // repr(C) 基类在偏移 0，`as *mut AstNode` 是基址不变的类型视图转换。
    let node_base = node.as_ast_node();
    // Safety: `node_base` 由 `process`/`visit` 传入的 arena 根或嵌套 body/then
    // 等子节点指针构成（parser 分配、lint 全程存活），入口判空后即为存活
    // repr(C) 节点；try_as_ptr 只读偏移 0 的 class_index 判型，命中返回的
    // &'static 引用与 cpp `node->as<AstStatBlock>()` 同一基址，且全程只读 AST。
    if let Some(block) = unsafe { ast_node_try_as_ptr::<AstStatBlock>(node_base) } {
      let body = &block.body;
      for (i, si) in body.iter_nodes().enumerate() {
        let step = self.analyze(si.as_ptr());
        if step != Status::Unknown {
          if i + 1 == body.len() {
            return step;
          }
          let next = body[i + 1].as_ptr();
          // ast_node_is 的指针形态对 null 返回 false、非空才读 class_index，
          // 是安全门面；si/next 同为 arena 存活语句节点。
          if step == Status::Error
            && unsafe { ast_node_is_ptr::<AstStatExpr>(si.as_ptr()) }
            && unsafe { ast_node_is_ptr::<AstStatReturn>(next) }
            && i + 2 == body.len()
          {
            return Status::Error;
          }
          // 句柄为 Copy，先局部化，使 emit_warning 的 `&mut` 重建不与
          // `self.get_reason` 的只读借用相互冲突。
          let mut ctx = self.context;
          emit_warning(
            ctx.get(),
            Code::UnreachableCode,
            // Safety: next 是 body 数组槽位里的存活语句指针（parser 成对写入
            // data/size），location 为偏移 0 基类上的 Copy 字段读取。
            unsafe { (*next).base.location },
            format_args!(
              "Unreachable code (previous statement always {}s)",
              self.get_reason(step)
            ),
          );
          return step;
        }
      }
      return Status::Unknown;
    }
    // Safety: 同上——入口已保证 node 为存活节点基址；判型命中后只在共享引用
    // 上读子指针。thenbody 按 parser 不变量非空，elsebody 可为 null（cpp
    // `elsebody ? analyze(elsebody) : Unknown` 的同款判空）；递归 analyze 本身
    // 是安全函数且对 null 入口返回 Unknown。
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatIf>(node_base) } {
      let ifs = self.analyze(stat.thenbody.cast::<AstStat>().as_ptr());
      let elses = if stat.elsebody.is_null() {
        Status::Unknown
      } else {
        self.analyze(stat.elsebody.as_ptr())
      };
      return min_status(ifs, elses);
    }
    // Safety: node 存活非空且只读 class_index 判型；命中的 while 分支只经共享
    // 引用读出 body 指针（parser 恒置非空），再交给安全的递归 analyze。
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatWhile>(node_base) } {
      self.analyze(stat.body.cast::<AstStat>().as_ptr());
      return Status::Unknown;
    }
    // Safety: repeat 分支与 while 同构——判型引用只读；body 已句柄化为 Node
    // （parser 非空由类型层承载），cast/as_ptr 桥交指针形态的 analyze。
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatRepeat>(node_base) } {
      self.analyze(stat.body.cast::<AstStat>().as_ptr());
      return Status::Unknown;
    }
    if unsafe { ast_node_is_ptr::<AstStatBreak>(node_base) } {
      return Status::Break;
    }
    if unsafe { ast_node_is_ptr::<AstStatContinue>(node_base) } {
      return Status::Continue;
    }
    if unsafe { ast_node_is_ptr::<AstStatReturn>(node_base) } {
      return Status::Return;
    }
    // Safety: 判型命中的引用只读；expr 是 AstStatExpr 恒非空的实参槽位
    // （parser 构造保证），交给安全的递归前先经共享引用取值。
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatExpr>(node_base) } {
      let expr = stat.expr;
      // Safety: expr 为存活 arena 表达式指针（上一条论证），第二次 try_as_ptr
      // 仅再读其 class_index；命中后 does_call_error 收只读引用，不改节点。
      if let Some(call) = unsafe { ast_node_try_as_ptr::<AstExprCall>(expr) }
        && does_call_error(call)
      {
        return Status::Error;
      }
      return Status::Unknown;
    }
    // Safety: for 分支同 while/repeat——判型引用只读；body 已句柄化为 Node，
    // cast/as_ptr 桥交指针形态的 analyze。
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatFor>(node_base) } {
      self.analyze(stat.body.cast::<AstStat>().as_ptr());
      return Status::Unknown;
    }
    // Safety: forin 分支同上——判型只读 class_index，body 读的是存活
    // `AstStatForIn` 的字段值。
    if let Some(stat) = unsafe { ast_node_try_as_ptr::<AstStatForIn>(node_base) } {
      self.analyze(stat.body.cast::<AstStat>().as_ptr());
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

// —— 原 methods/lint_unreachable_code_get_reason.rs ——
impl<'ctx> LintUnreachableCode<'ctx> {
  pub fn get_reason(&self, status: Status) -> &str {
    match status {
      Status::Continue => "continue",
      Status::Break => "break",
      Status::Return => "return",
      Status::Error => "error",
      _ => "unknown",
    }
  }
}

// —— 原 methods/lint_unreachable_code_process.rs ——
impl<'ctx> LintUnreachableCode<'ctx> {
  pub fn process(context: &mut LintContext) {
    let root = context.root;
    let mut pass = LintUnreachableCode {
      context: LintContextHandle::from_ref(context),
    };
    pass.analyze(root);
    // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
    // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
    unsafe {
      ast_stat_visit(root, &mut pass);
    }
  }
}

// —— 原 methods/lint_unreachable_code_visit.rs ——
impl<'ctx> LintUnreachableCode<'ctx> {
  /// cpp `visit(AstExprFunction*)`：`node` 为分析期存活、由 arena 持有的函数节点
  /// 共享借用（cpp 裸指针形参的 Rust 对应）；`body` 字段仍是裸指针，交由 `analyze`
  /// 按其自身契约处理。
  pub fn visit(&mut self, node: &AstExprFunction) -> bool {
    self.analyze(node.body.cast::<AstStat>().as_ptr());
    true
  }
}
