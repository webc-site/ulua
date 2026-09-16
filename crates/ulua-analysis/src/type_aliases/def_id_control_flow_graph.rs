// C++ (ControlFlowGraph): `using DefId = NotNull<Definition>;` where
// `Definition = SymDef`. NotNull -> raw pointer.
use crate::type_aliases::definition::Definition;
pub type DefId = *mut Definition;
