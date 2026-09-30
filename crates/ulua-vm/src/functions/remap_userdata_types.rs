use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::functions::{fits, read_var_int::read_var_int};

/// 单字节类型重映射：带符号差转索引，命中重映射表才写回。索引为负（`t < BASE`）
/// 经 `try_into` 失败自然跳过，越界经 `get` 返回 `None` 也跳过——全程无 `unsafe`。
fn remap_byte(t: &mut u8, remap: &[u8]) {
  let index = *t as i32 - LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as i32;

  if let Ok(index) = usize::try_from(index)
    && let Some(&mapped) = remap.get(index)
  {
    *t = mapped;
  }
}

/// cpp `remapUserdataTypes`（lvmload.cpp:220-280）：typeinfo 三段布局的
/// userdata 类型重映射。
///
/// 与 cpp 的差异：cpp 的段长度全部来自不可信 varint，只靠末尾
/// `LUAU_ASSERT(offset == size)` 兜底（release 编译掉就是越界读写），Rust 侧
/// 每段先硬校验再取，任一环节不自洽即返回 `false`，由 `loadsafe` 转成
/// 「损坏字节码」错误。以 `&mut [u8]`/`&[u8]` 为入参，段内读写皆走切片，本身
/// 无 `unsafe`。
///
/// 返回 `false` 表示 typeinfo 里的 varint 跑到了缓冲外（截断），或三段布局
/// 没有刚好吃完 typeinfo（`offset != size`，cpp 对此只有一条 release 编译掉的
/// `LUAU_ASSERT`，拿着垃圾计数就会继续往下走）。
pub(crate) fn remap_userdata_types(data: &mut [u8], remap: &[u8]) -> bool {
  let size = data.len();
  let mut offset: usize = 0;

  // 三个头部计数：任何一个读不完都是截断
  let Some(type_size) = read_var_int(data, &mut offset) else {
    return false;
  };
  let Some(upval_count) = read_var_int(data, &mut offset) else {
    return false;
  };
  let Some(local_count) = read_var_int(data, &mut offset) else {
    return false;
  };

  if type_size != 0 {
    // types[2..type_size] 的读写都必须在缓冲内
    if !fits(offset, type_size as usize, size) {
      return false;
    }

    // Skip two bytes of function type introduction
    if type_size > 2 {
      for t in &mut data[offset + 2..offset + type_size as usize] {
        remap_byte(t, remap);
      }
    }

    offset += type_size as usize;
  }

  if upval_count != 0 {
    if !fits(offset, upval_count as usize, size) {
      return false;
    }

    for t in &mut data[offset..offset + upval_count as usize] {
      remap_byte(t, remap);
    }

    offset += upval_count as usize;
  }

  // 保留计数重复：locals 段每项变长（1 字节类型 + 1 字节寄存器 + 两个 varint），
  // offset 游标随解码推进、与循环次数不同步，无固定步长切片可迭代
  for _ in 0..local_count {
    // locals 项：1 字节类型 + 1 字节寄存器 + 两个 varint，逐项都要落在缓冲内
    if !fits(offset, 1, size) {
      return false;
    }

    remap_byte(&mut data[offset], remap);

    offset += 2;

    // 每个局部变量的两个 varint（类型 id / 名字 id）都必须落在缓冲内
    if read_var_int(data, &mut offset).is_none() || read_var_int(data, &mut offset).is_none() {
      return false;
    }
  }

  // 三段布局必须刚好吃完 typeinfo：cpp 用 LUAU_ASSERT 表达同一约束
  offset == size
}
