use ulua_vm::{
  functions::lua_v_getimport::lua_v_getimport,
  records::{lua_state::LuaState, slot::Slot},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `extern "C-unwind"` FFI 边界，调用方（生成的原生代码/VM）按 Lua codegen 回调约定保证：
/// `l` 为存活 `LuaState` 且 `(*l).ci` 在 CallInfo 数组内，`res` 为帧内活栈槽，`pc` 为当前
/// 闭包 `code` 内的合法指令偏移，`id` 由调用方在帧内提供；`lua_v_getimport` 只访问这些活对象。
pub unsafe extern "C-unwind" fn get_import(l: *mut LuaState, res: StkId, id: u32, pc: u32) {
  // Safety: l 为活 LuaState, (*l).ci 在 CallInfo 数组内; as_closure((*ci).func) 得活 L 闭包,
  // 其 inner.l.p 为 Some 的活 Proto, code 数组有 sizecode 项、k 常量数组有 sizek 项。
  // pc 为本 GETIMPORT 起始偏移(在 sizecode 内), code.add(pc) 合法; res/id 由调用方在帧内提供,
  // lua_v_getimport 只访问这些活对象。
  unsafe {
    let cl = (*(*(*l).ci).func).as_closure();
    (*(*l).ci).savedpc = (*cl.inner.l.p).code.add(pc as usize);

    lua_v_getimport(l, cl.env, (*cl.inner.l.p).k, Slot::from_raw(res), id, false);
  }
}
