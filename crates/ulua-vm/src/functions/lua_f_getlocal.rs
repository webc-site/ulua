use core::ptr;

use crate::{
  functions::c_slice,
  records::{loc_var::LocVar, proto::Proto},
};

/// cpp 调用方可传 nullptr 的查询语义（cpp lfunc.cpp:203）收为 `Option<&Proto>`，体内仅余一处 unsafe。
pub fn lua_f_getlocal(func: Option<&Proto>, mut local_number: i32, pc: i32) -> *const LocVar {
  let Some(func) = func else {
    return ptr::null();
  };

  // Safety: Proto 构造不变式保证 locvars 指向 sizelocvars 个可读、对齐的 LocVar，扫描仅读原型常量区；
  // 返回裸指针寿命随该 Proto
  unsafe {
    for loc in c_slice(func.locvars, func.sizelocvars as usize) {
      if pc >= loc.startpc && pc < loc.endpc {
        local_number -= 1;
        if local_number == 0 {
          return loc as *const LocVar;
        }
      }
    }
  }

  ptr::null()
}
