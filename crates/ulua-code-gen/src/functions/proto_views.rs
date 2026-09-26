//! `Proto` 只读视图收口（review.md §2「`*const T + len` → `&[T]`、`(*p).field` → 具名字段」）。
//!
//! ulua-vm 的 [`Proto`] 以「裸基址 + `size*` 计数」描述字节码、常量表、局部变量表等子数组
//! （§11 的 arena/`Vec` 化属 VM 阶段，本模块不动其布局）。code-gen 的读侧（IR 构建、字节码
//! 类型分析、常量折叠、IR dump/日志）只需要只读视图，故把 (基址, 长度) → 切片的换算收敛在
//! 本模块：`unsafe` 只出现在下方 `span`/`string_constant*`/`name_str` 几个收口里，业务侧一律
//! 走安全切片（带边界检查、可 `iter()`/`enumerate()`）。
//!
//! 生命周期：视图切片的寿命锚定在入参 `&'p Proto` 上——Proto 由 VM 在整段编译会话内持有
//! （cpp 侧同一 `Proto&` 契约），子数组与串常量随 Proto 一起由 `luaF_freeproto`/GC 回收。
//!
//! 空基址约定：VM 在 `size* == 0` 时允许子数组基址为 null，cpp 的 `p[i]` 空循环不解引用；
//! 本模块把「空基址」或「负长度」统一折叠为 `&[]`（debug 构建的 `-Zub-checks` 禁止以 null 为
//! 起点构造切片），与各改造点原先的 `is_null()` 短路语义一致。
//!
//! [`Proto`]: ulua_vm::records::proto::Proto

use core::{ffi::c_int, ptr::from_ref, slice::from_raw_parts, str::from_utf8};

use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{
  macros::getstr::getstr,
  records::{loc_var::LocVar, proto::Proto, t_string::tstring},
  type_aliases::{instruction::Instruction, t_value::TValue},
};

use crate::functions::ts_string_layout::ts_hash;

/// `*const T` + 计数 → 只读切片：空基址或负长度折叠为 `&[]`。
///
/// # Safety
/// `ptr` 非空时必须指向 `len` 个类型为 `T`、对齐一致、连续存放且在返回值寿命内只读存活的
/// 元素（VM 子数组布局契约）。
unsafe fn span<'p, T>(ptr: *const T, len: c_int) -> &'p [T] {
  if ptr.is_null() || len <= 0 {
    return &[];
  }
  // Safety: 前置条件由下方各视图函数的契约说明交给调用方保证。
  unsafe { from_raw_parts(ptr, len as usize) }
}

/// 常量表 `k[0..sizek]`。
///
/// 契约：`proto` 指向存活 Proto；`k`/`sizek` 由字节码加载接线一致。
pub(crate) fn constants(proto: &Proto) -> &[TValue] {
  // Safety: 依模块文档——`proto.k` 为随存活 Proto 分配的 TValue 数组基址，`sizek` 是元素数。
  unsafe { span(proto.k.cast_const(), proto.sizek) }
}
/// 字节码字流 `code[0..sizecode]`。
///
/// 契约：`proto` 指向存活 Proto；`code`/`sizecode` 由字节码加载接线一致。
pub(crate) fn code(proto: &Proto) -> &[Instruction] {
  // Safety: 同上——`proto.code` 为 `sizecode` 个 `Instruction`（u32）连续数组基址。
  unsafe { span(proto.code.cast_const(), proto.sizecode) }
}

/// 子原型表 `p[0..sizep]`（元素仍是跨 crate 的 C-ABI `*mut Proto` 句柄）。
///
/// 契约：`proto` 指向存活 Proto；`p`/`sizep` 由字节码加载接线一致。
pub(crate) fn child_protos(proto: &Proto) -> &[*mut Proto] {
  // Safety: 同上——`proto.p` 为 `sizep` 个子原型指针连续数组基址，元素指向存活 Proto。
  unsafe { span(proto.p.cast_const(), proto.sizep) }
}

/// 局部变量表 `locvars[0..sizelocvars]`。
///
/// 契约：`proto` 指向存活 Proto；`locvars`/`sizelocvars` 由字节码加载接线一致。
pub(crate) fn locvars(proto: &Proto) -> &[LocVar] {
  // Safety: 同上——`proto.locvars` 为 `sizelocvars` 个 `LocVar` 连续数组基址。
  unsafe { span(proto.locvars.cast_const(), proto.sizelocvars) }
}

/// 上值名表 `upvalues[0..sizeupvalues]`（元素为空或指向存活 TString）。
///
/// 契约：`proto` 指向存活 Proto；`upvalues`/`sizeupvalues` 由字节码加载接线一致。
pub(crate) fn upvalue_names(proto: &Proto) -> &[*mut tstring] {
  // Safety: 同上——`proto.upvalues` 为 `sizeupvalues` 个 `tstring*` 连续数组基址。
  unsafe { span(proto.upvalues.cast_const(), proto.sizeupvalues) }
}

/// 字节码类型信息缓冲区 `typeinfo[0..sizetypeinfo]`。
///
/// 契约：`proto` 指向存活 Proto；`typeinfo`/`sizetypeinfo` 由字节码加载接线一致。
pub(crate) fn typeinfo(proto: &Proto) -> &[u8] {
  // Safety: 同上——`proto.typeinfo` 为 `sizetypeinfo` 字节连续缓冲区基址（u8 对齐平凡）。
  unsafe { span(proto.typeinfo.cast_const(), proto.sizetypeinfo) }
}

/// 常量表第 `index` 项的字符串载荷（cpp `getstr(tsvalue(&proto->k[aux]))` + `ts->len`）。
///
/// 返回长度是串本体字节数，不含 VM 追加的 NUL 终止符；终止符仍在 `bytes.len()` 处可读，
/// 故 `bytes.as_ptr()` 可直接交给 `*const c_char` 契约的宿主 hook。
///
/// 契约：`proto` 指向存活 Proto，`index` 为该项是 GC 字符串常量的合法下标（字节码验证器
/// 保证）。越界按空串处理——cpp 此处是越界读，取安全方向。
pub(crate) fn string_constant(proto: &Proto, index: usize) -> &[u8] {
  let Some(ts) = string_constant_ts(proto, index) else {
    return &[];
  };
  // Safety: `ts` 由 `string_constant_ts` 证明为存活 TString，`(*ts).len` 为同址 `pub` 字段；
  // `getstr` 契约给出可读 `len + 1` 字节的柔性数组首地址。寿命经 `proto` 锚定为 `'p`。
  unsafe { from_raw_parts(getstr(ts).cast::<u8>(), (*ts).len as usize) }
}

/// 常量表第 `index` 项的字符串哈希（cpp `tsvalue(&proto->k[aux])->hash`）。
///
/// `hash` 在 ulua-vm 侧是 `pub(crate)`，故经 [`ts_hash`] 的 `#[repr(C)]` 布局镜像读取。
///
/// 契约：同 [`string_constant`]。越界按 `0` 处理（cpp 为越界读）。
pub(crate) fn string_constant_hash(proto: &Proto, index: usize) -> u32 {
  let Some(ts) = string_constant_ts(proto, index) else {
    return 0;
  };
  // Safety: 同上——`ts` 指向存活 TString，`ts_hash` 按其布局镜像只读 hash 字段。
  unsafe { ts_hash(ts) }
}

/// 常量表第 `index` 项的 TString 地址；项越界时 `None`。
///
/// 契约：`index` 指向的常量项为 GC 字符串常量（`as_string` 内置 tt 断言复核）。
fn string_constant_ts(proto: &Proto, index: usize) -> Option<*const tstring> {
  let constant = constants(proto).get(index)?;
  // Safety: 依本函数契约——`value.gc` 命中联合体存活变体（tt 为 String，as_string 断言复核），
  // `(*gc).ts` 是该存活 GCObject 的 tstring 变体域。
  let ts: &tstring = unsafe { (*constant.value.gc).as_string().unwrap() };
  Some(from_ref(ts))
}

/// TString 名字的原字节（NUL 前），空指针为 `None`（cpp 把 `const char*` 直接交给
/// `std::string` 的字段，如 `Proto.debugname`/`Proto.source`）。
///
/// 契约：`ts` 为空或指向存活 TString；`_anchor` 只用于把返回寿命锚定到宿主 Proto 的存活期
/// （名字串随宿主 Proto 可达，故不早于 `'p` 失效），不参与运行时逻辑。
pub(crate) fn name_bytes(ts: *const tstring, _anchor: &Proto) -> Option<&[u8]> {
  if ts.is_null() {
    return None;
  }
  // Safety: `ts` 已判非空且指向存活 TString——`getstr` 给出可读的 NUL 结尾缓冲区首地址，
  // `cstr_bytes` 取 NUL 前借用，存活期由 `anchor` 覆盖。
  Some(unsafe { cstr_bytes(getstr(ts)) })
}

/// TString 名字的 UTF-8 视图（按 NUL 结尾读取，与 cpp 把 `const char*` 交给 `std::string`
/// 的行为一致）；空指针为 `None`，非法 UTF-8 降级为空串（仅用于 dump/日志展示路径）。
///
/// 需要保留原字节（如统计名要 `from_utf8_lossy`）的调用点用 [`name_bytes`]。
///
/// 契约：同 [`name_bytes`]。
pub(crate) fn name_str(ts: *const tstring, anchor: &Proto) -> Option<&str> {
  name_bytes(ts, anchor).map(|bytes| from_utf8(bytes).unwrap_or(""))
}
