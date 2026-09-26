use core::{
  fmt::Arguments,
  ptr::{from_mut, from_ref},
};
use std::cmp::max;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_attr::AstAttr,
    ast_expr::AstExpr,
    ast_expr_binary::AstExprBinaryOp,
    ast_expr_call::AstExprCall,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
    ast_visitor::AstVisitor,
    location::Location,
  },
  visit::ast_stat_visit,
};
use ulua_config::enums::code::Code;

use crate::{
  enums::table_state::TableState,
  functions::{
    emit_warning::emit_warning, follow_type, get_type, is_string::is_string, similar::similar,
    size_type_pack::size,
  },
  records::{
    function_type::FunctionType, intersection_type::IntersectionType, lint_context::LintContext,
    lint_context_handle::LintContextHandle, table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

#[derive(Debug, Clone)]
pub struct LintTableOperations<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

impl<'ctx> AstVisitor for LintTableOperations<'ctx> {
  fn visit_expr_unary(&mut self, node: &mut AstExprUnary) -> bool {
    self.visit_ast_expr_unary(from_mut(node))
  }

  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    self.visit_ast_expr_call(from_mut(node))
  }

  // visit_node 沿用 trait 默认实现（返回 true）
  fn visit_attr(&mut self, _node: &mut AstAttr) -> bool {
    false
  }
}

// —— 原 methods/lint_table_operations_check_indexer.rs ——
impl<'ctx> LintTableOperations<'ctx> {
  pub fn check_indexer(&mut self, node: &AstExpr, expr: &AstExpr, op: &str) {
    // Safety: `expr` 是分发链借出的 `&AstExpr`（引用即存活证明），转成
    // 基址相同的裸指针仅作查表键，`get_type` 只读。
    let Some(ty) = self.context.get().get_type(from_ref(expr).cast_mut()) else {
      return;
    };
    let followed = follow_type::follow(ty);
    let Some(tty_ref) = get_type::get::<TableType>(followed) else {
      return;
    };
    if tty_ref.indexer.is_none()
      && !tty_ref.props.is_empty()
      && tty_ref.state != TableState::Generic
    {
      let msg = format!(
        "Using '{}' on a table without an array part is likely a bug",
        op
      );
      // Pass `format_args!` straight into the call: a `fmt::Arguments`
      // borrows its operands, so storing it in a `let` and using it in a
      // later statement is the shape that dangles if an operand is ever a
      // temporary (the E0716 fixed in ulua-vm's pusherror.rs, issue #3).
      emit_warning(
        self.context.get(),
        Code::TableOperations,
        node.base.location,
        format_args!("{}", msg),
      );
    } else if let Some(indexer) = &tty_ref.indexer
      && is_string(indexer.index_type)
    {
      let msg = format!("Using '{}' on a table with string keys is likely a bug", op);
      // Pass `format_args!` straight into the call: a `fmt::Arguments`
      // borrows its operands, so storing it in a `let` and using it in a
      // later statement is the shape that dangles if an operand is ever a
      // temporary (the E0716 fixed in ulua-vm's pusherror.rs, issue #3).
      emit_warning(
        self.context.get(),
        Code::TableOperations,
        node.base.location,
        format_args!("{}", msg),
      );
    }
  }
}

// —— 原 methods/lint_table_operations_check_table_call.rs ——
impl<'ctx> LintTableOperations<'ctx> {
  /// cpp `checkTableCall(AstExprCall*, AstExprIndexName*)`：`node`/`func` 为分析期
  /// 存活、由 arena 持有的节点共享借用（cpp 裸指针形参的 Rust 对应），本方法对其
  /// 只读；`args` 元素仍是裸指针，按各自 `// Safety` 说明解引用。
  pub fn check_table_call(&mut self, node: &AstExprCall, func: &AstExprIndexName) {
    let args = node.args.as_slice();
    let index = &func.index;
    // 字符串关键字一次派发为整型 id（match 生成 DFA 跳转），
    // 避免同一名字最多 5 次顺序 strcmp（对照 cpp enum 模式）。
    let op = match index.as_bytes() {
      b"insert" => Some(TableOps::Insert),
      b"remove" => Some(TableOps::Remove),
      b"move" => Some(TableOps::Move),
      b"create" => Some(TableOps::Create),
      _ => None,
    };
    if op == Some(TableOps::Insert) && args.len() == 2 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };
      if let AstExprRef::Call(tail) = arg1.as_expr_ref()
        && let Some(funty) = self.context.get().get_type(tail.func)
      {
        let ret = self.get_return_count(follow_type::follow(funty));
        if ret > 1 {
          warn(
            self.context.get(),
            tail.base.base.location,
            format_args!(
              "table.insert may change behavior if the call returns more than one result; consider adding parentheses around second argument"
            ),
          );
        }
      }
    }
    if op == Some(TableOps::Insert) && args.len() >= 3 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };
      if self.is_constant(args[1], 0.0) {
        warn(
          self.context.get(),
          arg1.base.location,
          format_args!("table.insert uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }
      if self.is_length(args[1], args[0]) {
        warn(
          self.context.get(),
          arg1.base.location,
          format_args!(
            "table.insert will insert the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
          ),
        );
      }
      if let AstExprRef::Binary(add) = arg1.as_expr_ref()
        && add.op == AstExprBinaryOp::Add
        // left/right 已句柄化恒非空；is_length/is_constant 为既有裸指针 API，经 as_ptr 桥接。
        && self.is_length(add.left.as_ptr(), args[0])
        && self.is_constant(add.right.as_ptr(), 1.0)
      {
        warn(
          self.context.get(),
          arg1.base.location,
          format_args!(
            "table.insert will append the value to the table; consider removing the second argument for efficiency"
          ),
        );
      }
    }
    if op == Some(TableOps::Remove) && args.len() >= 2 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };
      if self.is_constant(args[1], 0.0) {
        warn(
          self.context.get(),
          arg1.base.location,
          format_args!("table.remove uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }
      if let AstExprRef::Binary(sub) = arg1.as_expr_ref()
        && sub.op == AstExprBinaryOp::Sub
        // 同上：既有裸指针 API 经 as_ptr 桥接。
        && self.is_length(sub.left.as_ptr(), args[0])
        && self.is_constant(sub.right.as_ptr(), 1.0)
      {
        warn(
          self.context.get(),
          arg1.base.location,
          format_args!(
            "table.remove will remove the value before the last element, which is likely a bug; consider removing the second argument or wrap it in parentheses to silence"
          ),
        );
      }
    }
    if op == Some(TableOps::Move) && args.len() >= 4 {
      if self.is_constant(args[1], 0.0) {
        // SAFETY: args 元素由解析器保证非空。
        let arg1 = unsafe { &*args[1] };
        warn(
          self.context.get(),
          arg1.base.location,
          format_args!("table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      } else if self.is_constant(args[3], 0.0) {
        // SAFETY: args 元素由解析器保证非空。
        let arg3 = unsafe { &*args[3] };
        warn(
          self.context.get(),
          arg3.base.location,
          format_args!("table.move uses index 0 but arrays are 1-based; did you mean 1 instead?"),
        );
      }
    }
    if op == Some(TableOps::Create) && args.len() == 2 {
      // SAFETY: args 元素由解析器保证非空。
      let arg1 = unsafe { &*args[1] };
      if matches!(arg1.as_expr_ref(), AstExprRef::Table(_)) {
        warn(
          self.context.get(),
          arg1.base.location,
          format_args!(
            "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
          ),
        );
      }
      if let AstExprRef::TypeAssertion(assertion) = arg1.as_expr_ref() {
        // assertion.expr 已句柄化恒非空（类型断言必有内层表达式）：.get() 安全借用。
        let inner = assertion.expr.get();
        if matches!(inner.as_expr_ref(), AstExprRef::Table(_)) {
          warn(
            self.context.get(),
            inner.base.location,
            format_args!(
              "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead"
            ),
          );
        }
      }
    }
  }
}
/// `table.` 成员函数关键字 id（替代对同一 AstName 的重复字符串比较）。
#[derive(Clone, Copy, PartialEq, Eq)]
enum TableOps {
  Insert,
  Remove,
  Move,
  Create,
}
/// 统一告警入口（Code 固定为 TableOperations，雷同调用收敛于此）。
fn warn(context: &mut LintContext, location: Location, args: Arguments<'_>) {
  emit_warning(context, Code::TableOperations, location, args);
}

// —— 原 methods/lint_table_operations_get_return_count.rs ——
impl<'ctx> LintTableOperations<'ctx> {
  pub fn get_return_count(&mut self, ty: TypeId) -> usize {
    let ty = follow_type::follow(ty);
    if let Some(ftv) = get_type::get::<FunctionType>(ty) {
      return size(ftv.ret_types, None);
    }
    if let Some(itv) = get_type::get::<IntersectionType>(ty) {
      // We don't process the type recursively to avoid having to deal with
      // self-recursive intersection types
      let mut result = 0;
      for &part in itv.parts.iter() {
        let followed_part = follow_type::follow(part);
        if let Some(ftv) = get_type::get::<FunctionType>(followed_part) {
          let count = size(ftv.ret_types, None);
          result = max(result, count);
        }
      }
      return result;
    }
    0
  }
}

// —— 原 methods/lint_table_operations_is_constant.rs ——
impl<'ctx> LintTableOperations<'ctx> {
  pub(crate) fn is_constant(&mut self, expr: *mut AstExpr, value: f64) -> bool {
    let Some(expr_ref) = (unsafe { expr.as_ref() }) else {
      return false;
    };
    matches!(expr_ref.as_expr_ref(), AstExprRef::ConstantNumber(n) if n.value == value)
  }
}

// —— 原 methods/lint_table_operations_is_length.rs ——
impl<'ctx> LintTableOperations<'ctx> {
  pub(crate) fn is_length(&mut self, expr: *mut AstExpr, table: *mut AstExpr) -> bool {
    let Some(expr_ref) = (unsafe { expr.as_ref() }) else {
      return false;
    };
    if let AstExprRef::Unary(n_ref) = expr_ref.as_expr_ref() {
      // expr 已句柄化；similar 为既有裸指针 API，经 as_ptr 桥接。
      n_ref.op == AstExprUnaryOp::Len && unsafe { similar(n_ref.expr.as_ptr(), table) }
    } else {
      false
    }
  }
}

// —— 原 methods/lint_table_operations_process.rs ——
impl<'ctx> LintTableOperations<'ctx> {
  pub fn process(context: &mut LintContext) {
    if context.module.is_null() {
      return;
    }
    let root = context.root;
    let mut pass = LintTableOperations {
      context: LintContextHandle::from_ref(context),
    };
    // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
    // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
    unsafe {
      ast_stat_visit(root, &mut pass);
    }
  }
}

// —— 原 methods/lint_table_operations_visit_linter.rs ——
impl<'ctx> LintTableOperations<'ctx> {
  pub(crate) fn visit_ast_expr_unary(&mut self, node: *mut AstExprUnary) -> bool {
    let node_ref = unsafe { &*node };
    if node_ref.op == AstExprUnaryOp::Len {
      self.check_indexer(&node_ref.base, node_ref.expr.get(), "#");
    }
    true
  }

  pub(crate) fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
    let node_ref = unsafe { &*node };
    let Some(func_ref) = (unsafe { node_ref.func.as_ref() }) else {
      return true;
    };
    match func_ref.as_expr_ref() {
      AstExprRef::Global(func_global) => {
        if func_global.name == "ipairs" && node_ref.args.len() == 1 {
          let arg0 = node_ref.args.as_slice()[0];
          // SAFETY: arg0 由 parser 保证非空。
          self.check_indexer(&node_ref.base, unsafe { &*arg0 }, "ipairs");
        }
      }
      AstExprRef::IndexName(func_index) => {
        if let AstExprRef::Global(tablib) = func_index.expr.as_expr_ref()
          && tablib.name == "table"
        {
          self.check_table_call(node_ref, func_index);
        }
      }
      _ => {}
    }
    true
  }
}
