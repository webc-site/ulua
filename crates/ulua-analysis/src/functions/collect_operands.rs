use alloc::vec::Vec;

use crate::{
  records::{cell::Cell, def_registry::def_as, phi::Phi},
  type_aliases::def_id_def::DefId,
};

/// 收集 def 的全部操作数（单 def 惯用返回值形态，替代 C 风格出参，review.md §3）。
pub(crate) fn collect_operands(def: DefId) -> Vec<DefId> {
  let mut operands = Vec::new();
  collect_operands_into(def, &mut operands);
  operands
}

/// 收集 def 的全部操作数并追加到已有缓冲（供多 def 批量收集复用）。
pub(crate) fn collect_operands_into(def: DefId, operands: &mut Vec<DefId>) {
  if operands.contains(&def) {
    return;
  }

  if def_as::<Cell>(def).is_some() {
    operands.push(def);
  } else if let Some(phi) = def_as::<Phi>(def) {
    if phi.operands.is_empty() {
      operands.push(def);
    } else {
      for operand in &phi.operands {
        collect_operands_into(*operand, operands);
      }
    }
  }
}
