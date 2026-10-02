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
  ffi::{c_char, c_int, c_uint, c_void},
  mem::zeroed,
  ptr::{from_mut, null, null_mut, write},
  slice::from_raw_parts,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
use ulua_code_gen::{
  functions::{
    compile_internal::compile_internal, get_assembly::get_assembly,
    luau_codegen_create::luau_codegen_create,
  },
  records::{
    assembly_options::AssemblyOptions, compilation_options::CompilationOptions,
    compilation_result::CompilationResult, compilation_stats::CompilationStats,
    lowering_stats::LoweringStats,
  },
  type_aliases::module_id::ModuleId,
};
use ulua_common::functions::c_str::cstr_bytes;
use ulua_compiler::{functions::compile::compile, records::compile_options::CompileOptions};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_break::lua_break,
    lua_breakpoint::lua_breakpoint,
    lua_c_allocationrate::lua_c_allocationrate,
    lua_c_dump::lua_c_dump,
    lua_c_enumheap::lua_c_enumheap,
    lua_c_fullgc::lua_c_fullgc,
    lua_c_validate::lua_c_validate,
    lua_callbacks::lua_callbacks,
    lua_callyieldable_impl::lua_callyieldable,
    lua_checkstack::lua_checkstack,
    lua_cleartable::lua_cleartable,
    lua_clonefunction::lua_clonefunction,
    lua_clonetable::lua_clonetable,
    lua_cpcall::lua_cpcall,
    lua_createtable::lua_createtable,
    lua_debugtrace::lua_debugtrace,
    lua_equal::lua_equal,
    lua_g_isnative::lua_g_isnative,
    lua_gc::lua_gc,
    lua_getallocf::lua_getallocf,
    lua_getargument::lua_getargument,
    lua_getcoverage::lua_getcoverage,
    lua_getinfo::lua_getinfo,
    lua_getlightuserdataname::lua_getlightuserdataname,
    lua_getlocal::lua_getlocal,
    lua_getreadonly::lua_getreadonly,
    lua_gettable::lua_gettable,
    lua_getupvalue::lua_getupvalue,
    lua_getuserdatadtor::lua_getuserdatadtor,
    lua_getuserdatametatable::lua_getuserdatametatable,
    lua_is_lfunction::lua_is_lfunction,
    lua_isnumber::lua_isnumber,
    lua_isstring::lua_isstring,
    lua_isuserdata::lua_isuserdata,
    lua_isyieldable::lua_isyieldable,
    lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checklstring::lua_l_checklstring_ref,
    lua_l_checkoption::lua_l_checkoption,
    lua_l_checkstack::lua_l_checkstack,
    lua_l_checkudata::lua_l_checkudata,
    lua_l_checkunsigned::lua_l_checkunsigned,
    lua_l_checkvector::lua_l_checkvector,
    lua_l_openlibs::lua_l_openlibs,
    lua_l_optboolean::lua_l_optboolean,
    lua_l_optinteger::lua_l_optinteger,
    lua_l_register::lua_l_register,
    lua_l_sandbox::lua_l_sandbox,
    lua_l_sandboxthread::lua_l_sandboxthread,
    lua_l_typeerror_l::lua_l_typeerror_l,
    lua_l_typename::lua_l_typename,
    lua_lightuserdatatag::lua_lightuserdatatag,
    lua_namecallatom::lua_namecallatom,
    lua_newbuffer::lua_newbuffer,
    lua_newstate::lua_newstate,
    lua_newthread::lua_newthread,
    lua_newuserdatadtor::lua_newuserdatadtor,
    lua_newuserdatatagged::lua_newuserdatatagged,
    lua_newuserdatataggedwithmetatable::lua_newuserdatataggedwithmetatable,
    lua_pcallyieldable::lua_pcallyieldable,
    lua_pushcclosurek::lua_pushcclosurek,
    lua_pushlightuserdatatagged::lua_pushlightuserdatatagged,
    lua_pushlstring::lua_pushlstring_bytes,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
    lua_rawget::lua_rawget,
    lua_rawgetfield::lua_rawgetfield_bytes,
    lua_rawgeti::lua_rawgeti,
    lua_rawgetptagged::lua_rawgetptagged,
    lua_rawiter::lua_rawiter,
    lua_rawsetfield::lua_rawsetfield_bytes,
    lua_rawseti::lua_rawseti,
    lua_rawsetptagged::lua_rawsetptagged,
    lua_ref::lua_ref,
    lua_registeruserdatadirectaccess::lua_registeruserdatadirectaccess,
    lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget,
    lua_resumeerror::lua_resumeerror,
    lua_setfenv::lua_setfenv,
    lua_setlightuserdataname::lua_setlightuserdataname,
    lua_setsafeenv::lua_setsafeenv,
    lua_settable::lua_settable,
    lua_setuserdatadtor::lua_setuserdatadtor,
    lua_setuserdatametatable::lua_setuserdatametatable,
    lua_setuserdatatag::lua_setuserdatatag,
    lua_singlestep::lua_singlestep,
    lua_status::lua_status,
    lua_tobuffer::lua_tobuffer,
    lua_tolightuserdata::lua_tolightuserdata,
    lua_tolightuserdatatagged::lua_tolightuserdatatagged,
    lua_tolstring::lua_tolstring,
    lua_tonumberx::lua_tonumberx,
    lua_topointer::lua_topointer,
    lua_tostringatom::lua_tostringatom,
    lua_tothread::lua_tothread,
    lua_touserdata::lua_touserdata,
    lua_touserdatatagged::lua_touserdatatagged,
    lua_type::lua_type,
    lua_typename::lua_typename,
    lua_unref::lua_unref,
    lua_userdatadirectfield_setboolean::lua_userdatadirectfield_setboolean,
    lua_userdatadirectfield_setnumber::lua_userdatadirectfield_setnumber,
    lua_userdatatag::lua_userdatatag,
    lua_xmove::lua_xmove,
    lua_yield::lua_yield,
    luaopen_base::luaopen_base,
    luau_callhook::luau_callhook,
    luau_load::luau_load,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  records::{
    lua_callbacks::LuaCallbacks, lua_debug::LuaDebug, lua_l_reg::LuaLReg, lua_state::LuaState,
  },
  type_aliases::{
    lua_alloc::LuaAlloc, lua_c_function::LuaCFunction, lua_continuation::LuaContinuation,
    lua_coverage::LuaCoverage, lua_destructor::LuaDestructor, lua_hook::LuaHook,
    lua_userdata_direct_access::LuaUserdataDirectAccess,
    lua_userdata_direct_field_get::LuaUserdataDirectFieldGet,
    lua_userdata_direct_namecall::LuaUserdataDirectNamecall,
  },
};

use crate::common::functions::{cpcall_test::cpcall_test, cstr::cstr, cstr_text::diagnostic_text};

/// C ABI 状态句柄别名；仅在本门面内部被解引用（每函数恰一次 `unsafe`）。
pub type L = *mut LuaState;

/// 「判空 + 引用重建」单点收口：把 [`L`] 转回 `&LuaState`，供 `LuaState` 的固有
/// 安全方法与 ulua-vm 已收口的引用形 `lua_*` 自由函数直接调用。
///
/// 这是全 conformance 对 `(*l)` 共享解引用的**唯一**发生点（对照 ulua-rt 的
/// `StateView` 驱动视图：以 NonNull 编码非空、单一收口点恢复引用语义；测试侧
/// 无 `Rc` 句柄可锚定生命周期，故以模块级 `l` 契约 + 判空断言承担同等论证）。
/// 空指针是调用方违约，当场 panic（比后续空指针解引用 UB 更响亮的等价失败）。
pub fn state_ref<'a>(l: L) -> &'a LuaState {
  assert!(!l.is_null(), "LuaState handle must not be null");
  // Safety: 模块级 `l` 契约（存活、本用例独占驱动）由判空断言 + 调用点维持。
  unsafe { &*l }
}

/// [`state_ref`] 的独占形态：全 conformance 对 `(*l)` 可变解引用的唯一发生点。
///
/// 返回引用的生命周期不由编译器证明（裸指针进、引用出），其正确性即模块级
/// `l` 契约：同一 state 的引用只在单线程串行驱动中顺序使用，从不重叠持有——
/// 与 ulua-rt `StateView::deref_mut` 的驱动契约同语义。
pub fn state_mut<'a>(l: L) -> &'a mut LuaState {
  assert!(!l.is_null(), "LuaState handle must not be null");
  // Safety: 同 [`state_ref`]；独占由测试单线程串行驱动纪律保证。
  unsafe { &mut *l }
}

/// `lua_callbacks` 的单点收口：交回该状态的回调表可变引用（判空 + 引用重建）。
/// 调用方只做具名字段赋值（`interrupt`/`debugbreak`/...），不再解引用裸表指针。
pub fn callbacks_mut<'a>(l: L) -> &'a mut LuaCallbacks {
  // Safety: 模块级 `l` 契约；`lua_callbacks` 返回随 `l` 存活的回调表指针。
  unsafe { &mut *lua_callbacks(l) }
}

/// `lua_newuserdatadtor` 的内联析构类型（与 `LuaDestructor` 一致）。
pub type UserdataDtorRaw = LuaDestructor;

/// `lua_newthread` 的返回值同样是存活线程栈，归父状态持有；用例经门面取得后
/// 可安全作为后续 [`L`] 参数。
pub type AtomAssignFn = Option<unsafe extern "C-unwind" fn(L, *const c_char, usize) -> i16>;

// ---------------------------------------------------------------------------
// 栈管理
// ---------------------------------------------------------------------------

/// `lua_gettop`：当前栈深。
pub fn gettop(l: L) -> c_int {
  state_ref(l).get_top()
}

/// `lua_settop`：截断 / 填充到 `idx`。
pub fn settop(l: L, idx: c_int) {
  state_mut(l).set_top(idx)
}

/// `lua_pop`：弹掉栈顶 `n` 槽。
pub fn pop(l: L, n: c_int) {
  state_mut(l).pop(n)
}

/// `lua_checkstack`：尝试扩容 `size` 槽，返回 C 侧布尔（0/非 0）。
pub fn checkstack(l: L, size: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_checkstack(l, size) }
}

/// `luaL_checkstack`：扩容失败即抛 Lua 错误（消息 `msg`）。
pub fn l_checkstack(l: L, space: c_int, msg: &str) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_l_checkstack(&mut *l, space, msg) }
}

/// `lua_pushvalue`：复制槽 `idx` 压栈。
pub fn pushvalue(l: L, idx: c_int) {
  state_mut(l).push_value(idx)
}

/// `lua_pushnumber`。
pub fn pushnumber(l: L, n: f64) {
  state_mut(l).push_number(n)
}

/// `lua_pushinteger`。
pub fn pushinteger(l: L, n: c_int) {
  state_mut(l).push_integer(n)
}

/// `lua_pushboolean`（保持 C 侧 `int` 参数语义）。
pub fn pushboolean(l: L, b: c_int) {
  state_mut(l).push_boolean(b != 0)
}

/// `lua_pushnil`。
pub fn pushnil(l: L) {
  state_mut(l).push_nil()
}

/// `lua_pushstring`，名字参数为静态字节串。
pub fn pushstr(l: L, s: &'static [u8]) {
  let s = s.strip_suffix(b"\0").unwrap_or(s);
  state_mut(l).push_bytes(s)
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
  callbacks_mut(l).useratom = f
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

/// `lua_resumeerror(co, from)`：向协程 `co` 注入来自 `from` 的错误恢复。
pub fn resumeerror(co: L, from: L) -> c_int {
  // Safety: `co`/`from` 均存活且同属一个 VM（模块级契约 + 用例布线）。
  unsafe { lua_resumeerror(co, from) }
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
  state_mut(l).new_table()
}

/// `lua_newthread`：在 `l` 上派生线程栈并压栈，返回其裸指针（用例随后按 [`L`] 用）。
pub fn newthread(l: L) -> L {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_newthread(l) }
}

/// `lua_newuserdata`（tag 0）：返回数据块裸指针，由 VM 持有。
pub fn newuserdata(l: L, sz: usize) -> *mut c_void {
  state_mut(l).new_userdata(sz)
}

/// `lua_concat`。
pub fn concat(l: L, n: c_int) {
  state_mut(l).concat(n)
}

// ---------------------------------------------------------------------------
// 调用
// ---------------------------------------------------------------------------

/// `lua_call`： unprotected 调用（Lua 侧错误会经 unwind 穿出本层，与直调一致）。
pub fn call(l: L, nargs: c_int, nresults: c_int) {
  state_mut(l).call(nargs, nresults)
}

/// `lua_pcall`：返回 [`LuaStatus`] 编码的 `c_int`。
pub fn pcall(l: L, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int {
  state_mut(l).pcall(nargs, nresults, errfunc)
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
  unsafe { lua_type(&*l, idx) }
}

/// `lua_typename(t)` 的原始字节视图。
pub fn typename_bytes<'a>(l: L, t: c_int) -> &'a [u8] {
  // Safety: `l` 存活；`lua_typename` 返回 VM 持有的 NUL 结尾常量串。
  unsafe { cstr_bytes(lua_typename(l, t)) }
}

/// `luaL_typename(l, idx)` 的原始字节视图。
pub fn l_typename_bytes<'a>(l: L, idx: c_int) -> &'a [u8] {
  // Safety: `l` 存活；返回 NUL 结尾类型名串（VM 内部缓冲）。
  unsafe { cstr_bytes(lua_l_typename(&*l, idx)) }
}

/// `lua_tostring!(l, idx)` + 原始字节收口（要求 `idx` 为字符串，与旧断言同前提）。
pub fn to_bytes<'a>(l: L, idx: c_int) -> &'a [u8] {
  // `idx` 为字符串槽（用例断言前置）；返回 NUL 结尾 VM 缓冲。
  state_mut(l).to_bytes(idx).unwrap_or_default()
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
  state_mut(l).check_bytes(idx)
}

/// `lua_tonumber!(l, idx)`（`lua_tonumberx(..).unwrap_or(0.0)`）。
pub fn tonumber(l: L, idx: c_int) -> f64 {
  // Safety: `l` 存活（模块级契约）；非数值回报 0，与宏一致。
  unsafe { lua_tonumberx(&*l, idx).unwrap_or(0.0) }
}

/// `lua_l_checkinteger`：非整数即抛 Lua 错误。
pub fn l_checkinteger(l: L, idx: c_int) -> c_int {
  state_mut(l).check_integer(idx)
}

/// `lua_toboolean`。
pub fn toboolean(l: L, idx: c_int) -> c_int {
  state_ref(l).to_boolean(idx) as c_int
}

/// `lua_isstring`（C 侧 0/非 0）。
pub fn isstring(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_isstring(&*l, idx) }
}

/// `lua_isnumber`（C 侧 0/非 0）。
pub fn isnumber(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_isnumber(&*l, idx) }
}

/// `lua_status`。
pub fn status(l: L) -> c_int {
  // `lua_status` 已是 ulua-vm 安全签名（`&LuaState`），经单点重建直接调用。
  lua_status(state_ref(l))
}

/// `lua_equal`（C 侧 0/1）。
pub fn equal(l: L, a: c_int, b: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_equal(&mut *l, a, b) }
}

/// `lua_objlen`。
pub fn objlen(l: L, idx: c_int) -> c_int {
  state_ref(l).obj_len(idx) as c_int
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
  state_mut(l).next(idx) as c_int
}

/// `lua_rawiter` 索引遍历。
pub fn rawiter(l: L, idx: c_int, iter: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawiter(&mut *l, idx, iter) }
}

/// `lua_getfield`。
pub fn getfield(l: L, idx: c_int, k: &'static [u8]) -> c_int {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // `key` 切片在调用返回前有效，被调侧不得逃逸。
  state_mut(l).get_field_bytes(idx, key)
}

/// `lua_setfield`。
pub fn setfield(l: L, idx: c_int, k: &'static [u8]) {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // `key` 切片在调用返回前有效，被调侧不得逃逸。
  state_mut(l).set_field_bytes(idx, key)
}

/// `lua_isboolean!`：栈位是否为 boolean（复用 [`type_]`，不新增边界）。
pub fn isboolean(l: L, idx: c_int) -> bool {
  type_(l, idx) == LuaType::Boolean as i32
}

/// `luaL_newmetatable(l, name)`：压入该名字的元表并返回存在性码。
pub fn newmetatable(l: L, name: &'static [u8]) -> c_int {
  let name = name.strip_suffix(b"\0").unwrap_or(name);
  state_mut(l).new_metatable_by_bytes(name)
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
  state_ref(l).raw_equal(a, b) as c_int
}

/// `lua_createtable(l, narray, nrec)`。
pub fn createtable(l: L, narray: c_int, nrec: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_createtable(l, narray, nrec) }
}

/// `lua_settable`（键值在栈顶，idx 指向目标表）。
pub fn settable(l: L, idx: c_int) {
  // Safety: `l` 存活且栈顶两槽为键值对（用例配平契约）。
  unsafe { lua_settable(&mut *l, idx) }
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
  state_mut(l).get_metatable_by_bytes(name)
}

/// `luaL_checkudata(l, idx, tname)`：按元表名校验 userdata，返回数据块裸指针。
pub fn l_checkudata(l: L, idx: c_int, tname: &str) -> *mut c_void {
  // Safety: `l` 存活；`idx` 在使用点未被改栈失效。
  unsafe { lua_l_checkudata(&mut *l, idx, tname) }
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
  unsafe { lua_rawgetfield_bytes(&mut *l, idx, key) }
}

/// `lua_rawsetfield`。
pub fn rawsetfield(l: L, idx: c_int, k: &'static [u8]) {
  let key = k.strip_suffix(b"\0").unwrap_or(k);
  // Safety: `l` 存活（模块级契约）；`key` 切片在调用返回前有效，被调侧不得逃逸
  unsafe { lua_rawsetfield_bytes(&mut *l, idx, key) }
}

/// `lua_rawget`（键在栈顶）。
pub fn rawget(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawget(&mut *l, idx) }
}

/// `lua_gettable`（键在栈顶）。
pub fn gettable(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_gettable(&mut *l, idx) }
}

/// `lua_rawgeti`。
pub fn rawgeti(l: L, idx: c_int, n: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawgeti(&mut *l, idx, n) }
}

/// `lua_rawseti`。
pub fn rawseti(l: L, idx: c_int, n: c_int) {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_rawseti(&mut *l, idx, n) }
}

/// `lua_rawgetptagged`。
pub fn rawgetptagged(l: L, idx: c_int, p: *mut c_void, tag: c_int) -> c_int {
  // Safety: `l` 存活；`p` 为用例持有的载荷指针（仅比较，不解引用）。
  unsafe { lua_rawgetptagged(&mut *l, idx, p, tag) }
}

/// `lua_rawsetptagged`。
pub fn rawsetptagged(l: L, idx: c_int, p: *mut c_void, tag: c_int) {
  // Safety: 同 [`rawgetptagged`]。
  unsafe { lua_rawsetptagged(&mut *l, idx, p, tag) }
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
  state_mut(l).set_metatable(idx)
}

/// `lua_setfenv`。
pub fn setfenv(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_setfenv(&mut *l, idx) }
}

// ---------------------------------------------------------------------------
// buffer / 指针读取
//
// r12 T11 裁决（镜像保留）：以下 6 个门面是 C-ABI 镜像契约本体——逐字对应
// `lua_newbuffer`/`lua_tobuffer`/`luaL_checkbuffer`/`lua_topointer` 公共 C 签名，
// 且 `conformance_api_buffer` 的断言对象正是 `(void*, size_t* len)` 出参语义
// （NULL 仅取址分支、非 NULL 写长度、跨调用指针同一性）与 cpp 用例对齐；切片
// 形（`lua_tobuffer_bytes_ref`/`lua_l_checkbuffer_ref`）无法观察出参可观察面，
// 故保留裸形 + 逐门面 Safety 注，不随 ulua-rt 消费方迁移。
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
  unsafe { lua_l_checkbuffer(&mut *l, narg, null_mut()) }
}

/// `luaL_checkbuffer` 取址兼长度出参形态。
pub fn l_checkbuffer_len(l: L, narg: c_int, len: &mut usize) -> *mut c_void {
  // Safety: `l` 存活；`len` 为本帧可写出参。
  unsafe { lua_l_checkbuffer(&mut *l, narg, len) }
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
  // `key` 切片在调用返回前有效，被调侧不得逃逸。
  state_mut(l).set_global_bytes(key)
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
  // 返回借用指向 VM 栈缓冲。
  state_mut(l).to_str(idx)
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
  // `k` 借用仅在调用期内有效。
  state_mut(l).set_field_str(idx, k)
}

// ---------------------------------------------------------------------------
// 栈位调整与引用
// ---------------------------------------------------------------------------

/// `lua_getref(l, ref)`：把引用句柄压栈。
pub fn getref(l: L, r: c_int) {
  // `r` 为本用例先前 `lua_ref` 交回的引用句柄。
  state_mut(l).get_ref(r)
}

/// `lua_ref(l, idx)`：给 `idx` 处对象建引用并弹栈。
pub fn luaref(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）；`idx` 为可读槽。
  unsafe { lua_ref(&mut *l, idx) }
}

/// `lua_unref(l, ref)`：释放引用。
pub fn unref(l: L, r: c_int) {
  // Safety: `l` 存活；`r` 为先前 `lua_ref` 交回的引用句柄。
  unsafe { lua_unref(&mut *l, r) }
}

/// `lua_isuserdata`（C 侧 0/非 0）。
pub fn isuserdata(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活（模块级契约）。
  unsafe { lua_isuserdata(&*l, idx) }
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

/// `lua_allocationrate(l)` 的语义收口。
pub fn allocationrate(l: L) -> i64 {
  // Safety: `l` 存活（模块级契约）。
  lua_c_allocationrate(unsafe { &*l })
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

// ---------------------------------------------------------------------------
// yield / 可让出调用族（cpcall×call 的可 yield 形态，cpp Conformance.test.cpp
// 「passthrough call」/「multiple yields」用例的回调侧收口）
// ---------------------------------------------------------------------------

/// `lua_isyieldable`（C 侧 0/非 0）。
pub fn isyieldable(l: L) -> c_int {
  // Safety: `l` 存活（模块级契约）；只读当前上下文的可让出位。
  unsafe { lua_isyieldable(l) }
}

/// `lua_yield`：以栈顶 `nresults` 个值让出回宿主（只应在可 yield 的 C 回调内调用）。
pub fn yield_(l: L, nresults: c_int) -> c_int {
  // Safety: `l` 存活且处于可让出点（用例均在 continuation 回调内调用）。
  unsafe { lua_yield(l, nresults) }
}

/// `lua_callyieldable`：调用栈上函数，可在其内 yield。
pub fn callyieldable(l: L, nargs: c_int, nresults: c_int) -> c_int {
  // Safety: `l` 存活；`nargs`/`nresults` 满足 `lua_call` 栈元素约束（用例配平）。
  unsafe { lua_callyieldable(l, nargs, nresults) }
}

/// `lua_pcallyieldable`：受保护的可 yield 调用。
pub fn pcallyieldable(l: L, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int {
  // Safety: 同 [`callyieldable`]；pcall 恢复链由 VM 维护。
  unsafe { lua_pcallyieldable(l, nargs, nresults, errfunc) }
}

// ---------------------------------------------------------------------------
// 调试 / 回溯 / 覆盖率族（debugger、interrupt inspection、coverage 用例）
// ---------------------------------------------------------------------------

/// `lua_Debug ar = {}` 的等价收口：`repr(C)` 记录全零初始化（各字段的全零位模式
/// 均合法），随后由 [`getinfo`] 按 what 掩码填写。
pub fn zero_debug() -> LuaDebug {
  // Safety: `LuaDebug` 为全字段可全零表示的 repr(C) 记录（与 cpp `{}` 同形）。
  unsafe { zeroed() }
}

/// `lua_getinfo`：按 `what` 掩码（NUL 结尾静态串，如 `b"f\0"`）填充 `ar`；
/// 返回 0 表示该 `level` 无函数帧。
pub fn getinfo(l: L, level: c_int, what: &'static [u8], ar: &mut LuaDebug) -> c_int {
  // Safety: `l` 存活且调用栈深度覆盖 `level`；`what` 为 NUL 结尾静态串；`ar` 为
  // 本帧存活记录（模块级契约 + 用例栈形）。
  unsafe { lua_getinfo(l, level, cstr(what), ar) }
}

/// `lua_getlocal`：取 `level` 帧第 `n` 个局部变量名字（并压值入栈）；越界得 NULL。
pub fn getlocal(l: L, level: c_int, n: c_int) -> *const c_char {
  // Safety: `l` 存活、`level`/`n` 由用例按 getinfo 结果限定。
  unsafe { lua_getlocal(l, level, n) }
}

/// `lua_getupvalue`：取 `funcindex` 闭包第 `n` 个上值名字（并压值入栈）；越界得 NULL。
pub fn getupvalue(l: L, funcindex: c_int, n: c_int) -> *const c_char {
  // Safety: `l` 存活、`funcindex` 为闭包槽（用例断言前置）。
  unsafe { lua_getupvalue(l, funcindex, n) }
}

/// `lua_getargument`：取 `level` 帧第 `n` 个实参（C 侧 0/非 0）。
pub fn getargument(l: L, level: c_int, n: c_int) -> c_int {
  // Safety: `l` 存活、`level` 为用例核过的帧层级。
  unsafe { lua_getargument(l, level, n) }
}

/// `lua_getcoverage`：按行覆盖回调遍历 `funcindex` 处 Lua 函数。
pub fn getcoverage(l: L, funcindex: c_int, context: *mut c_void, callback: LuaCoverage) {
  // Safety: `l` 存活、`funcindex` 为 Lua 函数槽；`context` 为用例独占载荷指针，
  // 遍历期间对回调全程有效。
  unsafe { lua_getcoverage(l, funcindex, context, callback) }
}

/// `luau_callhook`：以受保护边界临时装一个调用钩子。
pub fn callhook(l: L, hook: LuaHook, userdata: Option<*mut c_void>) {
  // Safety: `l` 存活；钩子签名遵循 C ABI，重入调用后栈形由 VM 恢复。
  unsafe { luau_callhook(l, hook, userdata) }
}

/// `lua_break`：请求在下一个安全点打断执行。
pub fn brk(l: L) -> c_int {
  // Safety: `l` 存活（模块级契约）；只置中断请求位。
  unsafe { lua_break(l) }
}

/// `lua_breakpoint`：对 `funcindex` 闭包的 `line` 行启用/禁用断点。
pub fn breakpoint(l: L, funcindex: c_int, line: c_int, enabled: c_int) -> c_int {
  // Safety: `l` 存活、`funcindex` 处为 Lua 闭包（用例断言前置）。
  unsafe { lua_breakpoint(l, funcindex, line, enabled) }
}

/// `lua_debugtrace` 的原始指针形态：交回 VM 诊断串（或 NULL），由调用侧自行解码。
pub fn debugtrace_raw(l: L) -> *const c_char {
  // Safety: `l` 存活；只读栈打印调用栈诊断，不改动栈。
  unsafe { lua_debugtrace(l) }
}

/// `lua_singlestep`：开关单步执行（已是 ulua-vm 安全签名，经单点重建直调）。
pub fn singlestep(l: L, enabled: bool) {
  lua_singlestep(state_mut(l), enabled as c_int);
}

/// `lua_g_isnative`（C 侧 0/非 0）：`level` 帧是否为原生编译帧。
pub fn g_isnative(l: L, level: c_int) -> c_int {
  // Safety: `l` 存活；`level` 由用例按栈深限定，只读帧槽。
  unsafe { lua_g_isnative(l, level) }
}

/// `lua_is_lfunction`（C 侧 0/非 0）：`idx` 是否为 Lua 闭包。
pub fn is_lfunction(l: L, idx: c_int) -> c_int {
  // Safety: `l` 存活；只读槽位类型标签。
  unsafe { lua_is_lfunction(l, idx) }
}

// ---------------------------------------------------------------------------
// aux 库补集（collectgarbage/loadstring/tables/vector 用例回调侧）
// ---------------------------------------------------------------------------

/// `luaL_checkoption`：在 `lst`（NULL 收尾的 C 串数组）中匹配 `narg` 字符串选项，
/// 返回命中下标（缺省走 `def`，失配抛 Lua 错误）。
pub fn l_checkoption(l: L, narg: c_int, def: *const c_char, lst: *const *const c_char) -> c_int {
  // Safety: `l` 存活；`lst` 为调用方在本帧持有的 NULL 收尾静态选项表。
  unsafe { lua_l_checkoption(&mut *l, narg, def, lst) }
}

/// `luaL_checkunsigned`：非数值即抛 Lua 错误。
pub fn l_checkunsigned(l: L, narg: c_int) -> c_uint {
  // Safety: `l` 存活；`narg` 槽可读。
  unsafe { lua_l_checkunsigned(&mut *l, narg) }
}

/// `luaL_optboolean`：nil 时取 `def`，其余按布尔语义读取。
pub fn l_optboolean(l: L, narg: c_int, def: bool) -> bool {
  // Safety: `l` 存活；`narg` 槽可读。
  unsafe { lua_l_optboolean(&mut *l, narg, def) }
}

/// `luaL_optinteger`：nil 时取 `def`，否则按整数读取（失配抛 Lua 错误）。
pub fn l_optinteger(l: L, narg: c_int, def: c_int) -> c_int {
  // Safety: `l` 存活；`narg` 槽可读。
  unsafe { lua_l_optinteger(&mut *l, narg, def) }
}

/// `luaL_checklstring!` 的字节切片形态：非字符串即抛 Lua 错误。
pub fn l_checklstring<'a>(l: L, narg: c_int) -> &'a [u8] {
  // Safety: `l` 存活；返回借用指向 VM 栈内串，调用点即时消费。
  unsafe { lua_l_checklstring_ref(&mut *l, narg) }
}

/// `luaL_typeerror`：按类型名抛 Lua 错误（不返回）。
pub fn l_typeerror(l: L, narg: c_int, tname: &str) -> ! {
  // Safety: `l` 为存活受保护帧；抛错经 VM 展开发散。
  unsafe { lua_l_typeerror_l(l, narg, tname) }
}

/// `luaL_checkvector` 的切片形态：返回参数 `narg` 处 vector 的
/// `LUA_VECTOR_SIZE` 个 f32 分量（失配抛 Lua 错误）。
pub fn l_checkvector<'a>(l: L, narg: c_int) -> &'a [f32] {
  // Safety: `l` 存活且 `narg` 为 vector 槽（checkvector 失配即抛错）；返回指针
  // 指向该 vector 的分量缓冲，本帧内有效。
  unsafe { from_raw_parts(lua_l_checkvector(&mut *l, narg), LUA_VECTOR_SIZE as usize) }
}

/// `lua_pushvector`（3 分量形态，第 4 分量按构建期布局补 0 由 VM 侧处理）。
pub fn pushvector3(l: L, x: f32, y: f32, z: f32) {
  // Safety: `l` 存活；三个 f32 标量无借用前提，栈头寸由 VM 侧扩容。
  unsafe { lua_pushvector_lua_state_f32_f32_f32(l, x, y, z) }
}

/// `lua_pushvector`（4 分量形态）。
pub fn pushvector4(l: L, x: f32, y: f32, z: f32, w: f32) {
  // Safety: 同 [`pushvector3`]。
  unsafe { lua_pushvector_lua_state_f32_f32_f32_f32(l, x, y, z, w) }
}

/// `lua_pushlstring` 的切片形态（拷贝语义，净压一层）。
pub fn pushlstring(l: L, s: &[u8]) {
  // Safety: `l` 存活；`s` 为本帧合法切片，VM 侧整段拷入堆串；`&mut *l` 一次性
  // 重借用即切片 ref 核心期望的接收者形。
  unsafe { lua_pushlstring_bytes(&mut *l, s) }
}

// ---------------------------------------------------------------------------
// 线程 / 环境杂项
// ---------------------------------------------------------------------------

/// `lua_tothread`：`idx` 处 thread 的状态指针（非 thread 得 `None`）。
pub fn tothread(l: L, idx: c_int) -> Option<L> {
  // Safety: `l` 存活；只读槽位，返回线程状态随该 thread 对象存活。
  unsafe { lua_tothread(l, idx) }
}

/// `lua_xmove`：同 VM 内两线程间搬 `n` 个栈值。
pub fn xmove(from: L, to: L, n: c_int) {
  // Safety: 两侧同属一个存活 VM 且为不同 state、`from` 顶恰有 `n` 个待搬值
  //（用例配平契约）。
  unsafe { lua_xmove(from, to, n) }
}

/// `lua_setsafeenv`：切换 `objindex` 处环境表的 safeenv 标志。
pub fn setsafeenv(l: L, objindex: c_int, enabled: bool) {
  // Safety: `l` 存活、`objindex` 处为 table（用例契约），只翻 safeenv 标志位。
  unsafe { lua_setsafeenv(l, objindex, enabled as c_int) }
}

/// `lua_namecallatom`：读 namecall 原子串（`atom` 出参可选）；非 namecall 调用得 NULL。
pub fn namecallatom(l: L, atom: Option<&mut c_int>) -> *const c_char {
  // Safety: `l` 存活；`atom` 为本帧可写出参（或 None 即不取计数）。
  unsafe { lua_namecallatom(l, atom.map_or_else(null_mut, from_mut)) }
}

// ---------------------------------------------------------------------------
// userdata 直接访问族（DirectAccess 用例回调侧）
// ---------------------------------------------------------------------------

/// `lua_registeruserdatadirectaccess(l, tag, get, set, namecall)`。
pub fn registeruserdatadirectaccess(
  l: L,
  tag: c_int,
  get: LuaUserdataDirectAccess,
  set: LuaUserdataDirectAccess,
  namecall: LuaUserdataDirectNamecall,
) -> c_int {
  // Safety: `l` 存活、`tag` 界内；三个回调均为本用例的 `extern "C-unwind"` 桩。
  unsafe { lua_registeruserdatadirectaccess(l, tag, get, set, namecall) }
}

/// `lua_userdatadirectfield_setnumber(result, n)`：向 direct-field-get 结果槽写数值。
pub fn udfield_setnumber(result: *mut c_void, n: f64) {
  // Safety: `result` 指向本次回调期内独占可写、按 TValue 对齐的槽（回调契约）。
  unsafe { lua_userdatadirectfield_setnumber(result, n) }
}

/// `lua_userdatadirectfield_setboolean(result, b)`：向结果槽写布尔。
pub fn udfield_setboolean(result: *mut c_void, b: c_int) {
  // Safety: 同 [`udfield_setnumber`]。
  unsafe { lua_userdatadirectfield_setboolean(result, b) }
}

// ---------------------------------------------------------------------------
// codegen 门面补集（compile_internal 全参形态 / get_assembly）
// ---------------------------------------------------------------------------

/// `compile_internal` 全参形态：对 `idx` 处闭包做原生编译，返回主编译结果。
pub fn codegen_compile(
  module_id: &Option<ModuleId>,
  l: L,
  idx: c_int,
  options: &CompilationOptions,
  stats: Option<&mut CompilationStats>,
) -> CompilationResult {
  // Safety: `l` 存活、`idx` 为已加载 proto（用例断言前置）；`stats` 为本帧可写结构。
  unsafe { compile_internal(module_id, l, idx, options, stats) }
}

/// `get_assembly`：对 `idx` 处 Lua 闭包按 `options` 反汇编，交回汇编文本。
pub fn assembly(
  l: L,
  idx: c_int,
  options: AssemblyOptions,
  stats: Option<&mut LoweringStats>,
) -> Vec<u8> {
  // Safety: `l` 存活、`idx` 为 Lua 函数槽（cpp `getAssembly` 的 LUAU_ASSERT 同前提）；
  // `stats` 为本帧可写结构。
  unsafe { get_assembly(l, idx, options, stats) }
}
