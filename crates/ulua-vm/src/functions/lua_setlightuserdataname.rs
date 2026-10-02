use core::ffi::c_char;

use crate::{
  functions::cstr_bytes,
  macros::{
    api_check::api_check, fixedbit::FIXEDBIT, l_setbit::l_setbit, lua_lutag_limit::LUA_LUTAG_LIMIT,
    lua_s_new::lua_s_new,
  },
  records::lua_state::LuaState,
};

/// `lua_setlightuserdataname`（cpp/VM/src/lapi.cpp:2097）。r16-v4b 收口：接收者前移
/// `*mut` → `&mut LuaState`（写点落经 `l` 可达的 `global_State.lightuserdataname`
/// 注册槽，须独占承载；`l` 存活与独占由类型承载），unsafe 自整块体体内内移到真实裸
/// 触点（`lua_s_new` intern 与 `l_setbit` 位落笔两窗），gs 读走 `gs_ref`、写走
/// `gs_mut` 一句一借。签名留 `unsafe fn` 的裁决依据：裸参 `name` 直接流入
/// `unsafe fn` `cstr_bytes`（其本体确经该指针扫 NUL 读数，属 c-API 边界本体，判语同
/// `ulua-repl-cli/setup_state.rs` 先例）；safe 化须切 `cstr_bytes` 或引入全仓首例
/// `#[allow]`，均系禁扩面项，按纪律留形申报移交。`name: *const c_char` 保持裸形不
/// 切片化（切片化=语义扩面，另案）。
/// # Safety
/// 调用序契约（正确性，非内存安全）：`tag` 须 `< LUA_LUTAG_LIMIT`（`api_check` debug
/// 断言；release 越界由数组安全索引 panic 兜住，即调用方违约当场响亮失败）；`name` 须
/// 为 NUL 结尾、调用期间存活的 C 串（`lua_s_new` intern 后可提取）；目标槽须原本为空
/// （不支持重命名，命中非空即静默跳过，cpp 同形）。
pub unsafe fn lua_setlightuserdataname(l: &mut LuaState, tag: i32, name: *const c_char) {
  api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);
  // renaming not supported（gs_ref 只读视图一句一借，视图不出本句；断言纯读可安全承载）
  api_check!(l, l.gs_ref().lightuserdataname[tag as usize].is_null());

  if l.gs_ref().lightuserdataname[tag as usize].is_null() {
    // 入参为 NUL 结尾 C 串，经 cstr_bytes 扫首个 NUL 得字节切片（保持原 lua_s_new 的 strlen 语义）
    // SAFETY: 契约保证 `name` 为 NUL 结尾、调用期间存活的 C 串；intern 触点可分配/
    // 可 GC，故先于 gs 写借执行，借用窗止于本调用语句。
    let ts = unsafe { lua_s_new(l, cstr_bytes(name)) };
    // r16-b3 收编形制保持：槽写经 gs_mut 一句一借——借用起于 lua_s_new（分配/可 GC）
    // 返回之后，窗内再无其它调用，红线不跨
    l.gs_mut().lightuserdataname[tag as usize] = ts;
    // SAFETY: `ts` 为刚 intern 的存活 TString；l_setbit 位落笔仅写其 hdr.marked
    unsafe { l_setbit!((*ts).hdr.marked, FIXEDBIT) }; // never collect these names
  }
}
