use core::ptr::NonNull;

use crate::records::def::Def;
pub type DefId = NonNull<*const Def>;
