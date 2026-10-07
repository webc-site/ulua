use core::ffi::c_char;

use crate::{
  functions::cstr_bytes_ref::cstr_bytes_ref,
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
/// `gs_mut` 一句一借。r16-v4c 步骤 4（v4b 留形点回头）：`pub unsafe fn` → `pub fn`
/// ——crate 内新增 `pub(crate)` safe 门面 [`cstr_bytes_ref`] 收拢 `cstr_bytes` 的
/// NUL 扫读 unsafe（仿 `ulua-repl-cli` `state_ref` 先例，契约下沉 crate 内、非导出
/// 即免检），裸参 `name` 此后只入 safe 被调实参位，`clippy::not_unsafe_ptr_arg_deref`
/// 触发消亡；`name: *const c_char` 保持裸形不切片化（切片化=语义扩面，另案）。
/// # Safety
/// 调用序契约（正确性，非内存安全；safe fn 文档断言，由调用方承载）：`tag` 须
/// `< LUA_LUTAG_LIMIT`（`api_check` debug 断言；release 越界由数组安全索引 panic 兜住，
/// 即调用方违约当场响亮失败）；`name` 须为 NUL 结尾、调用期间存活的 C 串（门面内
/// NUL 扫描必终止，`lua_s_new` intern 后可提取）；目标槽须原本为空（不支持重命名，
/// 命中非空即静默跳过，cpp 同形）。
pub fn lua_setlightuserdataname(l: &mut LuaState, tag: i32, name: *const c_char) {
  api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);
  // renaming not supported（gs_ref 只读视图一句一借，视图不出本句；断言纯读可安全承载）
  api_check!(l, l.gs_ref().lightuserdataname[tag as usize].is_null());

  if l.gs_ref().lightuserdataname[tag as usize].is_null() {
    // 入参为 NUL 结尾 C 串，经 crate 内 safe 门面 cstr_bytes_ref 扫首个 NUL 得字节切片
    // （保持原 lua_s_new 的 strlen 语义）
    // SAFETY: 契约保证 `name` 为 NUL 结尾、调用期间存活的 C 串（读数收拢于 safe 门面
    // 单窗）；intern 触点 `lua_s_new` 为 C-ABI 镜像裸触点、可分配/可 GC，故先于 gs 写
    // 借执行，借用窗止于本调用语句。
    let ts = unsafe { lua_s_new(l, cstr_bytes_ref(name)) };
    // r16-b3 收编形制保持：槽写经 gs_mut 一句一借——借用起于 lua_s_new（分配/可 GC）
    // 返回之后，窗内再无其它调用，红线不跨
    l.gs_mut().lightuserdataname[tag as usize] = ts;
    // SAFETY: `ts` 为刚 intern 的存活 TString；l_setbit 位落笔仅写其 hdr.marked
    unsafe { l_setbit!((*ts).hdr.marked, FIXEDBIT) }; // never collect these names
  }
}
