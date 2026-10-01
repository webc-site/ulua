//! `def_arena` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  functions::collect_operands::collect_operands_into,
  records::{
    cell::Cell, def::Def, def_arena::DefArena, def_registry::register_def, phi::Phi, symbol::Symbol,
  },
  type_aliases::{def_id_def::DefId, variant::Variant as DefVariant},
};

// Source: `Analysis/src/Def.cpp` (Def.cpp:39-42, hand-ported)

impl DefArena {
  pub fn fresh_cell(&mut self, sym: Symbol, location: Location, subscripted: bool) -> DefId {
    // NotNull{allocator.allocate(Def{Cell{subscripted}, sym, location})}
    register_def(self.allocator.allocate(Def {
      v: DefVariant::V0(Cell { subscripted }),
      name: sym,
      location,
    }))
  }
}

impl DefArena {
  pub fn phi_def_id_def_id(&mut self, a: DefId, b: DefId) -> DefId {
    self.phi_vector_def_id(&[a, b])
  }

  pub fn phi_vector_def_id(&mut self, defs: &[DefId]) -> DefId {
    let mut operands: Vec<DefId> = Vec::new();
    for &operand in defs.iter() {
      collect_operands_into(operand, &mut operands);
    }

    // There's no need to allocate a Phi node for a singleton set.
    if operands.len() == 1 {
      operands[0]
    } else {
      register_def(self.allocator.allocate(Def {
        v: DefVariant::V1(Phi { operands }),
        name: Symbol::default(),
        location: Location::default(),
      }))
    }
  }
}
