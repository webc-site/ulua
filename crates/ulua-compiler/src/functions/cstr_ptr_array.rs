//! C ABI `const char* const*`（以 null 指针槽结尾的 C 字符串指针数组）的单点门面。
//!
//! 编译选项里有四处这种宿主登记的数组（`mutable_globals`、
//! `libraries_with_known_members`、`userdata_types`、`disabled_builtins`）。
//! 旧写法每处都手写「判空槽 + 双重解引用 `*ptr` + `ptr.add(1)` 递推」的循环，
//! 并把整段业务逻辑包在 `unsafe` 块里；门面把唯一的指针算术收口在
//! `cstr_ptr_array` 内，调用方按 `&[u8]`（NUL 前的原始内容，不含终止符）迭代、
//! 只写业务判定 —— `CStr` 类型不外泄，与 `ulua-ast` 名表的字节切片门面同族。

use core::{ffi::c_char, iter::from_fn, ptr::NonNull};

use ulua_common::functions::c_str::cstr_bytes;

/// 把宿主数组包装为只读字节串迭代器（NUL 前的内容，不含终止符），读到 null
/// 终止符为止；`array` 为 null 时得到空迭代器（对应 cpp 的 `if (options.x)` 早退），
/// 调用方因此无需自行判空。
///
/// 入参全部来自 `CompileOptions` 的 C ABI 字段（宿主按 `luau/compiler.h` 的
/// `const char* const*` 约定登记：null 结尾数组，非终止槽指向合法的 NUL 结尾
/// C 串，内容活过本次编译）。本门面是全 crate 对该形态唯一的指针算术收口点：
/// 判空槽、读槽、界内递推都在下方 `unsafe` 块内完成，业务调用方只见安全迭代器。
pub(crate) fn cstr_ptr_array<'a>(array: *const *const c_char) -> impl Iterator<Item = &'a [u8]> {
  // 槽地址以 `NonNull<*const _>` 记账（`NonNull::new` 只收 `*mut`，`cast_mut`
  // 不改变地址；后续只经 `as_ptr` 读取，从不写入该槽）。
  let mut slot = NonNull::new(array.cast_mut());

  from_fn(move || {
    let current = slot?;

    // Safety: 契约（见函数文档）保证 `current` 落在数组界内——首槽由 `NonNull`
    // 排除 null，其后只在读出非 null 时前进一格——这里取出的就是一个指针值。
    let entry = unsafe { *current.as_ptr() };
    if entry.is_null() {
      slot = None;
      return None;
    }

    // Safety: 契约保证当前槽非终止符，故 `+ 1` 至多停在终止符槽上，不越数组。
    slot = NonNull::new(unsafe { current.as_ptr().add(1) });

    // Safety: 契约保证非终止槽都指向 `'a` 内存活的合法 NUL 结尾 C 串；
    // `cstr_bytes` 是全仓唯一的 `CStr::from_ptr` 收口点，null 归一（此-entry 恒
    // 非 null 不会走到）与终止符截断由其契约承担。
    Some(unsafe { cstr_bytes::<'a>(entry) })
  })
}
