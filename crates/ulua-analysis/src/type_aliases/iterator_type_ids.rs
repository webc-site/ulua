use core::slice::IterMut;

use crate::type_aliases::type_id::TypeId;
pub type Iterator = IterMut<'static, TypeId>;
