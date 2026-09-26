use crate::{
  functions::{c_slice, validateobjref::validateobjref, validateref::validateref},
  macros::obj_2_gco::obj2gco,
  records::{global_state::global_State, luau_object::LuauObject},
};

/// # Safety
///
/// 输入字节流缓冲与读游标必须位于本次加载的串数据界内，长度参数覆盖所有被读字段。
pub(crate) unsafe fn validateobject(g: *mut global_State, inst: *mut LuauObject) {
  // Safety: 契约保证 gco 类型标签可读且对象字段区完整，逐类型校验仅读取不写出对象界
  unsafe {
    let obj = obj2gco!(inst);
    validateobjref(g, obj, obj2gco!((*inst).lclass));
    let numberofmembers = (*inst).numberofmembers;
    let members = (*inst).members;
    // Safety:members 为 C 指针 + numberofmembers 计数，随对象分配。
    for member in c_slice(members, numberofmembers as usize) {
      validateref(g, obj, member);
    }
  }
}
