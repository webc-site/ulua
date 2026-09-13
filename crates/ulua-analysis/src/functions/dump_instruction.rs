extern crate alloc;

use alloc::string::String;

use ulua_ast::records::ast_expr::AstExpr;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    dump_def::dump_def, dump_expr::dump_expr, dump_refinement::dump_refinement,
    find_rhs_expr_dump_cfg::find_rhs_expr_symbol_ast_stat_local,
    find_rhs_expr_dump_cfg_alt_b::find_rhs_expr_symbol_ast_stat_assign,
  },
  type_aliases::{definition::Definition, instruction::Instruction},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn dump_instruction(
  inst: *const Instruction,
  use_defs: &DenseHashMap<*mut AstExpr, *mut Definition>,
) -> String {
  unsafe {
    match &*inst {
      Instruction::Declare(decl) => {
        let mut result = format!("local {}", dump_def(decl.def));
        let rhs = find_rhs_expr_symbol_ast_stat_local((*decl.def).sym.clone(), decl.source);
        if !rhs.is_null() {
          result.push_str(" = ");
          result.push_str(&dump_expr(rhs, use_defs));
        }
        result
      }
      Instruction::Assign(assign) => {
        let mut result = dump_def(assign.def);
        let rhs = find_rhs_expr_symbol_ast_stat_assign((*assign.def).sym.clone(), assign.source);
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
          dump_refinement(&*flow.prop)
        )
      }
    }
  }
}
