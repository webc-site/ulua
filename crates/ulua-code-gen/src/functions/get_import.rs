use ulua_vm::{
  functions::lua_v_getimport::lua_v_getimport,
  macros::clvalue::clvalue,
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_import(l: *mut lua_State, res: StkId, id: u32, pc: u32) {
  unsafe {
    let cl = clvalue!((*(*l).ci).func as *const TValue);
    (*(*l).ci).savedpc = (*(*cl).inner.l.p).code.add(pc as usize);

    lua_v_getimport(l, (*cl).env, (*(*cl).inner.l.p).k, res, id, false);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_getImport")]
pub unsafe extern "C-unwind" fn get_import_export(l: *mut lua_State, res: StkId, id: u32, pc: u32) {
  unsafe { get_import(l, res, id, pc) }
}
