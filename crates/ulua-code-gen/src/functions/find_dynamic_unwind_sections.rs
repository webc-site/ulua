use crate::records::unw_dynamic_unwind_sections_t::unw_dynamic_unwind_sections_t;

pub fn find_dynamic_unwind_sections(_addr: usize, info: &mut unw_dynamic_unwind_sections_t) -> i32 {
  // 为 JIT 出的代码定义一个最小 mach header。
  // 原 C++ 使用 64 位 Mach-O header；此 Rust 翻译只复刻写入 unwind 回调的值。
  // 常量与 C++ 对齐：MH_MAGIC_64、CPU_TYPE_ARM64、CPU_SUBTYPE_ARM64_ALL、MH_DYLIB。
  //
  // 注意：这是架构相关的 native 代码；wasm 构建不应依赖它。

  #[repr(C)]
  struct mach_header_64 {
    magic: u32,
    cputype: i32,
    cpusubtype: i32,
    filetype: u32,
  }

  const MH_MAGIC_64: u32 = 0xfeedfacf;
  const CPU_TYPE_ARM64: i32 = 0x0100000c;
  const CPU_SUBTYPE_ARM64_ALL: i32 = 0x00000000;
  const MH_DYLIB: u32 = 0x6;

  static K_FAKE_MACH_HEADER: mach_header_64 = mach_header_64 {
    magic: MH_MAGIC_64,
    cputype: CPU_TYPE_ARM64,
    cpusubtype: CPU_SUBTYPE_ARM64_ALL,
    filetype: MH_DYLIB,
  };

  info.dso_base = &K_FAKE_MACH_HEADER as *const _ as usize;
  info.dwarf_section = 0;
  info.dwarf_section_length = 0;
  info.compact_unwind_section = 0;
  info.compact_unwind_section_length = 0;

  1
}
