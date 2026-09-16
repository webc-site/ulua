use crate::{
  functions::get_def::get_def_id,
  records::{cell::Cell, phi::Phi},
  type_aliases::def_id_def::DefId,
};

pub(crate) fn contains_subscripted_definition(def: DefId) -> bool {
  unsafe {
    let cell_ptr = get_def_id::<Cell>(def);
    if !cell_ptr.is_null() {
      return (*cell_ptr).subscripted;
    }

    let phi_ptr = get_def_id::<Phi>(def);
    if !phi_ptr.is_null() {
      let phi = &*phi_ptr;
      for &operand in &phi.operands {
        if contains_subscripted_definition(operand) {
          return true;
        }
      }
    }
  }

  false
}
