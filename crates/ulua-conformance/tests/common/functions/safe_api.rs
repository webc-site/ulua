//! C API 用例的安全门面（review.md §0/§2/§8）。
//!
//! `lua_*` C ABI 本身是 FFI 边界，`unsafe` 无法根除；本模块把「调用一个 C API」
//! 这一重复动作各自收口成一次性 `unsafe`，用例侧只做 safe 调用。集中契约：
//!
//! - **`l` 契约**：所有以 `l: L` 收参的函数，要求 `l` 为**存活**的 `LuaState`——
//!   即由 [`new_state`](super::new_state::new_state) / [`run_conformance`] /
//!   [`StateRef::as_ptr`](crate::common::records::state_ref::StateRef::as_ptr) /
//!   [`newthread`] 在本用例作用域内交回、且在使用点尚未被 `lua_close` 的指针。
//! - **索引契约**：`idx` 为合法栈索引（C API 的伪索引规则同样适用）；违反即 UB，
//!   与直接调 C API 完全同语义——本门面只消除 `unsafe` 块样板，不新增任何校验、
//!   不改变任何输入→输出行为。
//! - **返回借用契约**：返回 `&'a [u8]` / `Option<&'a mut c_void>`
//!   的函数，其借用指向 VM 内部缓冲或栈槽，在下一次改栈 / GC 前有效；用例须即时
//!   消费（比对 / 打印），与 `cstr_bytes(lua_tostring!(..))` 旧写法寿命一致。
//! - **`&'static [u8]` 名字参数**：须为 NUL 结尾字面量（`b"name\0"`），经 [`cstr`]
//!   单点转 `*const c_char`（review.md §10）。

use alloc::string::String;
use core::{
  ffi::{c_char, c_int, c_void},
  ptr::{from_mut, null, null_mut, write},
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_code_gen::{
  functions::{compile_internal::compile_internal, luau_codegen_create::luau_codegen_create},
  records::{compilation_options::CompilationOptions, compilation_result::CompilationResult},
};
use ulua_common::functions::c_str::cstr_bytes;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_c_dump::lua_c_dump, lua_c_enumheap::lua_c_enumheap, lua_c_fullgc::lua_c_fullgc,
    lua_c_validate::lua_c_validate, lua_callbacks::lua_callbacks, lua_checkstack::lua_checkstack,
    lua_cleartable::lua_cleartable, lua_clonefunction::lua_clonefunction,
    lua_clonetable::lua_clonetable, lua_cpcall::lua_cpcall, lua_createtable::lua_createtable,
    lua_debugtrace::lua_debugtrace, lua_equal::lua_equal, lua_gc::lua_gc,
    lua_getallocf::lua_getallocf, lua_getlightuserdataname::lua_getlightuserdataname,
    lua_getreadonly::lua_getreadonly, lua_gettable::lua_gettable,
    lua_getuserdatadtor::lua_getuserdatadtor, lua_getuserdatametatable::lua_getuserdatametatable,
    lua_isnumber::lua_isnumber, lua_isstring::lua_isstring, lua_isuserdata::lua_isuserdata,
    lua_l_checkbuffer::lua_l_checkbuffer, lua_l_checkstack::lua_l_checkstack,
    lua_l_checkudata::lua_l_checkudata, lua_l_openlibs::lua_l_openlibs,
    lua_l_register::lua_l_register, lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread, lua_l_typename::lua_l_typename,
    lua_lightuserdatatag::lua_lightuserdatatag, lua_newbuffer::lua_newbuffer,
    lua_newstate::lua_newstate, lua_newthread::lua_newthread,
    lua_newuserdatadtor::lua_newuserdatadtor, lua_newuserdatatagged::lua_newuserdatatagged,
    lua_newuserdatataggedwithmetatable::lua_newuserdatataggedwithmetatable,
    lua_pushcclosurek::lua_pushcclosurek, lua_pushlightuserdatatagged::lua_pushlightuserdatatagged,
    lua_rawget::lua_rawget, lua_rawgetfield::lua_rawgetfield_bytes, lua_rawgeti::lua_rawgeti,
    lua_rawgetptagged::lua_rawgetptagged, lua_rawiter::lua_rawiter,
    lua_rawsetfield::lua_rawsetfield_bytes, lua_rawseti::lua_rawseti,
    lua_rawsetptagged::lua_rawsetptagged, lua_ref::lua_ref,
    lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    lua_resumeerror::lua_resumeerror, lua_setfenv::lua_setfenv,
    lua_setlightuserdataname::lua_setlightuserdataname, lua_settable::lua_settable,
    lua_setuserdatadtor::lua_setuserdatadtor, lua_setuserdatametatable::lua_setuserdatametatable,
    lua_setuserdatatag::lua_setuserdatatag, lua_status::lua_status, lua_tobuffer::lua_tobuffer,
    lua_tolightuserdata::lua_tolightuserdata, lua_tolightuserdatatagged::lua_tolightuserdatatagged,
    lua_tolstring::lua_tolstring, lua_tonumberx::lua_tonumberx, lua_topointer::lua_topointer,
    lua_tostringatom::lua_tostringatom, lua_touserdata::lua_touserdata,
    lua_touserdatatagged::lua_touserdatatagged, lua_type::lua_type, lua_typename::lua_typename,
    lua_unref::lua_unref, lua_userdatatag::lua_userdatatag, luaopen_base::luaopen_base,
    luau_load::luau_load,
  },
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
  type_aliases::{
    lua_alloc::LuaAlloc, lua_c_function::LuaCFunction, lua_continuation::LuaContinuation,
    lua_destructor::LuaDestructor, lua_userdata_direct_field_get::LuaUserdataDirectFieldGet,
  },
};

use crate::common::functions::{cpcall_test::cpcall_test, cstr::cstr, cstr_text::diagnostic_text};

/// C ABI 状态句柄别名；仅在本门面内部被解引用（每函数恰一次 `unsafe`）。
pub type L = *mut LuaState;

/// `lua_newuserdatadtor` 的内联析构类型（VM 侧同名 type alias 未导出，此处镜像）。
pub type UserdataDtorRaw = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// `lua_newthread` 的返回值同样是存活线程栈，归父状态持有；用例经门面取得后
/// 可安全作为后续 [`L`] 参数。
pub type AtomAssignFn = Option<unsafe extern "C-unwind" fn(L, *const c_char, usize) -> i16>;

// ---------------------------------------------------------------------------
// 栈管理
// ---------------------------------------------------------------------------

/// `lua_gettop`：当前栈深。
pub fn gettop(l: L) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).get_top() }
}

/// `lua_settop`：截断 / 填充到 `idx`。
pub fn settop(l: L, idx: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).set_top(idx) }
}

/// `lua_pop`：弹掉栈顶 `n` 槽。
pub fn pop(l: L, n: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).pop(n) }
}

/// `lua_checkstack`：尝试扩容 `size` 槽，返回 C 侧布尔（0/非 0）。
pub fn checkstack(l: L, size: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_checkstack(l, size) }
}

/// `luaL_checkstack`：扩容失败即抛 Lua 错误（消息 `msg`）。
pub fn l_checkstack(l: L, space: c_int, msg: &str) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_l_checkstack(l, space, msg) }
}

/// `lua_pushvalue`：复制槽 `idx` 压栈。
pub fn pushvalue(l: L, idx: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).push_value(idx) }
}

/// `lua_pushnumber`。
pub fn pushnumber(l: L, n: f64) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).push_number(n) }
}

/// `lua_pushinteger`。
pub fn pushinteger(l: L, n: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).push_integer(n) }
}

/// `lua_pushboolean`（保持 C 侧 `int` 参数语义）。
pub fn pushboolean(l: L, b: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).push_boolean(b != 0) }
}

/// `lua_pushnil`。
pub fn pushnil(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).push_nil() }
}

/// `lua_pushstring`，名字参数为静态字节串。
pub fn pushstr(l: L, s: &'static [u8]) {
  let s = s.strip_suffix(b"\0").unwrap_or(s);
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).push_bytes(s) }
}

/// `lua_pushcclosurek` 的 `nup=0`、无 continuation 形态（cpp `lua_pushcfunction`，
/// `debugname` 传 NULL）。
pub fn pushcfunction(l: L, f: LuaCFunction) {
  // Safety: `l` 存活；`debugname` 为 null、`nup` 为 0，交 `lua_pushcclosurek` 契约。
  unsafe { lua_pushcclosurek(l, f, null(), 0, None) }
}

/// `lua_pushcclosurek` 全参形态（可 yield continuation 用例）。
/// `debugname` 为 `None` 时传 C 侧 NULL。
pub fn pushcclosurek(
  l: L,
  f: LuaCFunction,
  debugname: Option<&'static [u8]>,
  nup: c_int,
  cont: LuaContinuation,
) {
  // Safety: `l` 存活；名字串经 `cstr` 契约（NUL 结尾静态缓冲）或为 null。
  let name = debugname.map_or_else(null, cstr);
  unsafe { lua_pushcclosurek(l, f, name, nup, cont) }
}

/// 设置状态回调表的 `useratom`（字符串 atom 分配钩子）。
pub fn set_useratom(l: L, f: AtomAssignFn) {
  // Safety: `l` 存活；`lua_callbacks` 返回该状态持有的回调表指针；`f` 遵循 C ABI。
  unsafe { (*lua_callbacks(l)).useratom = f }
}

// ---------------------------------------------------------------------------
// 库装载 / 沙箱 / 校验（run_conformance 编排层的收口）
// ---------------------------------------------------------------------------

/// `luaL_openlibs`。
pub fn openlibs(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_l_openlibs(l) }
}

/// `luaopen_base`：装载 base 库并返回其栈占用（供 [`pop`] 回收）。
pub fn open_base(l: L) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { luaopen_base(l) }
}

/// `luaL_register(l, NULL, funcs)`。
pub fn l_register(l: L, funcs: &[LuaLReg]) {
  // Safety: `l` 存活；指针仅在本调用期内被读取。
  unsafe { lua_l_register(l, null(), funcs) }
}

/// `luaL_sandbox`。
pub fn sandbox(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_l_sandbox(l) }
}

/// `luaL_sandboxthread`。
pub fn sandboxthread(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_l_sandboxthread(l) }
}

/// `lua_validate`：VM 内部一致性校验。
pub fn c_validate(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_c_validate(l) }
}

/// `lua_resumeerror(l, NULL)`：向协程注入错误的恢复（c-API NULL 实参边界契约，既有约定 review.md §2）。
pub fn resumeerror(l: L) -> c_int {
  // Safety: `l` 存活；`from` 传 C 侧 NULL（主状态恢复，vm 契约明许）。
  unsafe { lua_resumeerror(l, null_mut()) }
}

// ---------------------------------------------------------------------------
// 编译-加载-释放整链
// ---------------------------------------------------------------------------

/// `compile` → `luau_load` 的一次性整链收口：编译 `source`（按
/// 原始字节透传，允许非 UTF-8）为字节码并加载进 `l`，返回 `luau_load` 的状态码；
/// 编译产物为本帧 owned `Vec<u8>`（cpp 的 malloc/free 契约在 Rust 端消解），
/// 不留悬垂缓冲。
pub fn load_source(l: L, chunkname: &str, source: &[u8], options: &mut CompileOptions) -> c_int {
  let bytecode = compile(source, options, &ParseOptions::default(), NoopEncoder);
  // Safety: `l` 存活；`chunkname` 为合法借用，`bytecode` 为本帧拥有的合法切片。
  unsafe { luau_load(l, chunkname, &bytecode, 0) }
}

/// `luau_load` 的直接形态：加载调用方已持有的字节码切片，返回状态码（0 为成功）。
/// 用于「同一份产物多轮加载/失败注入」这类不能重新编译的场景。
pub fn load_bytes(l: L, chunkname: &str, bytecode: &[u8], pass: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）；`chunkname`/`bytecode` 为调用方在本帧持有的合法借用。
  unsafe { luau_load(l, chunkname, bytecode, pass) }
}

/// `compile_internal`（native codegen）对栈顶函数的收口：`l` 存活且栈顶 -1 为
/// 已加载 proto；返回主编译结果供断言（统计出参不收集，与 cpp 传 NULL 同形）。
pub fn compile_native(l: L, options: &CompilationOptions) -> CompilationResult {
  // Safety: `l` 存活、栈顶为 proto；共享选项传 `&None`、统计出参传 null，与 cpp
  // `Luau::CodeGen::compile(L, -1, opts)` 逐字同形。
  unsafe { compile_internal(&None, l, -1, options, None) }
}

/// `luau_codegen_create`：为该状态开启 native codegen 上下文。
pub fn codegen_create(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { luau_codegen_create(l) }
}

/// 栈顶错误/结果串的带兜底诊断文本（null 译为 `"<null>"`，仅失败信息使用）。
pub fn top_diag_str(l: L, idx: c_int) -> String {
  // Safety: `l` 存活；`lua_tolstring` 返回 null 或本帧保活 NUL 串，`diagnostic_text`
  // 已对 null 兜底（c-API len 出参传 NULL 边界契约，既有约定 review.md §2）。
  unsafe { diagnostic_text(lua_tolstring(l, idx, null_mut())) }
}

/// `lua_debugtrace` 的带兜底诊断文本。
pub fn debugtrace_text(l: L) -> String {
  // Safety: `l` 存活；返回 null 或 NUL 结尾串，`diagnostic_text` 已兜底。
  unsafe { diagnostic_text(lua_debugtrace(l)) }
}

/// `lua_newtable`。
pub fn newtable(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).new_table() }
}

/// `lua_newthread`：在 `l` 上派生线程栈并压栈，返回其裸指针（用例随后按 [`L`] 用）。
pub fn newthread(l: L) -> L {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_newthread(l) }
}

/// `lua_newuserdata`（tag 0）：返回数据块裸指针，由 VM 持有。
pub fn newuserdata(l: L, sz: usize) -> *mut c_void {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).new_userdata(sz) }
}

/// `lua_concat`。
pub fn concat(l: L, n: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).concat(n) }
}

// ---------------------------------------------------------------------------
// 调用
// ---------------------------------------------------------------------------

/// `lua_call`： unprotected 调用（Lua 侧错误会经 unwind 穿出本层，与直调一致）。
pub fn call(l: L, nargs: c_int, nresults: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).call(nargs, nresults) }
}

/// `lua_pcall`：返回 [`LuaStatus`] 编码的 `c_int`。
pub fn pcall(l: L, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).pcall(nargs, nresults, errfunc) }
}

/// cpp `cpcalltest` 场景：以 `should_fail` 的裸指针为载荷 protected 调用。
/// 指针仅在本语句内存活并透传给被调 C 闭包，与旧写法逐字同语义。
pub fn cpcall_bool(l: L, should_fail: &mut bool) -> c_int {
  // Safety: `l` 存活；载荷为本帧 `&mut bool` 的裸指针，仅在受保护调用期间透传。
  unsafe {
    lua_cpcall(
      l,
      Some(cpcall_test),
      (from_mut(should_fail)).cast::<c_void>(),
    )
  }
}

/// `lua_resume`；`from` 用 `Option` 收编 C 侧 NULL 哨兵（resume from=NULL 边界契约，既有约定 review.md §2）。
pub fn resume(l: L, from: Option<L>, nargs: c_int) -> c_int {
  // Safety: `l`/`from`（若非 None）均为存活状态（模块级契约）。
  unsafe { (*l).resume(from.unwrap_or(null_mut()), nargs) }
}

// ---------------------------------------------------------------------------
// 栈槽读取
// ---------------------------------------------------------------------------

/// `lua_type`。
pub fn type_(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_type(l, idx) }
}

/// `lua_typename(t)` 的原始字节视图。
pub fn typename_bytes<'a>(l: L, t: c_int) -> &'a [u8] {
  // Safety: `l` 存活；`lua_typename` 返回 VM 持有的 NUL 结尾常量串。
  unsafe { cstr_bytes(lua_typename(l, t)) }
}

/// `luaL_typename(l, idx)` 的原始字节视图。
pub fn l_typename_bytes<'a>(l: L, idx: c_int) -> &'a [u8] {
  // Safety: `l` 存活；返回 NUL 结尾类型名串（VM 内部缓冲）。
  unsafe { cstr_bytes(lua_l_typename(l, idx)) }
}

/// `lua_tostring!(l, idx)` + 原始字节收口（要求 `idx` 为字符串，与旧断言同前提）。
pub fn to_bytes<'a>(l: L, idx: c_int) -> &'a [u8] {
  // Safety: `l` 存活且 `idx` 为字符串槽（用例断言前置）；返回 NUL 结尾 VM 缓冲。
  unsafe { (*l).to_bytes(idx).unwrap_or_default() }
}

/// `lua_tostringatom` 的成对读取：返回字符串原始字节与 atom 命中计数。
pub fn tostratom<'a>(l: L, idx: c_int) -> (&'a [u8], c_int) {
  let mut atom: c_int = 0;
  // Safety: `l` 存活且 `idx` 为字符串槽；`atom` 为本帧局部可写出参；返回指针指向
  // VM 内部缓冲（用例期间存活）。
  let p = unsafe { lua_tostringatom(l, idx, &mut atom) };
  // Safety: 上一步契约——`p` 非空且 NUL 结尾。
  (unsafe { cstr_bytes(p) }, atom)
}

/// `luaL_checkstring!(l, idx)` + 原始字节收口（非字符串即抛 Lua 错误，同 C 语义）。
pub fn l_checkstring_bytes<'a>(l: L, idx: c_int) -> &'a [u8] {
  // Safety: `l` 存活；返回 NUL 结尾栈内串。
  unsafe { (*l).check_bytes(idx) }
}

/// `lua_tonumber!(l, idx)`（`lua_tonumberx(..).unwrap_or(0.0)`）。
pub fn tonumber(l: L, idx: c_int) -> f64 {
  // Safety: `l` 存活（模块级契约）；非数值回报 0，与宏一致。
  unsafe { lua_tonumberx(l, idx).unwrap_or(0.0) }
}

/// `lua_l_checkinteger`：非整数即抛 Lua 错误。
pub fn l_checkinteger(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).check_integer(idx) }
}

/// `lua_toboolean`。
pub fn toboolean(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).to_boolean(idx) as c_int }
}

/// `lua_isstring`（C 侧 0/非 0）。
pub fn isstring(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_isstring(l, idx) }
}

/// `lua_isnumber`（C 侧 0/非 0）。
pub fn isnumber(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_isnumber(l, idx) }
}

/// `lua_status`。
pub fn status(l: L) -> c_int {
  // Safety: `l` 存活（模块级契约），`&*l` 只读引用重建前提成立。
  unsafe { lua_status(&*l) }
}

/// `lua_equal`（C 侧 0/1）。
pub fn equal(l: L, a: c_int, b: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_equal(l, a, b) }
}

/// `lua_objlen`。
pub fn objlen(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).obj_len(idx) as c_int }
}

/// `lua_gc`：`what` 传 [`LuaGcOp`] 的 C 编码值。
pub fn gc(l: L, what: c_int, data: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_gc(l, what, data) }
}

// ---------------------------------------------------------------------------
// 表 / 字段 / 环境
// ---------------------------------------------------------------------------

/// `lua_next` 游标步进。
pub fn next(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).next(idx) as c_int }
}

/// `lua_rawiter` 索引遍历。
pub fn rawiter(l: L, idx: c_int, iter: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawiter(l, idx, iter) }
}

/// `lua_getfield`。
pub fn getfield(l: L, idx: c_int, k: &'static [u8]) -> c_int {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // Safety: `l` 存活（模块级契约）；`key` 切片在调用返回前有效，被调侧不得逃逸
  unsafe { (*l).get_field_bytes(idx, key) }
}

/// `lua_setfield`。
pub fn setfield(l: L, idx: c_int, k: &'static [u8]) {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // Safety: `l` 存活（模块级契约）；`key` 切片在调用返回前有效，被调侧不得逃逸
  unsafe { (*l).set_field_bytes(idx, key) }
}

/// `lua_isboolean!`：栈位是否为 boolean（复用 [`type_]`，不新增边界）。
pub fn isboolean(l: L, idx: c_int) -> bool {
  type_(l, idx) == LuaType::Boolean as i32
}

/// `luaL_newmetatable(l, name)`：压入该名字的元表并返回存在性码。
pub fn newmetatable(l: L, name: &'static [u8]) -> c_int {
  let name = name.strip_suffix(b"\0").unwrap_or(name);
  // Safety: `l` 存活（模块级契约）；`name` 为去 NUL 后的合法字节串。
  unsafe { (*l).new_metatable_by_bytes(name) }
}

/// `LUA_PUSHCFUNCTION(l, f, name)`：把带调试名的 C 函数压栈（不挂全局，供元方法等落位）。
pub fn pushcfunction_named(l: L, f: LuaCFunction, name: &'static [u8]) {
  // Safety: `l` 存活；`f` 遵循 Lua C 函数约定；`cstr` NUL 结尾静态名串。
  unsafe { (*l).push_c_function(f, cstr(name)) }
}

/// `lua_setuserdatametatable(l, tag)`。
pub fn setuserdatametatable(l: L, tag: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_setuserdatametatable(l, tag) }
}

/// `lua_registeruserdatadirectfieldget(l, tag, field, fn_)`：注册 userdata 直接字段取数回调。
pub fn register_direct_field_get(
  l: L,
  tag: c_int,
  field: &'static [u8],
  fn_: LuaUserdataDirectFieldGet,
) {
  // Safety: `l` 存活；`tag` 界内；`cstr` NUL 结尾静态字段名；`fn_` 遵循直接字段取数约定。
  unsafe { lua_registeruserdatadirectfieldget(l, tag, cstr(field), fn_) }
}

// ---------------------------------------------------------------------------
// lightuserdata / tagged userdata 族（cpp Conformance.test.cpp userdata 用例收口）
// ---------------------------------------------------------------------------

/// `lua_pushlightuserdatatagged(l, p, tag)`。
pub fn pushlightuserdatatagged(l: L, p: *mut c_void, tag: c_int) {
  // Safety: `l` 存活（模块级契约）；载荷指针原样入栈不被解引用。
  unsafe { lua_pushlightuserdatatagged(l, p, tag) }
}

/// `lua_lightuserdatatag(l, idx)`。
pub fn lightuserdatatag(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活；`idx` 为 lightuserdata 槽（用例契约）。
  unsafe { lua_lightuserdatatag(l, idx) }
}

/// `lua_tolightuserdatatagged(l, idx, tag)`：tag 不符或非 lightuserdata 得 `None`。
pub fn tolightuserdatatagged(l: L, idx: c_int, tag: c_int) -> Option<*mut c_void> {
  // Safety: `l` 存活；`idx` 在使用点未被改栈失效。
  unsafe { lua_tolightuserdatatagged(l, idx, tag) }
}

/// `lua_tolightuserdata(l, idx)`：非 lightuserdata 得 NULL。
pub fn tolightuserdata(l: L, idx: c_int) -> *mut c_void {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_tolightuserdata(l, idx) }
}

/// `lua_setlightuserdataname(l, tag, name)`。
pub fn setlightuserdataname(l: L, tag: c_int, name: &'static [u8]) {
  // Safety: `l` 存活；`tag` 界内；`cstr` NUL 结尾静态名串。
  unsafe { lua_setlightuserdataname(l, tag, cstr(name)) }
}

/// `lua_getlightuserdataname(l, tag)`：未注册得 `None`，否则为登记名原始字节。
pub fn getlightuserdataname<'a>(l: L, tag: c_int) -> Option<&'a [u8]> {
  // Safety: `l` 存活；`tag` 界内；返回 NULL 或登记时的 NUL 结尾静态串。
  let p = unsafe { lua_getlightuserdataname(l, tag) };
  if p.is_null() {
    None
  } else {
    Some(unsafe { cstr_bytes(p) })
  }
}

/// `lua_rawequal(l, a, b)`。
pub fn rawequal(l: L, a: c_int, b: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).raw_equal(a, b) as c_int }
}

/// `lua_createtable(l, narray, nrec)`。
pub fn createtable(l: L, narray: c_int, nrec: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_createtable(l, narray, nrec) }
}

/// `lua_settable`（键值在栈顶，idx 指向目标表）。
pub fn settable(l: L, idx: c_int) {
  // Safety: `l` 存活且栈顶两槽为键值对（用例配平契约）。
  unsafe { lua_settable(l, idx) }
}

/// `lua_newuserdatatagged(l, sz, tag)`：返回 VM 持有的数据块裸指针。
pub fn newuserdatatagged(l: L, sz: usize, tag: c_int) -> *mut c_void {
  // Safety: `l` 存活；`tag` 界内（模块级契约）。
  unsafe { lua_newuserdatatagged(l, sz, tag) }
}

/// `lua_newuserdatadtor(l, sz, dtor)`：带内联析构的 userdata。
pub fn newuserdatadtor(l: L, sz: usize, dtor: UserdataDtorRaw) -> *mut c_void {
  // Safety: `l` 存活；`dtor` 遵循 Lua 析构 C 约定（用例桩函数自带）。
  unsafe { lua_newuserdatadtor(l, sz, dtor) }
}

/// `lua_newuserdatataggedwithmetatable(l, sz, tag)`：直接挂 tag 全局元表。
pub fn newuserdatataggedwithmetatable(l: L, sz: usize, tag: c_int) -> *mut c_void {
  // Safety: `l` 存活；`tag` 界内且已注册全局元表（用例契约）。
  unsafe { lua_newuserdatataggedwithmetatable(l, sz, tag) }
}

/// `lua_userdatatag(l, idx)`。
pub fn userdatatag(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活；`idx` 为 userdata 槽（用例契约）。
  unsafe { lua_userdatatag(l, idx) }
}

/// `lua_setuserdatatag(l, idx, tag)`。
pub fn setuserdatatag(l: L, idx: c_int, tag: c_int) {
  // Safety: `l` 存活；`idx` 为 userdata 槽、`tag` 界内（用例契约）。
  unsafe { lua_setuserdatatag(l, idx, tag) }
}

/// `lua_touserdatatagged(l, idx, tag)`：tag 不符得 NULL。
pub fn touserdatatagged(l: L, idx: c_int, tag: c_int) -> *mut c_void {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_touserdatatagged(l, idx, tag) }
}

/// `lua_touserdata(l, idx)`：非 userdata 得 `None`。
pub fn touserdata<'a>(l: L, idx: c_int) -> Option<&'a mut c_void> {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_touserdata(l, idx) }
}

/// `lua_getuserdatadtor(l, tag)`：读回 tag 注册的全局析构。
pub fn getuserdatadtor(l: L, tag: i32) -> LuaDestructor {
  // Safety: `l` 存活；`tag` 界内。
  unsafe { lua_getuserdatadtor(l, tag) }
}

/// `lua_setuserdatadtor(l, tag, dtor)`：注册 tag 全局析构。
pub fn setuserdatadtor(l: L, tag: i32, dtor: LuaDestructor) {
  // Safety: `l` 存活；`tag` 界内；`dtor` 遵循 Lua 析构 C 约定。
  unsafe { lua_setuserdatadtor(l, tag, dtor) }
}

/// `lua_getuserdatametatable(l, tag)`：把 tag 全局元表压栈。
pub fn getuserdatametatable(l: L, tag: c_int) {
  // Safety: `l` 存活；`tag` 界内。
  unsafe { lua_getuserdatametatable(l, tag) }
}

/// `luaL_getmetatable(l, name)`：把注册表里该名字的元表压栈。
pub fn l_getmetatable(l: L, name: &'static [u8]) -> c_int {
  let name = name.strip_suffix(b"\0").unwrap_or(name);
  // Safety: `l` 存活（模块级契约）；`name` 为去 NUL 后的合法字节串。
  unsafe { (*l).get_metatable_by_bytes(name) }
}

/// `luaL_checkudata(l, idx, tname)`：按元表名校验 userdata，返回数据块裸指针。
pub fn l_checkudata(l: L, idx: c_int, tname: &str) -> *mut c_void {
  // Safety: `l` 存活；`idx` 在使用点未被改栈失效。
  unsafe { lua_l_checkudata(l, idx, tname) }
}

/// 把 [`newuserdatatagged`] / [`newuserdata`] 等交回的 VM 数据块按 `T` 原位写入。
/// 前置条件（由用例保证）：`p` 指向容量不小于 `T`、对齐满足 `T` 的存活 VM 块。
pub fn poke<T: Copy>(p: *mut c_void, v: T) {
  // Safety: 契约——`p` 为刚分配、尚在栈上的 VM 块，尺寸/对齐由用例的 `sz` 与 `T` 配对保证。
  unsafe { write(p.cast::<T>(), v) }
}

/// `lua_rawgetfield`。
pub fn rawgetfield(l: L, idx: c_int, k: &'static [u8]) -> c_int {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // Safety: `l` 存活（模块级契约）；`key` 切片在调用返回前有效，被调侧不得逃逸
  unsafe { lua_rawgetfield_bytes(l, idx, key) }
}

/// `lua_rawsetfield`。
pub fn rawsetfield(l: L, idx: c_int, k: &'static [u8]) {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // Safety: `l` 存活（模块级契约）；`key` 切片在调用返回前有效，被调侧不得逃逸
  unsafe { lua_rawsetfield_bytes(l, idx, key) }
}

/// `lua_rawget`（键在栈顶）。
pub fn rawget(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawget(l, idx) }
}

/// `lua_gettable`（键在栈顶）。
pub fn gettable(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_gettable(l, idx) }
}

/// `lua_rawgeti`。
pub fn rawgeti(l: L, idx: c_int, n: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawgeti(l, idx, n) }
}

/// `lua_rawseti`。
pub fn rawseti(l: L, idx: c_int, n: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawseti(l, idx, n) }
}

/// `lua_rawgetptagged`。
pub fn rawgetptagged(l: L, idx: c_int, p: *mut c_void, tag: c_int) -> c_int {
  // Safety: `l` 存活；`p` 为用例持有的载荷指针（仅比较，不解引用）。
  unsafe { lua_rawgetptagged(l, idx, p, tag) }
}

/// `lua_rawsetptagged`。
pub fn rawsetptagged(l: L, idx: c_int, p: *mut c_void, tag: c_int) {
  // Safety: 同 [`rawgetptagged`]。
  unsafe { lua_rawsetptagged(l, idx, p, tag) }
}

/// `lua_rawgetp`（tag 0，宏形态）。
pub fn rawgetp(l: L, idx: c_int, p: *mut c_void) -> c_int {
  // Safety: `l` 存活；`p` 仅比较不解引用。
  unsafe { (*l).raw_get_ptr(idx, p) }
}

/// `lua_rawsetp`（tag 0，宏形态）。
pub fn rawsetp(l: L, idx: c_int, p: *mut c_void) {
  // Safety: `l` 存活；`p` 仅存储不解引用。
  unsafe { (*l).raw_set_ptr(idx, p) }
}

/// `lua_clonefunction`。
pub fn clonefunction(l: L, idx: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_clonefunction(l, idx) }
}

/// `lua_clonetable`。
pub fn clonetable(l: L, idx: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_clonetable(l, idx) }
}

/// `lua_cleartable`。
pub fn cleartable(l: L, idx: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_cleartable(l, idx) }
}

/// `lua_setmetatable`。
pub fn setmetatable(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { (*l).set_metatable(idx) }
}

/// `lua_setfenv`。
pub fn setfenv(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_setfenv(l, idx) }
}

// ---------------------------------------------------------------------------
// buffer / 指针读取
// ---------------------------------------------------------------------------

/// `lua_newbuffer`：返回数据块裸指针（VM 持有）。
pub fn newbuffer(l: L, sz: usize) -> *mut c_void {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_newbuffer(l, sz) }
}

/// `lua_tobuffer` 只取址形态（C 侧 NULL 长出参边界契约，既有约定 review.md §2）。
pub fn tobuffer_ptr(l: L, idx: c_int) -> Option<*mut c_void> {
  // Safety: `l` 存活；长度出参传 null 即「仅取址」分支。
  unsafe { lua_tobuffer(l, idx, null_mut()).map(|r| r as *mut c_void) }
}

/// `lua_tobuffer` 取址兼长度出参形态。
pub fn tobuffer_len(l: L, idx: c_int, len: &mut usize) -> Option<*mut c_void> {
  // Safety: `l` 存活；`len` 为本帧可写出参。
  unsafe { lua_tobuffer(l, idx, len).map(|r| r as *mut c_void) }
}

/// `luaL_checkbuffer` 只取址形态（C 侧 NULL 长度出参边界契约，既有约定 review.md §2）。
pub fn l_checkbuffer_ptr(l: L, narg: c_int) -> *mut c_void {
  // Safety: `l` 存活；`narg` 为 buffer 槽（用例断言前置）；null 长度出参走仅取址分支。
  unsafe { lua_l_checkbuffer(l, narg, null_mut()) }
}

/// `luaL_checkbuffer` 取址兼长度出参形态。
pub fn l_checkbuffer_len(l: L, narg: c_int, len: &mut usize) -> *mut c_void {
  // Safety: `l` 存活；`len` 为本帧可写出参。
  unsafe { lua_l_checkbuffer(l, narg, len) }
}

/// `lua_topointer`。
pub fn topointer(l: L, idx: c_int) -> *const c_void {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_topointer(l, idx) }
}

// ---------------------------------------------------------------------------
// 全局表读写（LUA_GLOBALSINDEX 伪索引收口）
// ---------------------------------------------------------------------------

/// `lua_setglobal`：把栈顶值写入全局 `k` 并弹栈。
pub fn setglobal(l: L, k: &'static [u8]) {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // Safety: `l` 存活（模块级契约）；`key` 切片在调用返回前有效，被调侧不得逃逸
  unsafe { (*l).set_global_bytes(key) }
}

// ---------------------------------------------------------------------------
// 状态生命周期
// ---------------------------------------------------------------------------

/// `lua_newstate`：交回裸句柄，调用方须立即交给 [`StateRef`]（或经
/// [`run_conformance`](super::run_conformance::run_conformance) 持有）。`ud` 为
/// 用例持有的载荷指针，仅在状态存活期被分配器解引用。
pub fn newstate(f: LuaAlloc, ud: *mut c_void) -> L {
  // Safety: 分配器 `f` 遵循 Lua 分配器 C ABI（用例侧桩函数自带该契约）。
  unsafe { lua_newstate(f, ud) }
}

/// `lua_getallocf` 成对读取：分配器与载荷指针（C 侧 NULL 出参收口为返回值；`ud` 局部初值仅为 c-API `&mut` 出参落点，既有约定 review.md §2）。
pub fn getallocf(l: L) -> (LuaAlloc, *mut c_void) {
  let mut ud: *mut c_void = null_mut();
  // Safety: `l` 存活；`ud` 为本帧可写出参。
  let f = unsafe { lua_getallocf(l, &mut ud) };
  (f, ud)
}

/// `lua_isbuffer!(l, idx)`。
pub fn isbuffer(l: L, idx: c_int) -> bool {
  // Safety: `l` 存活（模块级契约）。
  type_(l, idx) == LuaType::Buffer as i32
}

// ---------------------------------------------------------------------------
// 常用值读取（`lua_tointeger`/`lua_tostring`/...）
// ---------------------------------------------------------------------------

/// `lua_tostring` 的 `Option<&'a str>` 视图：非字符串槽得 `None`。
pub fn to_str<'a>(l: L, idx: c_int) -> Option<&'a str> {
  // Safety: `l` 存活（模块级契约）；返回借用指向 VM 栈缓冲。
  unsafe { (*l).to_str(idx) }
}

/// `lua_getreadonly(l, idx)`（C 侧 0/非 0）。
pub fn getreadonly(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_getreadonly(l, idx) }
}

// ---------------------------------------------------------------------------
// 字段 / 全局读写（`&str` 名字形态，cpp `lua_setfield(l, idx, "…")`）
// ---------------------------------------------------------------------------

/// `lua_setfield` with `&str` 名。
pub fn set_field_str(l: L, idx: c_int, k: &str) {
  // Safety: `l` 存活（模块级契约）；`k` 借用仅在调用期内有效。
  unsafe { (*l).set_field_str(idx, k) }
}

// ---------------------------------------------------------------------------
// 栈位调整与引用
// ---------------------------------------------------------------------------

/// `lua_getref(l, ref)`：把引用句柄压栈。
pub fn getref(l: L, r: c_int) {
  // Safety: `l` 存活；`r` 为本用例先前 `lua_ref` 交回的引用句柄。
  unsafe { (*l).get_ref(r) }
}

/// `lua_ref(l, idx)`：给 `idx` 处对象建引用并弹栈。
pub fn luaref(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）；`idx` 为可读槽。
  unsafe { lua_ref(l, idx) }
}

/// `lua_unref(l, ref)`：释放引用。
pub fn unref(l: L, r: c_int) {
  // Safety: `l` 存活；`r` 为先前 `lua_ref` 交回的引用句柄。
  unsafe { lua_unref(l, r) }
}

/// `lua_isuserdata`（C 侧 0/非 0）。
pub fn isuserdata(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_isuserdata(l, idx) }
}

// ---------------------------------------------------------------------------
// lightuserdata / closure / 错误
// ---------------------------------------------------------------------------

/// `lua_pushcfunction(l, f)`（无调试名）：与 [`pushcfunction`] 同形，接受 `Option<LuaCFunction>`。
pub fn push_c_function(l: L, f: LuaCFunction, name: Option<&'static [u8]>) {
  let name = name.map_or_else(null, cstr);
  // Safety: `l` 存活；`name` 为 `cstr` 转的 NUL 结尾静态串或 NULL，二者均与底层 C 契约一致。
  unsafe { (*l).push_c_function(f, name) }
}

/// `lua_pushcclosure(l, f, nup)`：栈顶须有 `nup` 个上值。
pub fn push_c_closure(l: L, f: LuaCFunction, name: &'static [u8], nup: c_int) {
  // Safety: `l` 存活；栈顶 `nup` 个上值由用例配平；`cstr` NUL 结尾静态名串。
  unsafe { (*l).push_c_closure(f, cstr(name), nup) }
}

// ---------------------------------------------------------------------------
// 完整生命周期整链（收口 openlibs+sandbox+sandboxthread 与 compile+luau_load 组合）
// ---------------------------------------------------------------------------

/// cpp `openlibs+sandbox+sandboxthread` 三段前置：把库和沙箱一次装上。
pub fn openlibs_and_sandbox_all(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe {
    lua_l_openlibs(l);
    lua_l_sandbox(l);
    lua_l_sandboxthread(l);
  }
}

/// `lua_c_enumheap`：堆遍历的一次性收口。
pub fn enumheap(
  l: L,
  context: *mut c_void,
  node: Option<unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, u8, u8, usize, *const c_char)>,
  edge: Option<unsafe extern "C-unwind" fn(*mut c_void, *mut c_void, *mut c_void, *const c_char)>,
) {
  // Safety: `l` 存活；`context` 为用例独占的载荷指针，遍历期间对 C 回调全程有效。
  unsafe { lua_c_enumheap(l, context, node, edge) }
}

/// `lua_gc(l, LUA_GCCOLLECT, 0)` 的语义收口。
pub fn fullgc(l: L) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_c_fullgc(l) }
}

/// `lua_dump(l, f, category_name)`。`f` 为 `FILE*` 擦除后的 `*mut c_void`。
pub fn dump(
  l: L,
  f: *mut c_void,
  category_name: Option<unsafe extern "C-unwind" fn(*mut LuaState, u8) -> *const c_char>,
) {
  // Safety: `l` 存活；`f` 在使用点仍指向可写目标；`category_name` 为 None 即 C 侧 NULL。
  unsafe { lua_c_dump(l, f, category_name) }
}
