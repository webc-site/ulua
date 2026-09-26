use crate::{
  records::{cell::Cell, def_registry::def_as, phi::Phi},
  type_aliases::def_id_def::DefId,
};

pub(crate) fn contains_subscripted_definition(def: DefId) -> bool {
  if let Some(cell) = def_as::<Cell>(def) {
    return cell.subscripted;
  }

  if let Some(phi) = def_as::<Phi>(def) {
    for operand in &phi.operands {
      if contains_subscripted_definition(*operand) {
        return true;
      }
    }
  }

  false
}
