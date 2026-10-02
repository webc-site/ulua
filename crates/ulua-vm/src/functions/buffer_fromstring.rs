use crate::{
  functions::{
    buffer_window::buffer_data_ref, lua_l_checklstring::lua_l_checklstring_ref,
    lua_newbuffer::lua_newbuffer_push_ref,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// cpp `buffer_fromstring`（lbuflib.cpp:47）：按字符串实参长度新建 buffer 并整段复制。
///
/// # Safety
/// `l` 须为本次 buffer 库调用的存活 `LuaState` 且处于受保护帧（`checklstring` 非串
/// 实参经 `tag_error` 抛 "string expected" 发散），栈顶有压入新 buffer 的余量。
/// 重入面论证：仅 #1 一次取参，走 `lua_tolstring`→`luaV_tostring` 内置数值转换
/// （cpp lvmutils.cpp:39，不调 `__tostring`、不跑 Lua 代码）；`lua_newbuffer_push_ref`
/// 只做分配/GC 步进/挪栈（Lua 串与 buffer 对象均不移动，#1 串切片按
/// `lua_l_checklstring_ref` 契约在本次调用内可读），其后到写入点之间不再有任何
/// 取参或可抛错重入路径——新对象在栈顶 `-1` 槽钉住、定长不 resize，数据窗经
/// `buffer_data_ref` 后置派生直达 `copy_from_slice`，源与目的分属两个对象不重叠。
pub(crate) unsafe fn buffer_fromstring(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 为存活调用帧；窗口派生与复制的边界论证见函数级 `# Safety`
  unsafe {
    let val = lua_l_checklstring_ref(&mut *l, 1);

    // 新 buffer 压入栈顶；返回裸数据指针面已随 r12-w6b 收窄消灭，数据窗经窄腰切片形取回
    lua_newbuffer_push_ref(&mut *l, val.len());
    // 刚压入的槽必是 buffer（typeerror 分支不可达），数据界即新建长度 `val.len()`
    let dst = buffer_data_ref(l, -1);
    debug_assert_eq!(dst.len(), val.len());
    dst.copy_from_slice(val);

    1
  }
}

lua_lib_fn!(pub(crate) fn buffer_fromstring, buffer_fromstring_arm);
