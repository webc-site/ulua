//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Def.cpp:39:DefArena::freshCell`
//! Source: `Analysis/src/Def.cpp` (Def.cpp:39-42, hand-ported)

use ulua_ast::records::location::Location;

use crate::{
  records::{cell::Cell, def::Def, def_arena::DefArena, symbol::Symbol},
  type_aliases::{def_id_def::DefId, variant::Variant as DefVariant},
};

impl DefArena {
  pub fn fresh_cell(&mut self, sym: Symbol, location: Location, subscripted: bool) -> DefId {
    // NotNull{allocator.allocate(Def{Cell{subscripted}, sym, location})}
    self.allocator.allocate(Def {
      v: DefVariant::V0(Cell { subscripted }),
      name: sym,
      location,
    }) as DefId
  }
}
