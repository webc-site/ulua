use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
  ast_expr_group::AstExprGroup, ast_expr_local::AstExprLocal, ast_local::AstLocal,
  node_handle::OptNode,
};

use crate::{
  records::{arena_handle::alias_ref, cfg_builder::CfgBuilder, symbol::Symbol},
  type_aliases::def_id_control_flow_graph::DefId,
};

impl CfgBuilder {
  pub fn lower_expr_ast_expr(&mut self, expr: *mut AstExpr) {
    // `expr` 槽位仍是裸指针：经句柄门面 `OptNode::from_ptr` 借出基类引用，
    // 判型链全部走句柄上生命周期正确的 `is`/`try_as`（null/类位不命中折叠为
    // false/None，即 cpp `expr->as<T>()` 两段式），借用半径由本栈帧局部句柄
    // 供给，不再锻造假 'static；递归每层由槽位指针重建句柄。
    let expr_node = OptNode::from_ptr(expr);
    if expr_node.is::<AstExprLocal>() {
      self.lower_expr_ast_expr_local(expr.cast::<AstExprLocal>());
    } else if let Some(binop) = expr_node.try_as::<AstExprBinary>() {
      // left/right 已句柄化恒非空，cpp 的 LUAU_ASSERT(!=nullptr) 由类型端兑现；
      // lower_expr_ast_expr 为既有裸指针 API，经 as_ptr 桥接。
      self.lower_expr_ast_expr(binop.left.as_ptr());
      self.lower_expr_ast_expr(binop.right.as_ptr());
    } else if let Some(call) = expr_node.try_as::<AstExprCall>() {
      // C++ `else if (auto call = expr->as<AstExprCall>()) lowerExpr(call);`
      self.lower_expr_ast_expr_call(call);
    } else if let Some(group) = expr_node.try_as::<AstExprGroup>() {
      // C++ `else if (auto group = expr->as<AstExprGroup>()) lowerExpr(group->expr);`
      // expr 已句柄化恒非空，cpp 的 LUAU_ASSERT(!=nullptr) 由类型端兑现；
      // lower_expr_ast_expr 为既有裸指针 API，经 as_ptr 桥接。
      self.lower_expr_ast_expr(group.expr.as_ptr());
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
  pub(crate) fn lower_expr_ast_expr_call(&mut self, call: &AstExprCall) {
    // if (tryLowerAssertion(call)) return;
    if self.try_lower_assertion(call) {
      return;
    }

    // lowerExpr(call->func);
    self.lower_expr_ast_expr(call.func);

    // for (size_t i = 0; i < call->args.size; i++) lowerExpr(call->args.data[i]);
    for &arg in call.args.iter() {
      self.lower_expr_ast_expr(arg);
    }
  }
}

/// AST 句柄只读探针：解引用收口进私有 helper、公开方法保持 safe（同
/// `clone_clone::type_is_persistent` 写法）。local 由分派方保证指向 parser
/// arena 存活节点（块地址不移动），仅拷贝指针字段。
fn local_binding(local: *mut AstExprLocal) -> *mut AstLocal {
  alias_ref(local).local.as_ptr()
}
