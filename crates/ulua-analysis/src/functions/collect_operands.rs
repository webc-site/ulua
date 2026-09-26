use alloc::vec::Vec;

use crate::{
  records::{cell::Cell, def_registry::def_as, phi::Phi},
  type_aliases::def_id_def::DefId,
};

pub(crate) fn collect_operands(def: DefId, operands: &mut Vec<DefId>) {
  // cpp 的 `LUAU_ASSERT(operands)` 判空不变量已由 `&mut Vec` 类型编码，省去。
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
        collect_operands(*operand, operands);
      }
    }
  }
}
