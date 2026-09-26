use core::ptr::null;

use crate::{
  functions::c_slice,
  records::{loc_var::LocVar, proto::Proto},
};

/// 指针形参收为 `&Proto`（非空/对齐由引用保证），体内仅余一处 unsafe（cpp lfunc.cpp:218）。
pub fn lua_f_findlocal(f: &Proto, local_reg: i32, pc: i32) -> *const LocVar {
  // Safety: Proto 构造不变式保证 locvars 指向 sizelocvars 个可读、对齐的 LocVar，数组随 Proto 存活；
  // 返回裸指针寿命随该 Proto，pc/reg 为值型无额外前提
  unsafe {
    // cpp lfunc.cpp:61 线性扫描 locvars；c_slice 统一处理 null+0 长守卫
    for locvar in c_slice(f.locvars, f.sizelocvars as usize) {
      if local_reg == locvar.reg as i32 && pc >= locvar.startpc && pc < locvar.endpc {
        return locvar as *const LocVar;
      }
    }

    null()
  }
}
