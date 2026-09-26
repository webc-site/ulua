use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_common::functions::c_slice::c_slice;

use crate::{records::builtin_types::BuiltinTypes, type_aliases::type_id::TypeId};

/// 数字类转换符集（cpp `BuiltinDefinitions.cpp:627` 的 `cdiouxXeEfgGqs*`）。
const K_OPTIONS: &[u8] = b"cdiouxXeEfgGqs*";

/// `K_OPTIONS` 的编译期位掩码（集合内字节均 < 0x80，u128 即全覆盖）。
///
/// 实测代码生成：15 项常量切片的 `contains` 不会被 LLVM 折叠为位测试，
/// 仍生成逐字节内存循环；位掩码把成员判定降为单次 128 位移位与。
/// 形态参照 ulua-vm `nospecials` 的 `SPECIAL_MASK` 先例。
const K_OPTIONS_MASK: u128 = build_options_mask();

const fn build_options_mask() -> u128 {
  let mut mask = 0u128;
  let mut i = 0;
  while i < K_OPTIONS.len() {
    mask |= 1u128 << K_OPTIONS[i];
    i += 1;
  }
  mask
}

/// 成员判定：`>= 0x80` 恒非成员（集合内均为 ASCII），短路后移位量必 < 128。
#[inline]
const fn is_number_option(b: u8) -> bool {
  b < 0x80 && (K_OPTIONS_MASK >> b) & 1 != 0
}

// 定表自检：掩码恰由 K_OPTIONS 的 15 个互异字节构成（popcount 相等 ⇔ 无缺项、
// 无重复、无杂位），漂表或漂掩码即编译失败。
const _: () = assert!(K_OPTIONS_MASK.count_ones() == K_OPTIONS.len() as u32);

/// 逐字节扫描格式串，等价 C++ `parseFormatString`（BuiltinDefinitions.cpp:627）。
pub fn parse_format_string_bytes(
  builtin_types: NonNull<BuiltinTypes>,
  data_slice: &[u8],
) -> Vec<TypeId> {
  // Safety: `builtin_types` 是 `NonNull` 句柄（非空不变量由类型保证）——各调用方
  // （magic_format 的新/旧求解器分支）都从不空的会话级 `BuiltinTypes` 表借用构造
  // 它，其寿命覆盖整次格式解析；`as_ref` 仅授予只读借用读 TypeId 字段。
  let builtin_types = unsafe { builtin_types.as_ref() };

  let mut result = Vec::new();
  let mut bytes = data_slice.iter().copied().peekable();
  while let Some(b) = bytes.next() {
    if b != b'%' {
      continue;
    }

    // "%%" → 字面百分号，整对跳过
    if bytes.next_if_eq(&b'%').is_some() {
      continue;
    }

    // 忽略标志/精度等字符，直到首个字母或 '*'。
    // C++ `data[i] > 0` 排除的高位字节已被 is_ascii_alphabetic / b'*' 蕴含。
    let Some(c) = bytes
      .by_ref()
      .find(|&b| b.is_ascii_alphabetic() || b == b'*')
    else {
      break;
    };

    result.push(if matches!(c, b'q' | b's') {
      builtin_types.string_type
    } else if c == b'*' {
      builtin_types.unknown_type
    } else if is_number_option(c) {
      builtin_types.number_type
    } else {
      builtin_types.error_recovery_type(builtin_types.any_type)
    });
  }

  result
}

/// # Safety
/// `data/size` 须构成合法字节区域（批 2 后 `*const u8`：cpp `const char*`+
/// `size_t` 同字节域，判定点不变）。
pub unsafe fn parse_format_string(
  builtin_types: NonNull<BuiltinTypes>,
  data: *const u8,
  size: usize,
) -> Vec<TypeId> {
  // Safety: `data/size` 是调用方从 RTTI 判型命中的存活 `AstExprConstantString`
  // 节点读出的 arena 成对写入「指针+计数」（C++ 字符串字面量惯用法），满足
  // `c_slice` 的「`p` 对 `len` 个元素有效可读」契约；u8 无对齐要求，null 或
  // 0 长度由 `c_slice` 内部守卫收敛为空切片。
  let data_slice = unsafe { c_slice::<u8>(data, size) };
  parse_format_string_bytes(builtin_types, data_slice)
}
