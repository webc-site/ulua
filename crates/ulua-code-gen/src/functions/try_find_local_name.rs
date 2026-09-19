use core::{ffi::CStr, slice::from_raw_parts};

use ulua_vm::{macros::getstr::getstr, records::proto::Proto};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn try_find_local_name<'a>(
  proto: *const Proto,
  reg: i32,
  pcpos: i32,
) -> Option<&'a str> {
  unsafe {
    // cpp 的顺序扫描在 locvars==nullptr && sizelocvars==0 时不迭代；
    // debug 构建的 -Zub-checks 禁止 null 起点切片，显式短路对齐 C++ 语义。
    if (*proto).locvars.is_null() {
      return None;
    }
    let locals = from_raw_parts((*proto).locvars, (*proto).sizelocvars.max(0) as usize);

    // 命中首个匹配即返回（对齐 cpp 顺序扫描语义）
    let local = locals
      .iter()
      .find(|l| reg == l.reg as i32 && pcpos >= l.startpc && pcpos < l.endpc)?;

    if local.varname.is_null() {
      return None;
    }
    let s = getstr(local.varname);
    if s.is_null() {
      None
    } else {
      Some(CStr::from_ptr(s).to_str().unwrap_or(""))
    }
  }
}
