use crate::enums::host_metamethod::HostMetamethod;

pub type HostUserdataMetamethodBytecodeType =
  Option<unsafe extern "C-unwind" fn(lhs_ty: u8, rhs_ty: u8, method: HostMetamethod) -> u8>;
