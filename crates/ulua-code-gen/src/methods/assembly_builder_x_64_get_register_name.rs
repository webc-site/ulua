use crate::{
  enums::size_x_64::SizeX64,
  records::{assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64},
};

impl AssemblyBuilderX64 {
  pub fn get_register_name(&self, reg: RegisterX64) -> &'static str {
    static NAMES: [[&str; 16]; 7] = [
      [
        "rip", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
      ],
      [
        "al", "cl", "dl", "bl", "spl", "bpl", "sil", "dil", "r8b", "r9b", "r10b", "r11b", "r12b",
        "r13b", "r14b", "r15b",
      ],
      [
        "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "r8w", "r9w", "r10w", "r11w", "r12w",
        "r13w", "r14w", "r15w",
      ],
      [
        "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "r8d", "r9d", "r10d", "r11d",
        "r12d", "r13d", "r14d", "r15d",
      ],
      [
        "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12",
        "r13", "r14", "r15",
      ],
      [
        "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10",
        "xmm11", "xmm12", "xmm13", "xmm14", "xmm15",
      ],
      [
        "ymm0", "ymm1", "ymm2", "ymm3", "ymm4", "ymm5", "ymm6", "ymm7", "ymm8", "ymm9", "ymm10",
        "ymm11", "ymm12", "ymm13", "ymm14", "ymm15",
      ],
    ];

    let size_index = match reg.size() {
      SizeX64::None => 0,
      SizeX64::Byte => 1,
      SizeX64::Word => 2,
      SizeX64::Dword => 3,
      SizeX64::Qword => 4,
      SizeX64::Xmmword => 5,
      SizeX64::Ymmword => 6,
    };
    let index = reg.index() as usize;

    NAMES[size_index][index]
  }
}
