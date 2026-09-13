use core::ffi::c_int;

use crate::type_aliases::stk_id::StkId;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct CallS {
  pub(crate) func: StkId,
  pub(crate) nresults: c_int,
}
