extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    dump_def::dump_def,
    dump_expr::dump_expr,
    dump_refinement::dump_refinement,
    find_rhs_expr_dump_cfg::{
      find_rhs_expr_symbol_ast_stat_assign, find_rhs_expr_symbol_ast_stat_local,
    },
  },
  records::{
    instr_registry::resolve_instruction, sym_def_registry::resolve_sym_def, symbol::Symbol,
  },
  type_aliases::{def_id_control_flow_graph::DefId, instr_id::InstrId, instruction::Instruction},
};

/// 句柄 → 存活 `SymDef` 的解析：def 句柄均由 `sym_def_registry` 在本 CFG 构建
/// 期发放（契约见该模块头），转储期内 arena 节点存活，未命中即注册表契约被
/// 破坏（cpp 侧对应悬垂/非空解引用 UB，expect 比 UB 保守）。
fn sym_of(def: DefId) -> Symbol {
  resolve_sym_def(def)
    .expect("DefId 为本次构建期 register_sym_def 发放的存活句柄")
    .sym
    .clone()
}

/// 对应 C++ `std::string dumpInstruction(InstrId inst, const DenseHashMap<AstExpr*, DefId>& uses)`
/// (`cpp/Analysis/src/DumpCFG.cpp`)。`inst` 自 §2 续起为 `InstrId` u32 句柄：
/// 解引用收口在 `instr_registry`（见该模块契约），转储期内句柄均由本构建期
/// `register_instruction` 发放，解析未命中即契约被破坏（cpp 侧对应悬垂
/// `NotNull` 解引用 UB，expect 比 UB 保守），本函数对调用方为 safe。
pub fn dump_instruction(inst: InstrId, use_defs: &DenseHashMap<*mut AstExpr, DefId>) -> String {
  let inst =
    resolve_instruction(inst).expect("InstrId 为本次构建期 register_instruction 发放的存活句柄");
  // Safety: inst 由注册表回到 Block 独占的指令 arena 节点，在转储期间存活且块地址不
  // 移动（instr_registry 契约）；decl.source/assign.source 是构造期接线的 parser
  // arena 存活节点地址（块地址不移动），只读解引用供 find_rhs_expr_*；
  // decl.def/assign.def 与 join.operands 是 CFG 构建期为每条 Declare/Assign/Join
  // 经 register_sym_def 发放的 SymDef 句柄（只读经 resolve_sym_def 回到节点，见
  // sym_def_registry 契约），其 sym 只读克隆；
  // flow.prop 是 NotNull `RefinementId`（Handle），`get()` 物化 RefinementArena
  // 存活节点的共享只读借用；find_rhs_expr_* 的
  // 可空返回值先判空才交 dump_expr。全程单线程只读，无别名冲突。
  unsafe {
    match inst {
      Instruction::Declare(decl) => {
        let mut result = format!("local {}", dump_def(decl.def));
        let rhs = find_rhs_expr_symbol_ast_stat_local(sym_of(decl.def), &*decl.source);
        if !rhs.is_null() {
          result.push_str(" = ");
          result.push_str(&dump_expr(rhs, use_defs));
        }
        result
      }
      Instruction::Assign(assign) => {
        let mut result = dump_def(assign.def);
        let rhs = find_rhs_expr_symbol_ast_stat_assign(sym_of(assign.def), &*assign.source);
        if !rhs.is_null() {
          result.push_str(" = ");
          result.push_str(&dump_expr(rhs, use_defs));
        }
        result
      }
      Instruction::Join(join) => {
        let mut result = format!("{} = join(", dump_def(join.definition));
        for (i, operand) in join.operands.iter().enumerate() {
          if i > 0 {
            result.push_str(", ");
          }
          result.push_str(&dump_def(*operand));
        }
        result.push(')');
        result
      }
      Instruction::Refine(flow) => {
        format!(
          "{} = refine({})",
          dump_def(flow.definition),
          dump_refinement(flow.prop.get())
        )
      }
    }
  }
}
