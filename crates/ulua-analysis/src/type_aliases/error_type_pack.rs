//! Source: `Analysis/include/Luau/TypePack.h:63` — `using ErrorTypePack = Unifiable::Error<TypePackId>` (hand-ported)

use crate::{records::unifiable::Error, type_aliases::type_pack_id::TypePackId};
pub type ErrorTypePack = Error<TypePackId>;
