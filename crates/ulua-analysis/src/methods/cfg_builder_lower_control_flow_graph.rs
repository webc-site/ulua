use alloc::string::ToString;
use core::ptr::{from_ref, null_mut};

use ulua_ast::{
  records::{
    ast_stat::AstStat, ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock,
    ast_stat_expr::AstStatExpr, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_while::AstStatWhile,
  },
  rtti::{AstNodePtr, ast_node_try_as},
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::block_kind::BlockKind,
  functions::extract_l_value_symbol::extract_l_value_symbol,
  records::{
    arena_handle::alias_ref, assign::Assign, block_registry::resolve_block_mut,
    cfg_builder::CfgBuilder, declare::Declare, symbol::Symbol,
  },
  type_aliases::{
    block_id::BlockId, def_id_control_flow_graph::DefId,
    refinement_id_control_flow_graph::RefinementId,
  },
};

impl CfgBuilder {
  /// cpp `CFGBuilder::lower(AstStat*)`（ControlFlowGraph.cpp:233）。
  ///
  /// 本分派链唯一的裸指针物化收口点：一次 `&*` 把 arena 节点换成 `&AstNode`，
  /// 之后各子类判定与下转全部走安全门面 `ast_node_try_as`，被调方以引用形参
  /// 拿到存活证明，不再有裸指针契约。
  fn lower_ast_stat(&mut self, statement: *mut AstStat) {
    let node = alias_ref(statement.as_ast_node());

    if let Some(block) = ast_node_try_as::<AstStatBlock>(node) {
      self.lower_ast_stat_block(block);
    } else if let Some(local) = ast_node_try_as::<AstStatLocal>(node) {
      self.lower_ast_stat_local(local);
    } else if let Some(assn) = ast_node_try_as::<AstStatAssign>(node) {
      self.lower_ast_stat_assign(assn);
    } else if let Some(stat_if) = ast_node_try_as::<AstStatIf>(node) {
      self.lower_ast_stat_if(stat_if);
    } else if let Some(stat_while) = ast_node_try_as::<AstStatWhile>(node) {
      self.lower_ast_stat_while(stat_while);
    } else if let Some(stat_expr) = ast_node_try_as::<AstStatExpr>(node) {
      self.lower_ast_stat_expr(stat_expr);
    } else {
      LUAU_ASSERT!(false);
    }
  }

  /// cpp `CFGBuilder::lower(AstStatBlock*)`（ControlFlowGraph.cpp:281）：
  /// 逐条 lower 块内语句。`statement` 为存活 arena 节点（引用即证明）。
  pub fn lower_ast_stat_block(&mut self, statement: &AstStatBlock) {
    for stat in statement.body.iter_nodes() {
      self.lower_ast_stat(stat.as_ptr());
    }
  }

  /// cpp `CFGBuilder::lower(AstStatLocal*)`（ControlFlowGraph.cpp:287）：
  /// 为每个 `AstLocal` 发 `Declare` 指令。`values` 短于 `vars` 时对应初始化
  /// 表达式缺省（cpp 以 `i < values.size` 判定），此处折叠为 null 判空。
  fn lower_ast_stat_local(&mut self, local: &AstStatLocal) {
    let values = local.values.as_slice();

    for (i, &loc) in local.vars.as_slice().iter().enumerate() {
      // 既有约定（review.md §2）：expr 为 `*mut AstExpr` 身份面，空 = 该 name 无对应初始值
      // （cpp `values.data[i] ? : nullptr` 同型），仅由紧邻的 `is_null` 守卫消费，本批次不改元素类型。
      let expr = values.get(i).copied().unwrap_or(null_mut());

      if !expr.is_null() {
        self.lower_expr_ast_expr(expr);
      }

      // C++:
      //   Symbol sym(loc);
      //   DefId def = newDefinition(sym);
      //   emit<Declare>(currentBlock, def, local);
      //   currentBlock->setReachingDefinition(sym, def);
      let sym = Symbol::from_local(loc);
      let def: DefId = self.new_definition(sym.clone());
      let current_block = self.current_block;
      // Declare.source 只作指令溯源身份（dump_cfg 的 find_rhs_expr 只读消费），
      // 从存活引用还原的地址即原 arena 节点地址，语义与 cpp 存 `local` 指针一致。
      self.emit::<Declare, _>(current_block, (def, from_ref(local).cast_mut()));
      // `current_block` 为构建期 register_block 发放的存活句柄（NotNull 语义），
      // 经注册表解析写回（见 `block_registry` 模块契约），单线程独占写。
      resolve_block_mut(current_block)
        .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
        .set_reaching_definition(sym, def);
    }
  }

  /// cpp `CFGBuilder::lower(AstStatAssign*)`（ControlFlowGraph.cpp:315）：
  /// 先 lower 右值，再为每个 lvalue 目标发 `Assign` 指令。
  fn lower_ast_stat_assign(&mut self, assn: &AstStatAssign) {
    for &expr in assn.values.as_slice() {
      self.lower_expr_ast_expr(expr);
    }

    for &target in assn.vars.as_slice() {
      // C++:
      //   if (auto sym = extractLValueSymbol(target)) {
      //       DefId def = newDefinition(*sym);
      //       emit<Assign>(currentBlock, def, assn);
      //       currentBlock->setReachingDefinition(*sym, def);
      //   } else LUAU_ASSERT(!"Unhandled lvalue type");
      // SAFETY: `target` 取自存活父节点 `assn.vars`，parser 保证非空的 arena
      // 表达式节点；只读借用供 extract_l_value_symbol 判别。
      if let Some(sym) = extract_l_value_symbol(alias_ref(target)) {
        let def = self.new_definition(sym.clone());
        let current_block = self.current_block;
        // Assign.source 同 Declare.source：仅溯源身份，只读消费。
        self.emit::<Assign, _>(current_block, (def, from_ref(assn).cast_mut()));
        // 同 `lower_ast_stat_local`——句柄经注册表解析写回，单线程独占写。
        resolve_block_mut(current_block)
          .expect("BlockId 为本次构建期 register_block 发放的存活句柄")
          .set_reaching_definition(sym, def);
      } else {
        LUAU_ASSERT!(false);
      }
    }
  }

  /// cpp `CFGBuilder::lower(AstStatIf*)`（ControlFlowGraph.cpp:363）：
  /// then/else 双分支 + merge 块的接线。`stat_if.elsebody` 可缺省
  /// （`AstStatIf` 的空指针字段），使用前判空。
  pub fn lower_ast_stat_if(&mut self, stat_if: &AstStatIf) {
    // Block* currBlock = currentBlock.get();
    let curr_block: BlockId = self.current_block;

    // Block* thenBlock = newBlock(BlockKind::Linear, "then branch", currBlock);
    let then_block = self.new_block(
      BlockKind::Linear,
      "then branch".to_string(),
      Some(curr_block),
    );
    // auto ref = resolveCondition(statIf->condition);
    let ref_opt = self.resolve_condition(stat_if.condition.as_ptr());
    // if (ref) emitRefineInstruction(thenBlock, *ref);
    if let Some(r) = ref_opt {
      self.emit_refine_instruction(then_block, r);
    }

    // Then only has one predecessor
    // seal(thenBlock);
    self.seal(then_block);
    // Block* thenExit;
    // { BlockScope scope(*this, thenBlock); lower(statIf->thenbody); thenExit = currentBlock.get(); }
    let then_exit: BlockId = {
      let saved = self.current_block;
      self.block_scope_cfg_builder_block(then_block);
      self.lower_ast_stat_block(&stat_if.thenbody);
      let exit = self.current_block;
      self.current_block = saved; // ~BlockScope restores currentBlock
      exit
    };

    // Else branch (may be nullptr, another AstStatIf for elseif, or a block)
    // Block* elseBlock = newBlock(BlockKind::Linear, "else branch", currBlock);
    let else_block = self.new_block(
      BlockKind::Linear,
      "else branch".to_string(),
      Some(curr_block),
    );
    // Block* elseExit = elseBlock; // If there is an else body, overwrite this
    let mut else_exit: BlockId = else_block;
    // seal(elseBlock);
    self.seal(else_block);
    // if (ref) emitRefineInstruction(elseBlock, allocator->refinementArena.negation(*ref));
    if let Some(r) = ref_opt {
      let neg = self.allocator.get_mut().refinement_arena.negation_mut(r);
      self.emit_refine_instruction(else_block, neg);
    }

    // if (statIf->elsebody) { BlockScope scope(*this, elseBlock); lower(statIf->elsebody); elseExit = currentBlock.get(); }
    if stat_if.elsebody.is_some() {
      let saved = self.current_block;
      self.block_scope_cfg_builder_block(else_block);
      self.lower_ast_stat(stat_if.elsebody.as_ptr());
      else_exit = self.current_block;
      self.current_block = saved; // ~BlockScope restores currentBlock
    }

    // Merge block — all paths converge here
    // Block* mergeBlock = newBlock(BlockKind::Linear, "merge");
    let merge_block = self.new_block(BlockKind::Linear, "merge".to_string(), None);
    // thenExit->addSuccessor(mergeBlock);
    then_exit.add_successor(merge_block);
    // elseExit->addSuccessor(mergeBlock);
    else_exit.add_successor(merge_block);
    // seal(mergeBlock);
    self.seal(merge_block);
    // currentBlock = NotNull{mergeBlock};
    self.current_block = merge_block;
  }

  /// cpp `CFGBuilder::lower(AstStatWhile*)`（ControlFlowGraph.cpp:406）：
  /// 回边 loop header + body + exit 块的接线；`condition`/`body` 由 parser
  /// 保证非空（随 `&AstStatWhile` 引用一并成立）。
  pub fn lower_ast_stat_while(&mut self, stat_while: &AstStatWhile) {
    // Block* preLoop = currentBlock.get();
    let pre_loop: BlockId = self.current_block;

    // Loop Header — receives the back-edge so we don't seal it yet. Resolve the
    // condition inside the Header's scope so reads of variables mutated in the
    // body hit this unsealed block, emit an incomplete Join, and get their
    // operands filled in when the Header is sealed after the back-edge.
    // Block* loopHeader = newBlock(BlockKind::Condition, "while-loop condition", preLoop);
    let loop_header = self.new_block(
      BlockKind::Condition,
      "while-loop condition".to_string(),
      Some(pre_loop),
    );
    // std::optional<RefinementId> ref;
    // { BlockScope scope(*this, loopHeader); ref = resolveCondition(statWhile->condition); }
    let ref_opt: Option<RefinementId> = {
      let saved = self.current_block;
      self.block_scope_cfg_builder_block(loop_header);
      let r = self.resolve_condition(stat_while.condition.as_ptr());
      self.current_block = saved; // ~BlockScope restores currentBlock
      r
    };

    // Block* bodyBlock = newBlock(BlockKind::Linear, "while-loop body", loopHeader);
    let body_block = self.new_block(
      BlockKind::Linear,
      "while-loop body".to_string(),
      Some(loop_header),
    );
    // if (ref) emitRefineInstruction(bodyBlock, *ref);
    if let Some(r) = ref_opt {
      self.emit_refine_instruction(body_block, r);
    }
    // seal(bodyBlock);
    self.seal(body_block);
    // Block* bodyExit;
    // { BlockScope scope(*this, bodyBlock); lower(statWhile->body); bodyExit = currentBlock.get(); }
    let body_exit: BlockId = {
      let saved = self.current_block;
      self.block_scope_cfg_builder_block(body_block);
      self.lower_ast_stat_block(&stat_while.body);
      let exit = self.current_block;
      self.current_block = saved; // ~BlockScope restores currentBlock
      exit
    };

    // You can seal the loop Header now because no predecessors will be added to it.
    // bodyExit->addSuccessor(loopHeader);
    body_exit.add_successor(loop_header);
    // seal(loopHeader);
    self.seal(loop_header);

    // Block* exitBlock = newBlock(BlockKind::Linear, "while-loop exit", loopHeader);
    let exit_block = self.new_block(
      BlockKind::Linear,
      "while-loop exit".to_string(),
      Some(loop_header),
    );
    // if (ref) emitRefineInstruction(exitBlock, allocator->refinementArena.negation(*ref));
    if let Some(r) = ref_opt {
      let neg = self.allocator.get_mut().refinement_arena.negation_mut(r);
      self.emit_refine_instruction(exit_block, neg);
    }
    // seal(exitBlock);
    self.seal(exit_block);
    // currentBlock = NotNull{exitBlock};
    self.current_block = exit_block;
  }

  /// cpp `CFGBuilder::lower(AstStatExpr*)`（ControlFlowGraph.cpp:253）：
  /// 表达式语句直接 lower 其 `expr` 子节点（parser 恒填充非空）。
  fn lower_ast_stat_expr(&mut self, stat: &AstStatExpr) {
    // lowerExpr(stat->expr);（expr 已句柄化，lower 家族仍以 arena 裸指针身份为键）
    self.lower_expr_ast_expr(stat.expr.as_ptr());
  }
}
