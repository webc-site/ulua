//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Def.cpp:49:DefArena::phi`
//! Source: `Analysis/src/Def.cpp` (Def.cpp:49-60, hand-ported)

use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  functions::collect_operands::collect_operands,
  records::{def::Def, def_arena::DefArena, phi::Phi, symbol::Symbol},
  type_aliases::{def_id_def::DefId, variant::Variant as DefVariant},
};

impl DefArena {
  pub fn phi_vector_def_id(&mut self, defs: &[DefId]) -> DefId {
    let mut operands: Vec<DefId> = Vec::new();
    for &operand in defs.iter() {
      collect_operands(operand, &mut operands);
    }

    // There's no need to allocate a Phi node for a singleton set.
    if operands.len() == 1 {
      operands[0]
    } else {
      self.allocator.allocate(Def {
        v: DefVariant::V1(Phi { operands }),
        name: Symbol::default(),
        location: Location::default(),
      }) as DefId
    }
  }
}
