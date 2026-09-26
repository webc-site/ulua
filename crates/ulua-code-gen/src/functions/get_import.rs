use ulua_vm::{
  functions::lua_v_getimport::lua_v_getimport, records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn get_import(l: *mut LuaState, res: StkId, id: u32, pc: u32) {
  // Safety: l 为活 LuaState, (*l).ci 在 CallInfo 数组内; as_closure((*ci).func) 得活 L 闭包,
  // 其 inner.l.p 为 Some 的活 Proto, code 数组有 sizecode 项、k 常量数组有 sizek 项。
  // pc 为本 GETIMPORT 起始偏移(在 sizecode 内), code.add(pc) 合法; res/id 由调用方在帧内提供,
  // lua_v_getimport 只访问这些活对象。
  unsafe {
    let cl = (*(*(*l).ci).func).as_closure();
    (*(*l).ci).savedpc = (*cl.inner.l.p).code.add(pc as usize);

    lua_v_getimport(l, cl.env, (*cl.inner.l.p).k, res, id, false);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn get_import_export(l: *mut LuaState, res: StkId, id: u32, pc: u32) {
  // Safety: 导出 C ABI 入口原样转发 l/res/id/pc 给同契约 unsafe fn get_import; 调用方按 ABI
  // 保证 l 活、res 为帧内活栈槽、pc 为合法 code 偏移, 满足被调前置条件。
  unsafe { get_import(l, res, id, pc) }
}
