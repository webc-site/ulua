use ulua_vm::records::{Closure::Closure, Proto::Proto, lua_state::lua_State};

use crate::common::records::assert_inliner_data::AssertInlinerData;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn id_inliner_with_assert(
  l: *mut lua_State,
  caller: *mut Closure,
  target: *mut Closure,
  pc: u32,
) -> *mut Proto {
  unsafe {
    let data = (*(*l).global).ecbdata.as_mut_ptr() as *mut AssertInlinerData;
    assert!(!data.is_null());

    let caller_proto = (*caller).inner.l.p;
    let target_proto = (*target).inner.l.p;

    assert_eq!((*data).proto, caller_proto);
    assert_eq!((*data).target, target_proto);
    assert_eq!((*data).pc, pc);

    (*data).called = true;
    caller_proto
  }
}
