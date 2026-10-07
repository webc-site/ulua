use core::{slice::from_raw_parts, str::from_utf8};

use crate::{
  functions::{
    getthread::getthread, lua_l_optinteger::lua_l_optinteger, lua_l_traceback::lua_l_traceback,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 并处于 debug 库的受保护帧。本函数自身的裸指针前提只有一处：
/// `getthread` 从栈 1 号位取回的 `l1` 须指向存活 `LuaState`（该槽经 `is_thread` 判定为 thread
/// 值，其地址即 GC 持有的协程对象），且 `l1` 的调用栈自 `level` 起各帧可读——此前提由
/// `lua_l_traceback` 的逐帧 `lua_getinfo` 游走消费，本帧只把它原样转交、不就地解引用。
/// 另须 `msg` 快照界内可读：串体不可变不移动、由栈 `arg+1` 槽引用钉住至本帧末。
/// 栈 `arg+1` 为可选 msg、`arg+2` 为可选 level（负 level 经 `arg_check` 抛错发散）。
/// cpp/VM/src/ldblib.cpp:122 db_traceback。
pub(crate) unsafe fn db_traceback(l: &mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 存活，`l.as_mut_ptr()` 即其自身地址；被调方只读该帧栈 1 号位，
  // 借用窗止于本次调用，返回的 `l1` 在本帧不再经 `l` 的借用解引用
  let (l1, arg) = unsafe { getthread(l.as_mut_ptr()) };
  let default_level = if l1 == l.as_mut_ptr() { 1 } else { 0 };

  // 可抛 msg（cpp `luaL_optstring(L, arg + 1, NULL)`）：nil/缺席即 `None`，其余经 `check_bytes`
  // 取串窗（数字就地转串、非串抛错发散），界内长度由切片自带，不再走「指针 + 长度出参」。
  let msg: Option<&str> = if l.is_none_or_nil(arg + 1) {
    None
  } else {
    let bytes = l.check_bytes(arg + 1);
    // 锚定形：串窗借用止于 (ptr, len) 快照（Lua 串不可变不移动、`arg+1` 槽引用钉住至本帧末），
    // 故可越过其后 `optinteger`/`arg_check` 对 `l` 的再借用存活至 `lua_l_traceback` 转手
    let (ptr, len) = (bytes.as_ptr(), bytes.len());
    // SAFETY: 快照由本次取参时的存活借用铸成，指向的串体在本帧内不移动不释放（上述钉住论证），
    // 重物化窗的读界与原切片逐字节同
    Some(from_utf8(unsafe { from_raw_parts(ptr, len) }).unwrap_or(""))
  };

  let level = lua_l_optinteger(l, arg + 2, default_level);
  l.arg_check(level >= 0, arg + 2, "level can't be negative");

  // SAFETY: `l1` 存活前提见本函数 `# Safety`（该地址由 `getthread` 自 thread 槽取回，
  // 其调用栈自 `level` 起可读）；`msg` 窗按 C 语义截读至首个 NUL，本帧内可读
  unsafe { lua_l_traceback(l, l1, msg, level) };

  1
}

lua_lib_fn!(pub(crate) fn db_traceback @ref, db_traceback_arm);
