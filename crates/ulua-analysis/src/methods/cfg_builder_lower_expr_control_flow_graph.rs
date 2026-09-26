use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_binary::AstExprBinary,
    ast_expr_call::AstExprCall, ast_expr_group::AstExprGroup, ast_expr_local::AstExprLocal,
    ast_local::AstLocal,
  },
  rtti::{AstNodePtr, ast_node_is_ptr, ast_node_try_as_ptr},
};

use crate::{
  records::{cfg_builder::CfgBuilder, symbol::Symbol},
  type_aliases::def_id_control_flow_graph::DefId,
};

impl CfgBuilder {
  pub fn lower_expr_ast_expr(&mut self, expr: *mut AstExpr) {
    unsafe {
      // Safety: expr 由 parser 及递归前置的 LUAU_ASSERT 保证为非空 arena AST 节点；
      // 判型+判空折叠为 ast_node_try_as_ptr/ast_node_is Option 门面（cpp `expr->as<T>()`
      // 两段式），binop/group 字段读取走安全引用并在 LUAU_ASSERT 之后；转调的
      // 转调的 lower_expr_* 已降 safe，仅收守卫命中的 repr(C) 基址重合下转指针，
      // 单线程 &mut self 独占。
      if ast_node_is_ptr::<AstExprLocal>(expr.as_ast_node()) {
        self.lower_expr_ast_expr_local(expr.cast::<AstExprLocal>());
      } else if let Some(binop) = ast_node_try_as_ptr::<AstExprBinary>(expr.as_ast_node()) {
        // left/right 已句柄化恒非空，cpp 的 LUAU_ASSERT(!=nullptr) 由类型端兑现；
        // lower_expr_ast_expr 为既有裸指针 API，经 as_ptr 桥接。
        self.lower_expr_ast_expr(binop.left.as_ptr());
        self.lower_expr_ast_expr(binop.right.as_ptr());
      } else if ast_node_is_ptr::<AstExprCall>(expr.as_ast_node()) {
        // C++ `else if (auto call = expr->as<AstExprCall>()) lowerExpr(call);`
        self.lower_expr_ast_expr_call(expr.cast::<AstExprCall>());
      } else if let Some(group) = ast_node_try_as_ptr::<AstExprGroup>(expr.as_ast_node()) {
        // C++ `else if (auto group = expr->as<AstExprGroup>()) lowerExpr(group->expr);`
        // expr 已句柄化恒非空，cpp 的 LUAU_ASSERT(!=nullptr) 由类型端兑现；
        // lower_expr_ast_expr 为既有裸指针 API，经 as_ptr 桥接。
        self.lower_expr_ast_expr(group.expr.as_ptr());
      }
    }
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:362-366` (hand-ported)
// C++ `void CFGBuilder::lowerExpr(AstExprLocal* local)`.
impl CfgBuilder {
  /// 对应 C++ `CFGBuilder::lowerExpr(AstExprLocal*)`（`Analysis/src/ControlFlowGraph.cpp:362-366`）。
  /// 降 safe：`local` 只做一次字段读取（`local->local`），AST arena 节点地址不移动；
  /// `use_defs` 以裸指针为键属 CFG 既有写穿协议，解引用收进窄 `unsafe` 块。
  /// `self.cfg` 由 builder 构造接线为 `Some` 且比 builder 长寿。
  pub(crate) fn lower_expr_ast_expr_local(&mut self, local: *mut AstExprLocal) {
    // Safety: local 由分派方 lower_expr_ast_expr 保证指向存活 AstExprLocal（parser
    // arena 块地址不移动），解引用读其 local 字段有效；cfg.as_mut().unwrap() 取得
    // 存活 ControlFlowGraph 的 use_defs 可变借用，单线程内无别名。
    // C++:
    //   DefId def = readVariable(currentBlock, Symbol(local->local));
    //   cfg->useDefs[local] = def;
    // read_variable 自 §2 续起全链收发 u32 句柄（block/Join 经各自注册表解析），
    // 对调用方为 safe。
    let sym = Symbol::from_local(local_binding(local));
    let def: DefId = self.read_variable(self.current_block, sym);
    // useDefs is keyed by `AstExpr*`; `AstExprLocal*` upcasts to it.
    let key = local.cast::<AstExpr>();
    *self
      .cfg
      .as_mut()
      .expect("CFG 构造期接线 cfg 为 Some 且比 builder 长寿，取回必命中")
      .use_defs
      .get_or_insert(key) = def;
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:470-477` (hand-ported)
// C++ `void CFGBuilder::lowerExpr(AstExprCall* call)`.
impl CfgBuilder {
  /// 对应 C++ `CFGBuilder::lowerExpr(AstExprCall*)`（`Analysis/src/ControlFlowGraph.cpp:470-477`）。
  /// 降 safe：`call` 只读 func/args 子指针（parser arena，地址不移动且于整轮
  /// lower 存活）后递归，解引用收进窄 `unsafe` 块；`try_lower_assertion` 沿用同一
  /// 存活 `AstExprCall` 句柄，前提在块内一并证成。
  pub(crate) fn lower_expr_ast_expr_call(&mut self, call: *mut AstExprCall) {
    // Safety: call 由分派方保证指向存活 AstExprCall，读取其 func 与 args（parser 保证
    // 各子表达式指针非空）后递归 lower_expr_ast_expr，全程单线程 &mut self 独占、无别名。
    // if (tryLowerAssertion(call)) return;
    if unsafe { self.try_lower_assertion(call) } {
      return;
    }

    // lowerExpr(call->func);
    let func = call_func(call);
    self.lower_expr_ast_expr(func);

    // for (size_t i = 0; i < call->args.size; i++) lowerExpr(call->args.data[i]);
    let args = call_args(call);
    for &arg in args.iter() {
      self.lower_expr_ast_expr(arg);
    }
  }
}

/// AST 句柄只读探针：解引用收口进私有 helper、公开方法保持 safe（同
/// `clone_clone::type_is_persistent` 写法）。local/call 由分派方保证指向 parser
/// arena 存活节点（块地址不移动），仅拷贝指针字段。
fn local_binding(local: *mut AstExprLocal) -> *mut AstLocal {
  // SAFETY: 见函数 doc。local 槽已句柄化恒非空，经 as_ptr 还原裸指针身份。
  unsafe { (*local).local.as_ptr() }
}

fn call_func(call: *mut AstExprCall) -> *mut AstExpr {
  // SAFETY: 见函数 doc。
  unsafe { (*call).func }
}

fn call_args(call: *mut AstExprCall) -> AstArray<*mut AstExpr> {
  // SAFETY: 见函数 doc。
  unsafe { (*call).args }
}
