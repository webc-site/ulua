use alloc::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_def::get_def_id,
  records::{cell::Cell, phi::Phi},
  type_aliases::def_id_def::DefId,
};

pub(crate) fn collect_operands(def: DefId, operands: &mut Vec<DefId>) {
  unsafe {
    LUAU_ASSERT!(operands as *const Vec<DefId> as *const () as usize != 0);

    if operands.contains(&def) {
      return;
    }

    if !get_def_id::<Cell>(def).is_null() {
      operands.push(def);
    } else if !get_def_id::<Phi>(def).is_null() {
      let phi = get_def_id::<Phi>(def);
      if (*phi).operands.is_empty() {
        operands.push(def);
      } else {
        for &operand in (*phi).operands.iter() {
          collect_operands(operand, operands);
        }
      }
    }
  }
}
