//! Node: `cxx:Record:Luau.VM:VM/src/lmem.cpp:127:size_class_config`
//! Source: `VM/src/lmem.cpp:127-176` (hand-fixed: the constexpr constructor
//! was ported as an instance method nobody called, the `kSizeClassConfig`
//! static referenced by `sizeclass!` never existed, and kSizeClasses said 32
//! where the C++ says LUA_SIZECLASSES = 40 — the table is now built at
//! compile time by a const fn, faithful to the C++ constructor)

use core::ffi::c_int;
pub const K_SIZE_CLASSES: usize = 40; // LUA_SIZECLASSES
pub const K_MAX_SMALL_SIZE: usize = 1024;

/// 内存分配大小类别配置（内存分级方案）
///
/// 对标 C++ `SizeClassConfig`：
/// `class_for_size` 显式采用 `i8` 存储类别索引及 `-1` 哨兵值。
/// 跨平台说明：在 ARM64 Linux 上 `c_char` 为无符号 `u8`，若使用 `c_char` 会导致 `-1` 溢出为 `255`
/// 从而破坏 `< 0` 越界检查语义，此处强制统一为 `i8` 保证跨平台一致性。
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SizeClassConfig {
  pub size_of_class: [c_int; K_SIZE_CLASSES],
  pub class_for_size: [i8; K_MAX_SMALL_SIZE + 1],
  pub class_count: c_int,
}

impl SizeClassConfig {
  /// 根据分配字节数查询其对应的大小类别索引（O(1) 查表）。
  ///
  /// - 若 `sz` 在 `1..=K_MAX_SMALL_SIZE` 范围内，返回对应的类别索引（>= 0）；
  /// - 若 `sz == 0` 或 `sz > K_MAX_SMALL_SIZE`，返回 `-1` 哨兵。
  #[inline(always)]
  pub const fn get_size_class(&self, sz: usize) -> i8 {
    let idx = sz.wrapping_sub(1);
    if idx < K_MAX_SMALL_SIZE {
      self.class_for_size[sz]
    } else {
      -1
    }
  }
}

const fn build_size_class_config() -> SizeClassConfig {
  let mut size_of_class = [0i32; K_SIZE_CLASSES];
  let mut class_for_size = [-1i8; K_MAX_SMALL_SIZE + 1];
  let mut class_count = 0usize;

  // we use a progressive size class scheme:
  // - all size classes are aligned by 8b to satisfy pointer alignment requirements
  // - we first allocate sizes classes in multiples of 8
  // - after the first cutoff we allocate size classes in multiples of 16
  // - after the second cutoff we allocate size classes in multiples of 32
  // - after the third cutoff we allocate size classes in multiples of 64
  // this balances internal fragmentation vs external fragmentation
  let mut size = 8;
  while size < 64 {
    size_of_class[class_count] = size;
    class_count += 1;
    size += 8;
  }
  let mut size = 64;
  while size < 256 {
    size_of_class[class_count] = size;
    class_count += 1;
    size += 16;
  }
  let mut size = 256;
  while size < 512 {
    size_of_class[class_count] = size;
    class_count += 1;
    size += 32;
  }
  let mut size = 512;
  while size <= 1024 {
    size_of_class[class_count] = size;
    class_count += 1;
    size += 64;
  }

  assert!(class_count <= K_SIZE_CLASSES);

  // fill the lookup table for all classes
  let mut klass = 0usize;
  while klass < class_count {
    class_for_size[size_of_class[klass] as usize] = klass as i8;
    klass += 1;
  }

  // fill the gaps in lookup table
  let mut size = K_MAX_SMALL_SIZE as i32 - 1;
  while size >= 0 {
    if class_for_size[size as usize] < 0 {
      class_for_size[size as usize] = class_for_size[size as usize + 1];
    }
    size -= 1;
  }

  SizeClassConfig {
    size_of_class,
    class_for_size,
    class_count: class_count as c_int,
  }
}

pub static K_SIZE_CLASS_CONFIG: SizeClassConfig = build_size_class_config();
