//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/TypePack.h:63:error_type_pack`
//! Source: `Analysis/include/Luau/TypePack.h:63` — `using ErrorTypePack = Unifiable::Error<TypePackId>` (hand-ported)

use crate::{records::unifiable::Error, type_aliases::type_pack_id::TypePackId};
pub type ErrorTypePack = Error<TypePackId>;
