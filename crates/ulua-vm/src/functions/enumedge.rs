use core::ffi::c_char;

use crate::{
  functions::{
    enumtopointer::enumtopointer,
    fmt_cstr_buf::{cstr_display, fmt_cstr_buf},
  },
  macros::{gcvalue::gcvalue, getstr::getstr, iscollectable::iscollectable},
  records::{enum_context::EnumContext, gc_object::GCObject, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `ctx` 须指向存活的 `EnumContext`，其 `edge` 回调与 `context` 字段互相配套且符合 C 约定
/// （可空，空则跳过）；`from`/`to` 须为枚举期间存活、尚未回收的 GCObject，`edgename` 为有效 C 字符串。
/// 违反则调到悬垂函数指针或向回调传出已回收对象地址。cpp lgcdebug.cpp:765。
pub(crate) unsafe fn enumedge(
  ctx: *mut EnumContext,
  from: *mut GCObject,
  to: *mut GCObject,
  edgename: *const c_char,
) {
  // Safety: 契约保证 ctx/from/to 相互一致（from 为遍历中存活对象、to 为其引用字段目标），边记录仅追加输出不写对象
  unsafe {
    let ctx_ref = &*ctx;
    if let Some(edge_fn) = ctx_ref.edge {
      edge_fn(
        ctx_ref.context,
        enumtopointer(&mut *from),
        enumtopointer(&mut *to),
        edgename,
      );
    }
  }
}

/// 边名形态转换（§10 约定的唯一实现点）：NUL 结尾字节串常量 → `*const c_char`。
///
/// 供不走 [`enum_edge`] 门面的直调回调点（如 `enumproto` 的 native 边）复用，避免各处
/// 重复 `.as_ptr().cast()`；编译期求值，不产生运行期指令。
pub(crate) const fn edge_name(name: &[u8]) -> *const c_char {
  name.as_ptr().cast()
}

/// 边载荷/边名的裸指针形态转换收口（GC 枚举族共享门面）：`to` 为对象的任意字段指针，
/// `name` 为 NUL 结尾字节串常量（§10 约定：边名用 `&[u8]`，不引入 `CStr`/`c"…"`）。
/// 各 enum* 调用点因此不再重复 `as *mut GCObject` + `.as_ptr().cast()` 两步转换。
///
/// # Safety
/// 与 [`enumedge`] 同：`ctx` 指向存活 `EnumContext`（其 `edge` 回调与 `context` 配套，空则跳过），
/// `from`/`to` 须为枚举期间存活、尚未回收的对象，`name` 以 NUL 结尾且在本调用期间存活（常量串恒成立）。
#[inline]
pub(crate) unsafe fn enum_edge<T>(
  ctx: *mut EnumContext,
  from: *mut GCObject,
  to: *mut T,
  name: &[u8],
) {
  // Safety: 契约同 `enumedge`；此处只做同尺寸的指针形态转换与常量串的指针取出，原样转发
  unsafe {
    enumedge(ctx, from, to.cast(), edge_name(name));
  }
}

/// 成员边名缓冲长度：cpp `VM/src/lgcdebug.cpp:1046/1071` `char membername[32]`，
/// `snprintf(membername, sizeof(membername), …)` 的截断语义由 [`fmt_cstr_buf`] 同宽复刻。
const MEMBER_NAME_BUFSZ: usize = 32;

/// 成员区 → 命名边的枚举共享核心（enumclass 的 staticmembers 段与 enumobject 的 members 段
/// 同构骨架）：对每个 collectable 槽取名字表同基址项 `names[i]`，按 cpp 32 字节惯例格式化
/// 成员名后经 [`enumedge`] 登记。
///
/// # Safety
/// `vals`/`names` 须为被枚举对象成员区与名字表的存活切片视图，长度取自对象自身计数字段，
/// 且 `names.len() >= vals.len()`（同基址）；`ctx`/`obj` 满足 [`enumedge`] 契约；
/// `names[i]` 须为存活 TString。违反则成员名越界读/悬垂回调。
pub(crate) unsafe fn enum_member_edges(
  ctx: *mut EnumContext,
  obj: *mut GCObject,
  vals: &[TValue],
  names: &[*mut tstring],
) {
  // Safety: 契约保证切片即成员区/名字表视图、names 不短于 vals（zip 以 vals 长度为准，
  // 与旧 `names[i]` 索引读同界），块内只做只读枚举与回调转发
  unsafe {
    for (val, name_ts) in vals.iter().zip(names.iter().copied()) {
      if !iscollectable!(val) {
        continue;
      }

      let mut membername = [0 as c_char; MEMBER_NAME_BUFSZ];
      let name = cstr_display(getstr(name_ts));
      fmt_cstr_buf(&mut membername, format_args!("{name}"));

      enumedge(ctx, obj, gcvalue!(val), membername.as_ptr());
    }
  }
}
