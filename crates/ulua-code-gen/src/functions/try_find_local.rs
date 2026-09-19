use core::{ptr::null, slice::from_raw_parts};

#[repr(C)]
pub struct Proto {
  pub sizelocvars: i32,
  pub locvars: *const LocVar,
}

#[repr(C)]
pub struct LocVar {
  pub reg: i32,
  pub startpc: i32,
  pub endpc: i32,
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn try_find_local(proto: *const Proto, reg: i32, pcpos: i32) -> *const LocVar {
  let proto = unsafe { &*proto };
  // 切片迭代，命中即返回，免去逐次索引与越界检查
  let locals = unsafe { from_raw_parts(proto.locvars, proto.sizelocvars.max(0) as usize) };
  locals
    .iter()
    .find(|l| reg == l.reg && pcpos >= l.startpc && pcpos < l.endpc)
    .map_or(null::<LocVar>(), |l| l as *const LocVar)
}
