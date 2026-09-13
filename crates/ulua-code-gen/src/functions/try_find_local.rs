use core::ptr::null;
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
  unsafe {
    let proto = &*proto;

    for i in 0..proto.sizelocvars {
      let local = &*proto.locvars.add(i as usize);

      if reg == local.reg && pcpos >= local.startpc && pcpos < local.endpc {
        return local as *const LocVar;
      }
    }

    null::<LocVar>()
  }
}
