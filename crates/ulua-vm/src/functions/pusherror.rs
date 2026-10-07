use alloc::string::String;
use core::ffi::c_char;

use crate::{
  functions::{
    cstr_bytes_ref::cstr_bytes_ref, cstr_cow, currentline::currentline, getluaproto::get_lua_proto,
    lua_o_chunkid::lua_o_chunkid, lua_o_pushfstring::lua_o_pushfstring,
  },
  macros::{getstr::getstr, is_lua::isLua, lua_idsize::LUA_IDSIZE},
  records::lua_state::LuaState,
};

/// 安全字节切片版错误信息压栈。
///
/// r16-v7 步骤 1：接收者前移 `*mut` → `&mut LuaState`（压栈写面全经 `l` 可达，
/// 独占借用承载）；帧面/对象面裸读触点（`isLua!` 谓词、proto/source 字段
/// 读、`currentline` 引用形构造、`cstr_cow` NUL 扫读、`lua_o_pushfstring` 透传）
/// 保留窄 `unsafe {}` 窗（pub(crate) 豁免域），逐窗 SAFETY 一句；`l.ci` 帧字段
/// 经引用现读系 safe 位，不设窗。
pub(crate) fn pusherror_bytes(l: &mut LuaState, msg: &[u8]) {
  // `ci` 帧字段经存活 `&mut` 引用现读，拷贝裸读手柄即出——非裸触点，safe 位。
  let ci = l.ci;

  // SAFETY: `ci` 为刚自存活 `l` 取出的当前帧；`isLua!` 帧宏读 `(*ci).func` 及函数
  // 对象型字段，系纯读谓词，不留存引用。
  if unsafe { isLua!(ci) } {
    let mut chunkbuf: [c_char; LUA_IDSIZE as usize] = [0; LUA_IDSIZE as usize];
    // SAFETY: `isLua!` 谓词真值确立本帧为 Lua 闭包存活帧：`get_lua_proto` 帧面取
    // proto、`(*proto).source`/`(*source).len` 对象面裸读与 `getstr`/`lua_o_chunkid`
    // 裸指针垫片均在帧-对象存活界内成立；`chunkbuf` 写窗落点由本函数局部缓冲承载，
    // 返回指针寿命随 `chunkbuf`/`source`（`lua_o_chunkid` 契约），出窗仅透传该指针。
    let chunkid = unsafe {
      let proto = get_lua_proto(ci);
      let source = (*proto).source;
      lua_o_chunkid(
        chunkbuf.as_mut_ptr(),
        chunkbuf.len(),
        getstr(source),
        (*source).len as usize,
      )
    };
    // SAFETY: `ci` 为存活帧；`&*ci` 构造的引用形 `currentline` 系帧面纯读，借用窗
    // 止于本调用语句。
    let line = unsafe { currentline(&*ci) };
    // SAFETY: `chunkid` 为 `lua_o_chunkid` 落点（`chunkbuf` 内 NUL 结尾串或 TString
    // 源名偏移串），调用期间存活且 NUL 终止，`cstr_cow` 的 NUL 扫描必界内终止。
    let chunk = unsafe { cstr_cow(chunkid) };
    let msg_str = String::from_utf8_lossy(msg);
    lua_o_pushfstring(l, format_args!("{}:{}: {}", chunk, line, msg_str));
  } else {
    l.push_bytes(msg);
  }
}

/// r16-v7 步骤 3（仿 v4c 判例）：导出垫片转 safe——`pub unsafe fn(*mut)` →
/// `pub fn(&mut LuaState, *const c_char)`，体一行转调本票 safe 化的
/// [`pusherror_bytes`]，C 串裸参只经既有 pub(crate) safe 门面 [`cstr_bytes_ref`]
/// 消费（裸参入 safe 实参位，`not_unsafe_ptr_arg_deref` 触发消亡；不新造门面、
/// 不动 `cstr_bytes` 本体）。
/// # Safety
/// 调用序契约（正确性，非内存安全；safe fn 文档断言，由调用方承载）：`l` 存活与
/// 独占由 `&mut` 接收者类型承载；`msg` 须为 NUL 结尾、调用期间存活的 C 串
/// （门面内 NUL 扫描必终止），消息拼装语义与 cpp 参考实现逐位不变。
pub fn pusherror(l: &mut LuaState, msg: *const c_char) {
  pusherror_bytes(l, cstr_bytes_ref(msg));
}
