use crate::{
  functions::{
    cstr_cow, currfuncname::currfuncname, lua_a_toobject::lua_a_toobject,
    lua_t_objtypename::lua_t_objtypename,
  },
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活与独占已由 `&mut LuaState` 接收者类型
/// 承载，wave-6d 收形降为安全 `pub fn`）：`l` 须处于可抛错的受保护帧，函数末尾经
/// `luaL_error` 组装类型错误并 unwind（返回 `!`）；`narg` 为合法栈索引（`lua_a_toobject`
/// 取对象，可空需判 NULL；`currfuncname` 读当前帧名）；`tname` 为期望类型名。
/// 剩余窄 `unsafe` 只因两处裸转手：`&*obj`（`lua_a_toobject` 依契约返回存活槽指针或
/// NULL，判空后构造的引用借窗止于 `lua_t_objtypename` 当句）与 `cstr_cow`（其自身契约
/// 由 `lua_t_objtypename` 的存活 NUL 结尾返回串满足）。
/// cpp/VM/src/laux.cpp:45 luaL_typeerrorL。
pub fn lua_l_typeerror_l(l: &mut LuaState, narg: i32, tname: &str) -> ! {
  // wave-6d 收形后 `luaL_error!` 需 `&mut` 重借用，帧名借用不得跨该调用存活：在绑定处
  // 即物化为 owned String（抛错一次性路径，文案逐字节不变）。
  let fname: Option<String> = currfuncname(l).map(|f| String::from_utf8_lossy(f).into_owned());
  let obj: *const TValue = lua_a_toobject(l, narg);

  if !obj.is_null() {
    // SAFETY: `lua_a_toobject` 依 C-API 契约返回存活可读栈槽指针或 NULL，已判非空；
    // `&*obj` 构造的共享引用仅作 `lua_t_objtypename` 纯读的实参，借窗止于当句。
    let objtypename = unsafe { lua_t_objtypename(l, &*obj) };
    // SAFETY: `lua_t_objtypename` 依契约返回随 TString 存活的 NUL 结尾 C 串（或空指针
    // 形态由 `cstr_cow` 判空折叠），`cstr_cow` 的 NUL 扫描在对象寿命内界内终止。
    let objtypename = unsafe { cstr_cow(objtypename) };

    match fname {
      Some(fname) => luaL_error!(
        l,
        "invalid argument #{} to '{}' ({} expected, got {})",
        narg,
        fname,
        tname,
        objtypename
      ),
      None => luaL_error!(
        l,
        "invalid argument #{} ({} expected, got {})",
        narg,
        tname,
        objtypename
      ),
    }
  } else if let Some(fname) = fname {
    luaL_error!(
      l,
      "missing argument #{} to '{}' ({} expected)",
      narg,
      fname,
      tname
    );
  } else {
    luaL_error!(l, "missing argument #{} ({} expected)", narg, tname);
  }
}

// r7-tprod2 尾矿台账（本文件票面 2 枚：让 2）——帧名 `String::from_utf8_lossy` 原以
// Borrowed 零堆形态置于 luaL_error 抛错前最后一步。wave-6d 收形后 `lua_l_error_l` 收
// `&mut`，帧名借用不得跨抛错调用存活，故在绑定处 `into_owned()` 物化：仍为抛错一次性
// 路径（返回 !、unwind 前单点构造），报错文案逐字节对齐 cpp laux.cpp:45
// luaL_typeerrorL 的 `%s`，仅栈上零堆折为一次性堆拷（文本、拼接序、concat 计数不变）。
