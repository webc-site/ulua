// C++ (ControlFlowGraph): `using DefId = NotNull<Definition>;` where
// `Definition = SymDef`. NotNull -> raw pointer.
use crate::records::sym_def::SymDef;
pub type DefId = *mut SymDef;
