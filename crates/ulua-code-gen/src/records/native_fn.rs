//! `NativeContext` 回调槽位的 ABI 契约表：类型别名 + 各槽位到具体实现的绑定常量。
//!
//! 本文件命中的裸指针与 `unsafe extern "C-unwind"` 声明属 review.md §2 的**真 ABI 边界**档，
//! 逐条论证后保留（三条同时成立才允许裸指针渗透到这里）：
//!
//! 1. **调用方是 JIT 生成的机器码**，不是 Rust 代码。生成码按 `offset_of!(NativeContext, 槽位)`
//!    用 `ldr` 取出函数指针、`blr` 直调（见 `emit_fallback*` / `emit_inst_call` / `call_*` 系列），
//!    全程不经 Rust 借用系统，故参数只能是值语义的裸地址。
//! 2. **被调方签名是跨 crate 的既有 ABI**：ulua-vm 的 `extern "C-unwind"` 导出壳
//!    （`lua_v_lessthan_export` 等，本文件末尾即以其命名绑定）与 libc（`records::c_math`）。
//!    别名必须与之逐参数一致——窄化成 `&mut T`/trait 会同时破坏固化调用约定与 `#[repr(C)]` 布局。
//! 3. 因此 `*mut LuaState` / `StkId` / `*const TValue` 在这里是**线格式**，不是业务层的指针
//!    别名，也不是可空哨兵：可空性由 `NativeContext` 字段的 `Option<NativeXxxFn>` 表达，未挂钩
//!    即 `None`（全零位，见 `NativeContext::default`），生成码不得调用未挂钩槽位。
//!
//! 参数里的 `*mut c_void` 是导出壳对 `LuaTable` / `Udata` / `Closure` / `UpVal` 等做的不透明
//! 宽化（壳内再转回类型化指针），宽度与表示同类型指针一致，此处无需也不应窄化。
//!
//! 维护契约：新增/改字段时，本文件的别名与绑定常量、`records::native_context::NativeContext`
//! 的字段、`functions::init_functions` 的挂钩三处必须同步。

use core::ffi::c_void;

use ulua_vm::{
  enums::tms::TMS,
  functions::{
    lua_c_barrierback::lua_c_barrierback_export,
    lua_c_barrierf::lua_c_barrierf_export,
    lua_c_barriertable::lua_c_barriertable_export,
    lua_c_step::lua_c_step_export,
    lua_f_close::lua_f_close_export,
    lua_f_findupval::lua_f_findupval_export,
    lua_f_new_lclosure::lua_f_new_lclosure_export,
    lua_h_clone::lua_h_clone_export,
    lua_h_getn::lua_h_getn_export,
    lua_h_new::lua_h_new_export,
    lua_h_resizearray::lua_h_resizearray_export,
    lua_h_setnum::lua_h_setnum_export,
    lua_t_gettm::lua_t_gettm_export,
    lua_t_objtypenamestr::lua_t_objtypenamestr_export,
    lua_v_concat::lua_v_concat_export,
    lua_v_doarithimpl::{
      lua_v_doarithimpl_tm_add, lua_v_doarithimpl_tm_div, lua_v_doarithimpl_tm_idiv,
      lua_v_doarithimpl_tm_mod, lua_v_doarithimpl_tm_mul, lua_v_doarithimpl_tm_pow,
      lua_v_doarithimpl_tm_sub, lua_v_doarithimpl_tm_unm,
    },
    lua_v_dolen::lua_v_dolen_export,
    lua_v_equalval::lua_v_equalval_export,
    lua_v_gettable::lua_v_gettable_export,
    lua_v_lessequal::lua_v_lessequal_export,
    lua_v_lessthan::lua_v_lessthan_export,
    lua_v_settable::lua_v_settable_export,
  },
  records::{closure::Closure, lua_table::LuaTable, proto::Proto, udata::Udata},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

use crate::{
  functions::{
    call_epilog_c::call_epilog_c_export, call_fallback::call_fallback_export,
    call_prolog::call_prolog_export, execute_dupclosure::execute_dupclosure_export,
    execute_forgprep::execute_forgprep_export, execute_getglobal::execute_getglobal_export,
    execute_gettableks::execute_gettableks_export,
    execute_getvarargs_const::execute_getvarargsconst,
    execute_getvarargs_mult_ret::execute_getvarargsmult_ret,
    execute_namecall::execute_namecall_export, execute_prepvarargs::execute_prepvarargs_export,
    execute_setglobal::execute_setglobal_export, execute_setlist::execute_setlist_export,
    execute_settableks::execute_settableks_export, forg_loop_node_iter::forg_loop_node_iter_export,
    forg_loop_non_table_fallback::forg_loop_non_table_fallback_export,
    forg_loop_table_iter::forg_loop_table_iter_export,
    forg_prep_xnext_fallback::forg_prep_xnext_fallback_export, get_import::get_import_export,
    new_userdata::new_userdata_export,
  },
  records::{c_math, native_context::NativeContext},
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

// —— 比较与算术（`luaV_*`）——

/// 两个 TValue 的三元比较（`luaV_lessthan` / `luaV_lessequal` / `luaV_equalval`）。
pub type NativeCompareFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, lhs: *const TValue, rhs: *const TValue) -> i32;

/// 双操作数算术（`luaV_doarith` 各运算一档），结果写入 `ra`。
pub type NativeArithFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, ra: StkId, rb: *const TValue, rc: *const TValue);

/// 取长（`luaV_dolen`），结果写入 `ra`。
pub type NativeDolenFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, ra: StkId, rb: *const TValue);

/// 表读写（`luaV_gettable` / `luaV_settable`），`val` 为结果/待写栈槽。
pub type NativeTableAccessFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, t: *const TValue, key: *mut TValue, val: StkId);

/// 字符串拼接（`luaV_concat`），`total`/`last` 为栈上区间端点。
pub type NativeConcatFn = unsafe extern "C-unwind" fn(l: *mut LuaState, total: i32, last: i32);

// —— 表原语（`luaH_*`）：表句柄按不透明 `*mut c_void` 传递 ——

/// `luaH_getn`：取表序列部分长度。
pub type NativeHGetnFn = unsafe extern "C-unwind" fn(t: *mut c_void) -> i32;

/// `lua_h_new`：新建表，返回不透明表句柄。
pub type NativeHNewFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, narray: i32, lnhash: i32) -> *mut c_void;

/// `lua_h_clone`：克隆表，返回新表句柄。
pub type NativeHCloneFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, tt: *mut c_void) -> *mut c_void;

/// `luaH_resizearray`：调整序列部分容量。
pub type NativeHResizearrayFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, t: *mut c_void, nasize: i32);

/// `lua_h_setnum`：按数值键取可写值槽。
pub type NativeHSetnumFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, t: *mut c_void, key: i32) -> *mut TValue;

// —— 增量 GC 屏障与步进（`luaC_*`）：对象句柄按不透明 `*mut c_void` 传递 ——

/// `lua_c_barriertable`：表写屏障；两参数分别是不透明表句柄与被写对象句柄。
pub type NativeCBarriertableFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, t: *mut c_void, v: *mut c_void);

/// `lua_c_barrierf`：非表对象写屏障；两参数都是不透明 GC 句柄。
pub type NativeCBarrierfFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, o: *mut c_void, v: *mut c_void);

/// `luaC_barrierback`：反向写屏障，`gclist` 为待挂链表的出参位。
pub type NativeCBarrierbackFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, o: *mut c_void, gclist: *mut *mut c_void);

/// `lua_c_step`：推进一轮 GC 工作，`assist` 表示是否由被分配方代跑。
pub type NativeCStepFn = unsafe extern "C-unwind" fn(l: *mut LuaState, assist: bool) -> usize;

// —— 闭包与 upvalue（`luaF_*`）——

/// `lua_f_close`：关闭 `level` 及以上的 open upvalue。
pub type NativeFCloseFn = unsafe extern "C-unwind" fn(l: *mut LuaState, level: StkId);

/// `luaF_findupval`：查找（必要时创建）`level` 处的 open upvalue。
pub type NativeFFindupvalFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, level: StkId) -> *mut c_void;

/// `lua_f_new_lclosure`：新建带 `nelems` 个 upvalue 槽的 Lua 闭包。
pub type NativeFNewLclosureFn = unsafe extern "C-unwind" fn(
  l: *mut LuaState,
  nelems: i32,
  e: *mut c_void,
  p: *mut c_void,
) -> *mut c_void;

// —— 元表与类型名（`luaT_*`）——

/// `luaT_gettm`：按元方法枚举取元表项，缺失时回退到 `ename`。
pub type NativeTGettmFn =
  unsafe extern "C-unwind" fn(events: *mut c_void, event: TMS, ename: *mut c_void) -> *const TValue;

/// `luaT_objtypenamestr`：取对象类型名的 interned 字符串句柄。
pub type NativeTObjtypenamestrFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, o: *const TValue) -> *const c_void;

// —— libc 数学（`records::c_math`，真 libc FFI）——

/// 一元 `double -> double`（`exp` / `log` / `sin` 等）。
pub type NativeMathUnaryFn = unsafe extern "C-unwind" fn(f64) -> f64;

/// 二元 `double, double -> double`（`pow` / `fmod` / `atan2`）。
pub type NativeMathBinaryFn = unsafe extern "C-unwind" fn(f64, f64) -> f64;

/// `ldexp(x, exp)`：整数指数缩放。
pub type NativeMathLdexpFn = unsafe extern "C-unwind" fn(f64, i32) -> f64;

/// `frexp`：出参 `*mut i32` 存指数，返回尾数——libc 出参约定，非 Rust 多返回值。
pub type NativeMathFrexpFn = unsafe extern "C-unwind" fn(f64, *mut i32) -> f64;

/// `modf`：出参 `*mut f64` 存整数位，返回小数位——同上，libc 出参约定。
pub type NativeMathModfFn = unsafe extern "C-unwind" fn(f64, *mut f64) -> f64;

// —— `for` 循环与调用序言（`forgLoop*` / `call*` / `newUdata` / `getImport`）——

/// `forgLoopTableIter` / `forgLoopNodeIter`：数组段/节点段迭代一步，成功时写 `ra`。
pub type NativeForgLoopIterFn = unsafe extern "C-unwind" fn(
  l: *mut LuaState,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool;

/// `forgLoopNonTableFallback`：非表迭代回退，返回跳转后的 pc 偏移。
pub type NativeForgLoopFallbackFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, insn_a: i32, aux: i32) -> i32;

/// `forgPrepXnextFallback`：`next` 半程调用的前置回退。
pub type NativeForgPrepXnextFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, ra: *mut TValue, pc: i32);

/// `callProlog`：解析被调对象并准备调用帧，返回待调 Lua 闭包。
pub type NativeCallPrologFn = unsafe extern "C-unwind" fn(
  l: *mut LuaState,
  ra: *mut TValue,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure;

/// `callEpilogC`：C 闭包返回后的收尾（结果搬移与栈顶修正）。
pub type NativeCallEpilogCFn = unsafe extern "C-unwind" fn(l: *mut LuaState, nresults: i32, n: i32);

/// `newUdata`：分配带 `tag` 的 userdata，返回其头部指针。
pub type NativeNewUserdataFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, s: usize, tag: i32) -> *mut Udata;

/// `getImport`：解析导入常量并写入 `res`。
pub type NativeGetImportFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, res: StkId, id: u32, pc: u32);

/// `callFallback`：无法内联的调用整体回退到解释器。
pub type NativeCallFallbackFn = unsafe extern "C-unwind" fn(
  l: *mut LuaState,
  ra: StkId,
  argtop: StkId,
  nresults: i32,
) -> *mut Closure;

// —— 指令级慢路径解释器（`execute*`）——

/// 通用「单条指令慢路径」形态：pc 游标 + 栈基址 + 常量表入参，返回下一条指令。
pub type NativeExecuteOpcodeFn = unsafe extern "C-unwind" fn(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction;

/// `executeGETVARARGSMULTRET`：多返回值形态的 vararg 取回。
pub type NativeExecuteGetvarargsMultRetFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, pc: *const Instruction, base: StkId, rai: i32);

/// `executeGETVARARGSCONST`：定长 vararg 取回。
pub type NativeExecuteGetvarargsConstFn =
  unsafe extern "C-unwind" fn(l: *mut LuaState, base: StkId, rai: i32, b: i32);

// —— gate 入口（`NativeContext::gate_entry` 槽位）——

/// JIT 机器码 gate 入口签名。与其余槽位不同，被调方不是 Rust 导出壳，而是
/// `build_entry_function` 按该 C ABI 生成、以 `entry_locations.start` 为入口的 gate 机器码；
/// 写入方 `functions::init_header_functions_*` 在分配确认非空后一次性把代码地址收口为
/// `Option<GateFn>`（null niche：非空地址即 `Some`），读取方 `functions::on_enter` 直接类型化调用。
pub type GateFn =
  unsafe extern "C-unwind" fn(*mut LuaState, *mut Proto, usize, *mut NativeContext) -> i32;

// 槽位 -> 实现绑定：类型别名即该槽位的 ABI 契约，赋值时编译器逐参数校验与导出壳一致。

pub const LUA_V_LESSTHAN: NativeCompareFn = lua_v_lessthan_export;
pub const LUA_V_LESSEQUAL: NativeCompareFn = lua_v_lessequal_export;
pub const LUA_V_EQUALVAL: NativeCompareFn = lua_v_equalval_export;

pub const LUA_V_DOARITH_ADD: NativeArithFn = lua_v_doarithimpl_tm_add;
pub const LUA_V_DOARITH_SUB: NativeArithFn = lua_v_doarithimpl_tm_sub;
pub const LUA_V_DOARITH_MUL: NativeArithFn = lua_v_doarithimpl_tm_mul;
pub const LUA_V_DOARITH_DIV: NativeArithFn = lua_v_doarithimpl_tm_div;
pub const LUA_V_DOARITH_IDIV: NativeArithFn = lua_v_doarithimpl_tm_idiv;
pub const LUA_V_DOARITH_MOD: NativeArithFn = lua_v_doarithimpl_tm_mod;
pub const LUA_V_DOARITH_POW: NativeArithFn = lua_v_doarithimpl_tm_pow;
pub const LUA_V_DOARITH_UNM: NativeArithFn = lua_v_doarithimpl_tm_unm;

pub const LUA_V_DOLEN: NativeDolenFn = lua_v_dolen_export;
pub const LUA_V_GETTABLE: NativeTableAccessFn = lua_v_gettable_export;
pub const LUA_V_SETTABLE: NativeTableAccessFn = lua_v_settable_export;
pub const LUA_V_CONCAT: NativeConcatFn = lua_v_concat_export;

pub const LUA_H_GETN: NativeHGetnFn = lua_h_getn_export;
pub const LUA_H_NEW: NativeHNewFn = lua_h_new_export;
pub const LUA_H_CLONE: NativeHCloneFn = lua_h_clone_export;
pub const LUA_H_RESIZEARRAY: NativeHResizearrayFn = lua_h_resizearray_export;
pub const LUA_H_SETNUM: NativeHSetnumFn = lua_h_setnum_export;

pub const LUA_C_BARRIERTABLE: NativeCBarriertableFn = lua_c_barriertable_export;
pub const LUA_C_BARRIERF: NativeCBarrierfFn = lua_c_barrierf_export;
pub const LUA_C_BARRIERBACK: NativeCBarrierbackFn = lua_c_barrierback_export;
pub const LUA_C_STEP: NativeCStepFn = lua_c_step_export;

pub const LUA_F_CLOSE: NativeFCloseFn = lua_f_close_export;
pub const LUA_F_FINDUPVAL: NativeFFindupvalFn = lua_f_findupval_export;
pub const LUA_F_NEW_LCLOSURE: NativeFNewLclosureFn = lua_f_new_lclosure_export;

pub const LUA_T_GETTM: NativeTGettmFn = lua_t_gettm_export;
pub const LUA_T_OBJTYPENAMESTR: NativeTObjtypenamestrFn = lua_t_objtypenamestr_export;

pub const LIBM_EXP: NativeMathUnaryFn = c_math::exp;
pub const LIBM_POW: NativeMathBinaryFn = c_math::pow;
pub const LIBM_FMOD: NativeMathBinaryFn = c_math::fmod;
pub const LIBM_LOG: NativeMathUnaryFn = c_math::log;
pub const LIBM_LOG2: NativeMathUnaryFn = c_math::log2;
pub const LIBM_LOG10: NativeMathUnaryFn = c_math::log10;
pub const LIBM_LDEXP: NativeMathLdexpFn = c_math::ldexp;
pub const LIBM_ROUND: NativeMathUnaryFn = c_math::round;
pub const LIBM_FREXP: NativeMathFrexpFn = c_math::frexp;
pub const LIBM_MODF: NativeMathModfFn = c_math::modf;

pub const LIBM_ASIN: NativeMathUnaryFn = c_math::asin;
pub const LIBM_SIN: NativeMathUnaryFn = c_math::sin;
pub const LIBM_SINH: NativeMathUnaryFn = c_math::sinh;
pub const LIBM_ACOS: NativeMathUnaryFn = c_math::acos;
pub const LIBM_COS: NativeMathUnaryFn = c_math::cos;
pub const LIBM_COSH: NativeMathUnaryFn = c_math::cosh;
pub const LIBM_ATAN: NativeMathUnaryFn = c_math::atan;
pub const LIBM_ATAN2: NativeMathBinaryFn = c_math::atan2;
pub const LIBM_TAN: NativeMathUnaryFn = c_math::tan;
pub const LIBM_TANH: NativeMathUnaryFn = c_math::tanh;

pub const FORG_LOOP_TABLE_ITER: NativeForgLoopIterFn = forg_loop_table_iter_export;
pub const FORG_LOOP_NODE_ITER: NativeForgLoopIterFn = forg_loop_node_iter_export;
pub const FORG_LOOP_NON_TABLE_FALLBACK: NativeForgLoopFallbackFn =
  forg_loop_non_table_fallback_export;
pub const FORG_PREP_XNEXT_FALLBACK: NativeForgPrepXnextFn = forg_prep_xnext_fallback_export;
pub const CALL_PROLOG: NativeCallPrologFn = call_prolog_export;
pub const CALL_EPILOG_C: NativeCallEpilogCFn = call_epilog_c_export;
pub const NEW_USERDATA: NativeNewUserdataFn = new_userdata_export;
pub const GET_IMPORT: NativeGetImportFn = get_import_export;
pub const CALL_FALLBACK: NativeCallFallbackFn = call_fallback_export;

pub const EXECUTE_GETGLOBAL: NativeExecuteOpcodeFn = execute_getglobal_export;
pub const EXECUTE_SETGLOBAL: NativeExecuteOpcodeFn = execute_setglobal_export;
pub const EXECUTE_GETTABLEKS: NativeExecuteOpcodeFn = execute_gettableks_export;
pub const EXECUTE_SETTABLEKS: NativeExecuteOpcodeFn = execute_settableks_export;
pub const EXECUTE_NAMECALL: NativeExecuteOpcodeFn = execute_namecall_export;
pub const EXECUTE_FORGPREP: NativeExecuteOpcodeFn = execute_forgprep_export;
pub const EXECUTE_GETVARARGSMULT_RET: NativeExecuteGetvarargsMultRetFn = execute_getvarargsmult_ret;
pub const EXECUTE_GETVARARGSCONST: NativeExecuteGetvarargsConstFn = execute_getvarargsconst;
pub const EXECUTE_DUPCLOSURE: NativeExecuteOpcodeFn = execute_dupclosure_export;
pub const EXECUTE_PREPVARARGS: NativeExecuteOpcodeFn = execute_prepvarargs_export;
pub const EXECUTE_SETLIST: NativeExecuteOpcodeFn = execute_setlist_export;
