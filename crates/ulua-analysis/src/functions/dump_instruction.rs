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
  match inst {
    Instruction::Declare(decl) => {
      let mut result = format!("local {}", dump_def(decl.def));
      let source = unsafe { decl.source.as_ref() };
      if let Some(source) = source
        && let Some(rhs) = find_rhs_expr_symbol_ast_stat_local(sym_of(decl.def), source)
      {
        result.push_str(" = ");
        result.push_str(&dump_expr(rhs, use_defs));
      }
      result
    }
    Instruction::Assign(assign) => {
      let mut result = dump_def(assign.def);
      let source = unsafe { assign.source.as_ref() };
      if let Some(source) = source
        && let Some(rhs) = find_rhs_expr_symbol_ast_stat_assign(sym_of(assign.def), source)
      {
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
