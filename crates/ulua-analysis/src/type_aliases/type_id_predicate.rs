use alloc::boxed::Box;

use crate::type_aliases::type_id::TypeId;
pub type TypeIdPredicate = Box<dyn Fn(TypeId) -> Option<TypeId>>;
