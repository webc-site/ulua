use core::ffi::c_void;

use ulua_code_gen::enums::alignment_data_x_64::AlignmentDataX64 as UluaCodeGenAlignmentDataX64;
extern crate alloc;

mod assembly_builder_x_64_alignment_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_alignment_forms() {
    use ulua_code_gen::{
      enums::alignment_data_x_64::AlignmentDataX64,
      records::assembly_builder_x_64::AssemblyBuilderX64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(
      |b| {
        b.ret();
        b.align(8, AlignmentDataX64::Nop);
      },
      &[0xc3, 0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00],
    );

    check(
      |b| {
        b.ret();
        b.align(32, AlignmentDataX64::Nop);
      },
      &[
        0xc3, 0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x1f, 0x84, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x1f,
        0x40, 0x00,
      ],
    );

    check(
      |b| {
        b.ret();
        b.align(8, AlignmentDataX64::Int3);
      },
      &[0xc3, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc],
    );

    check(
      |b| {
        b.ret();
        b.align(8, AlignmentDataX64::Ud2);
      },
      &[0xc3, 0x0f, 0x0b, 0x0f, 0x0b, 0x0f, 0x0b, 0xcc],
    );
  }
}

mod assembly_builder_x_64_alignment_overflow {
  use ulua_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;

  use super::*;
  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_alignment_overflow() {
    // Test that alignment correctly resizes the code Buffer
    {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      build.ret();
      build.align(8192, UluaCodeGenAlignmentDataX64::Nop);
      build.finalize();
    }

    {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      build.ret();
      build.align(8192, UluaCodeGenAlignmentDataX64::Int3);
      build.finalize();
    }

    {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      for _ in 0..8192 {
        build.int3();
      }
      build.finalize();
    }

    {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      build.ret();
      build.align(8192, UluaCodeGenAlignmentDataX64::Ud2);
      build.finalize();
    }
  }
}

mod assembly_builder_x_64_avx_binary_instruction_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_avx_binary_instruction_forms() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{DWORD, OperandX64, QWORD, XMMWORD, YMMWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| b.vaddpd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x29, 0x58, 0xc6],
    );
    check(
      |b| b.vaddpd(R::XMM8.into(), R::XMM10.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x29, 0x58, 0x01],
    );
    check(
      |b| b.vaddpd(R::YMM8.into(), R::YMM10.into(), R::YMM14.into()),
      &[0xc4, 0x41, 0x2d, 0x58, 0xc6],
    );
    check(
      |b| b.vaddpd(R::YMM8.into(), R::YMM10.into(), idx(YMMWORD, R::R9)),
      &[0xc4, 0x41, 0x2d, 0x58, 0x01],
    );
    check(
      |b| b.vaddps(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x28, 0x58, 0xc6],
    );
    check(
      |b| b.vaddps(R::XMM8.into(), R::XMM10.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x28, 0x58, 0x01],
    );
    check(
      |b| b.vaddsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0x58, 0xc6],
    );
    check(
      |b| b.vaddsd(R::XMM8.into(), R::XMM10.into(), idx(QWORD, R::R9)),
      &[0xc4, 0x41, 0x2b, 0x58, 0x01],
    );
    check(
      |b| b.vaddss(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2a, 0x58, 0xc6],
    );
    check(
      |b| b.vaddss(R::XMM8.into(), R::XMM10.into(), idx(DWORD, R::R9)),
      &[0xc4, 0x41, 0x2a, 0x58, 0x01],
    );

    check(
      |b| b.vaddps(R::XMM1.into(), R::XMM2.into(), R::XMM3.into()),
      &[0xc4, 0xe1, 0x68, 0x58, 0xcb],
    );
    check(
      |b| {
        b.vaddps(
          R::XMM9.into(),
          R::XMM12.into(),
          idx(XMMWORD, R::R9 + R::R14 * 2 + 0x1c),
        )
      },
      &[0xc4, 0x01, 0x18, 0x58, 0x4c, 0x71, 0x1c],
    );
    check(
      |b| b.vaddps(R::YMM1.into(), R::YMM2.into(), R::YMM3.into()),
      &[0xc4, 0xe1, 0x6c, 0x58, 0xcb],
    );
    check(
      |b| {
        b.vaddps(
          R::YMM9.into(),
          R::YMM12.into(),
          idx(YMMWORD, R::R9 + R::R14 * 2 + 0x1c),
        )
      },
      &[0xc4, 0x01, 0x1c, 0x58, 0x4c, 0x71, 0x1c],
    );

    // Coverage for other instructions that follow the same pattern
    check(
      |b| b.vsubsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0x5c, 0xc6],
    );
    check(
      |b| b.vmulsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0x59, 0xc6],
    );
    check(
      |b| b.vdivsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0x5e, 0xc6],
    );

    check(
      |b| b.vsubps(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x28, 0x5c, 0xc6],
    );
    check(
      |b| b.vmulps(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x28, 0x59, 0xc6],
    );
    check(
      |b| b.vdivps(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x28, 0x5e, 0xc6],
    );

    check(
      |b| b.vorpd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x29, 0x56, 0xc6],
    );
    check(
      |b| b.vxorpd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x29, 0x57, 0xc6],
    );
    check(
      |b| b.vorps(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x28, 0x56, 0xc6],
    );

    check(
      |b| b.vandpd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x29, 0x54, 0xc6],
    );
    check(
      |b| b.vandnpd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x29, 0x55, 0xc6],
    );

    check(
      |b| b.vmaxsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0x5f, 0xc6],
    );
    check(
      |b| b.vminsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0x5d, 0xc6],
    );

    check(
      |b| b.vmaxss(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2a, 0x5f, 0xc6],
    );
    check(
      |b| b.vminss(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2a, 0x5d, 0xc6],
    );

    check(
      |b| b.vmaxps(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x28, 0x5f, 0xc6],
    );
    check(
      |b| b.vminps(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x28, 0x5d, 0xc6],
    );

    check(
      |b| b.vcmpeqsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0xc2, 0xc6, 0x00],
    );
    check(
      |b| b.vcmpltsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0xc2, 0xc6, 0x01],
    );
  }
}

mod assembly_builder_x_64_avx_conversion_instruction_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_avx_conversion_instruction_forms() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{DWORD, OperandX64, QWORD, XMMWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| b.vcvttsd2si(R::ECX.into(), R::XMM0.into()),
      &[0xc4, 0xe1, 0x7b, 0x2c, 0xc8],
    );
    check(
      |b| b.vcvttsd2si(R::R9D.into(), idx(XMMWORD, R::RCX + R::RDX)),
      &[0xc4, 0x61, 0x7b, 0x2c, 0x0c, 0x11],
    );
    check(
      |b| b.vcvttsd2si(R::RDX.into(), R::XMM0.into()),
      &[0xc4, 0xe1, 0xfb, 0x2c, 0xd0],
    );
    check(
      |b| b.vcvttsd2si(R::R13.into(), idx(XMMWORD, R::RCX + R::RDX)),
      &[0xc4, 0x61, 0xfb, 0x2c, 0x2c, 0x11],
    );
    check(
      |b| b.vcvtsi2sd(R::XMM5.into(), R::XMM10.into(), R::ECX.into()),
      &[0xc4, 0xe1, 0x2b, 0x2a, 0xe9],
    );
    check(
      |b| b.vcvtsi2sd(R::XMM6.into(), R::XMM11.into(), idx(DWORD, R::RCX + R::RDX)),
      &[0xc4, 0xe1, 0x23, 0x2a, 0x34, 0x11],
    );
    check(
      |b| b.vcvtsi2sd(R::XMM5.into(), R::XMM10.into(), R::R13.into()),
      &[0xc4, 0xc1, 0xab, 0x2a, 0xed],
    );
    check(
      |b| b.vcvtsi2sd(R::XMM6.into(), R::XMM11.into(), idx(QWORD, R::RCX + R::RDX)),
      &[0xc4, 0xe1, 0xa3, 0x2a, 0x34, 0x11],
    );
    check(
      |b| b.vcvtsd2ss(R::XMM5.into(), R::XMM10.into(), R::XMM11.into()),
      &[0xc4, 0xc1, 0x2b, 0x5a, 0xeb],
    );
    check(
      |b| b.vcvtsd2ss(R::XMM6.into(), R::XMM11.into(), idx(QWORD, R::RCX + R::RDX)),
      &[0xc4, 0xe1, 0xa3, 0x5a, 0x34, 0x11],
    );
    check(
      |b| b.vcvtss2sd(R::XMM3.into(), R::XMM8.into(), R::XMM12.into()),
      &[0xc4, 0xc1, 0x3a, 0x5a, 0xdc],
    );
    check(
      |b| b.vcvtss2sd(R::XMM4.into(), R::XMM9.into(), idx(DWORD, R::RCX + R::RSI)),
      &[0xc4, 0xe1, 0x32, 0x5a, 0x24, 0x31],
    );
  }
}

mod assembly_builder_x_64_avx_move_instruction_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_avx_move_instruction_forms() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{DWORD, OperandX64, QWORD, XMMWORD, YMMWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| b.vmovsd_operand_x_64_operand_x_64(idx(QWORD, R::R9), R::XMM10.into()),
      &[0xc4, 0x41, 0x7b, 0x11, 0x11],
    );
    check(
      |b| b.vmovsd_operand_x_64_operand_x_64(R::XMM8.into(), idx(QWORD, R::R9)),
      &[0xc4, 0x41, 0x7b, 0x10, 0x01],
    );
    check(
      |b| {
        b.vmovsd_operand_x_64_operand_x_64_operand_x_64(
          R::XMM8.into(),
          R::XMM10.into(),
          R::XMM14.into(),
        )
      },
      &[0xc4, 0x41, 0x2b, 0x10, 0xc6],
    );
    check(
      |b| b.vmovss_operand_x_64_operand_x_64(idx(DWORD, R::R9), R::XMM10.into()),
      &[0xc4, 0x41, 0x7a, 0x11, 0x11],
    );
    check(
      |b| b.vmovss_operand_x_64_operand_x_64(R::XMM8.into(), idx(DWORD, R::R9)),
      &[0xc4, 0x41, 0x7a, 0x10, 0x01],
    );
    check(
      |b| {
        b.vmovss_operand_x_64_operand_x_64_operand_x_64(
          R::XMM8.into(),
          R::XMM10.into(),
          R::XMM14.into(),
        )
      },
      &[0xc4, 0x41, 0x2a, 0x10, 0xc6],
    );
    check(
      |b| b.vmovapd(R::XMM8.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x79, 0x28, 0x01],
    );
    check(
      |b| b.vmovapd(idx(XMMWORD, R::R9), R::XMM10.into()),
      &[0xc4, 0x41, 0x79, 0x29, 0x11],
    );
    check(
      |b| b.vmovapd(R::YMM8.into(), idx(YMMWORD, R::R9)),
      &[0xc4, 0x41, 0x7d, 0x28, 0x01],
    );
    check(
      |b| b.vmovaps(R::XMM8.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x78, 0x28, 0x01],
    );
    check(
      |b| b.vmovaps(idx(XMMWORD, R::R9), R::XMM10.into()),
      &[0xc4, 0x41, 0x78, 0x29, 0x11],
    );
    check(
      |b| b.vmovaps(R::YMM8.into(), idx(YMMWORD, R::R9)),
      &[0xc4, 0x41, 0x7c, 0x28, 0x01],
    );
    check(
      |b| b.vmovupd(R::XMM8.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x79, 0x10, 0x01],
    );
    check(
      |b| b.vmovupd(idx(XMMWORD, R::R9), R::XMM10.into()),
      &[0xc4, 0x41, 0x79, 0x11, 0x11],
    );
    check(
      |b| b.vmovupd(R::YMM8.into(), idx(YMMWORD, R::R9)),
      &[0xc4, 0x41, 0x7d, 0x10, 0x01],
    );
    check(
      |b| b.vmovups(R::XMM8.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x78, 0x10, 0x01],
    );
    check(
      |b| b.vmovups(idx(XMMWORD, R::R9), R::XMM10.into()),
      &[0xc4, 0x41, 0x78, 0x11, 0x11],
    );
    check(
      |b| b.vmovups(R::YMM8.into(), idx(YMMWORD, R::R9)),
      &[0xc4, 0x41, 0x7c, 0x10, 0x01],
    );
    check(
      |b| b.vmovq(R::XMM1.into(), R::RBX.into()),
      &[0xc4, 0xe1, 0xf9, 0x6e, 0xcb],
    );
    check(
      |b| b.vmovq(R::RBX.into(), R::XMM1.into()),
      &[0xc4, 0xe1, 0xf9, 0x7e, 0xcb],
    );
    check(
      |b| b.vmovq(R::XMM1.into(), idx(QWORD, R::R9)),
      &[0xc4, 0xc1, 0xf9, 0x6e, 0x09],
    );
    check(
      |b| b.vmovq(idx(QWORD, R::R9), R::XMM1.into()),
      &[0xc4, 0xc1, 0xf9, 0x7e, 0x09],
    );
  }
}

mod assembly_builder_x_64_avx_ternary_instruction_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_avx_ternary_instruction_forms() {
    use ulua_code_gen::{
      enums::rounding_mode_x_64::RoundingModeX64,
      records::{
        assembly_builder_x_64::AssemblyBuilderX64,
        operand_x_64::{OperandX64, XMMWORD},
        register_x_64::RegisterX64 as R,
      },
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| {
        b.vroundsd(
          R::XMM7.into(),
          R::XMM12.into(),
          R::XMM3.into(),
          RoundingModeX64::RoundToNegativeInfinity,
        )
      },
      &[0xc4, 0xe3, 0x19, 0x0b, 0xfb, 0x09],
    );
    check(
      |b| {
        b.vroundsd(
          R::XMM8.into(),
          R::XMM13.into(),
          idx(XMMWORD, R::R13 + R::RDX),
          RoundingModeX64::RoundToPositiveInfinity,
        )
      },
      &[0xc4, 0x43, 0x11, 0x0b, 0x44, 0x15, 0x00, 0x0a],
    );
    check(
      |b| {
        b.vroundsd(
          R::XMM9.into(),
          R::XMM14.into(),
          idx(XMMWORD, R::RCX + R::R10),
          RoundingModeX64::RoundToZero,
        )
      },
      &[0xc4, 0x23, 0x09, 0x0b, 0x0c, 0x11, 0x0b],
    );

    check(
      |b| {
        b.vroundps(
          R::XMM1.into(),
          R::XMM3.into(),
          RoundingModeX64::RoundToNegativeInfinity,
        )
      },
      &[0xc4, 0xe3, 0x79, 0x08, 0xcb, 0x09],
    );
    check(
      |b| {
        b.vroundps(
          R::XMM12.into(),
          R::XMM14.into(),
          RoundingModeX64::RoundToNegativeInfinity,
        )
      },
      &[0xc4, 0x43, 0x79, 0x08, 0xe6, 0x09],
    );
    check(
      |b| {
        b.vroundps(
          R::XMM12.into(),
          idx(XMMWORD, R::RAX + R::R13),
          RoundingModeX64::RoundToNegativeInfinity,
        )
      },
      &[0xc4, 0x23, 0x79, 0x08, 0x24, 0x28, 0x09],
    );

    check(
      |b| b.vblendvpd(R::XMM7, R::XMM12, idx(XMMWORD, R::RCX + R::R10), R::XMM5),
      &[0xc4, 0xa3, 0x19, 0x4b, 0x3c, 0x11, 0x50],
    );

    check(
      |b| b.vpshufps(R::XMM7, R::XMM12, idx(XMMWORD, R::RCX + R::R10), 0b11010100),
      &[0xc4, 0xa1, 0x18, 0xc6, 0x3c, 0x11, 0xd4],
    );
    check(
      |b| b.vpinsrd(R::XMM7, R::XMM12, idx(XMMWORD, R::RCX + R::R10), 2),
      &[0xc4, 0xa3, 0x19, 0x22, 0x3c, 0x11, 0x02],
    );

    check(
      |b| b.vpextrd(R::ECX, R::XMM5, 2),
      &[0xc4, 0xe3, 0x79, 0x16, 0xe9, 0x02],
    );
    check(
      |b| b.vpextrd(R::R10D, R::XMM9, 1),
      &[0xc4, 0x43, 0x79, 0x16, 0xca, 0x01],
    );

    check(
      |b| {
        b.vdpps(
          R::XMM7.into(),
          R::XMM12.into(),
          idx(XMMWORD, R::RCX + R::R10),
          2,
        )
      },
      &[0xc4, 0xa3, 0x19, 0x40, 0x3c, 0x11, 0x02],
    );
  }
}

mod assembly_builder_x_64_avx_unary_merge_instruction_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_avx_unary_merge_instruction_forms() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{DWORD, OperandX64, QWORD, XMMWORD, YMMWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| b.vsqrtpd(R::XMM8.into(), R::XMM10.into()),
      &[0xc4, 0x41, 0x79, 0x51, 0xc2],
    );
    check(
      |b| b.vsqrtpd(R::XMM8.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x79, 0x51, 0x01],
    );
    check(
      |b| b.vsqrtpd(R::YMM8.into(), R::YMM10.into()),
      &[0xc4, 0x41, 0x7d, 0x51, 0xc2],
    );
    check(
      |b| b.vsqrtpd(R::YMM8.into(), idx(YMMWORD, R::R9)),
      &[0xc4, 0x41, 0x7d, 0x51, 0x01],
    );
    check(
      |b| b.vsqrtps(R::XMM8.into(), R::XMM10.into()),
      &[0xc4, 0x41, 0x78, 0x51, 0xc2],
    );
    check(
      |b| b.vsqrtps(R::XMM8.into(), idx(XMMWORD, R::R9)),
      &[0xc4, 0x41, 0x78, 0x51, 0x01],
    );
    check(
      |b| b.vsqrtsd(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2b, 0x51, 0xc6],
    );
    check(
      |b| b.vsqrtsd(R::XMM8.into(), R::XMM10.into(), idx(QWORD, R::R9)),
      &[0xc4, 0x41, 0x2b, 0x51, 0x01],
    );
    check(
      |b| b.vsqrtss(R::XMM8.into(), R::XMM10.into(), R::XMM14.into()),
      &[0xc4, 0x41, 0x2a, 0x51, 0xc6],
    );
    check(
      |b| b.vsqrtss(R::XMM8.into(), R::XMM10.into(), idx(DWORD, R::R9)),
      &[0xc4, 0x41, 0x2a, 0x51, 0x01],
    );

    // Coverage for other instructions that follow the same pattern
    check(
      |b| b.vucomisd(R::XMM1.into(), R::XMM4.into()),
      &[0xc4, 0xe1, 0x79, 0x2e, 0xcc],
    );
  }
}

mod assembly_builder_x_64_base_binary_instruction_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_base_binary_instruction_forms() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{BYTE, DWORD, OperandX64, QWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    // `size[addr]` — the `[]` stamps the size prefix onto the address expression.
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    // reg, reg
    check(|b| b.add(R::RAX.into(), R::RCX.into()), &[0x48, 0x03, 0xc1]);
    check(|b| b.add(R::RSP.into(), R::R12.into()), &[0x49, 0x03, 0xe4]);
    check(|b| b.add(R::R14.into(), R::R10.into()), &[0x4d, 0x03, 0xf2]);

    // reg, imm
    check(
      |b| b.add(R::RAX.into(), 0i32.into()),
      &[0x48, 0x83, 0xc0, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), 0x7fi32.into()),
      &[0x48, 0x83, 0xc0, 0x7f],
    );
    check(
      |b| b.add(R::RAX.into(), 0x80i32.into()),
      &[0x48, 0x81, 0xc0, 0x80, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R10.into(), 0x7fffffffi32.into()),
      &[0x49, 0x81, 0xc2, 0xff, 0xff, 0xff, 0x7f],
    );
    check(|b| b.add(R::AL.into(), 3i32.into()), &[0x80, 0xc0, 0x03]);
    check(
      |b| b.add(R::SIL.into(), 3i32.into()),
      &[0x40, 0x80, 0xc6, 0x03],
    );
    check(
      |b| b.add(R::R11B.into(), 3i32.into()),
      &[0x41, 0x80, 0xc3, 0x03],
    );

    // reg, [reg]
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX)),
      &[0x48, 0x03, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBX)),
      &[0x48, 0x03, 0x03],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RSP)),
      &[0x48, 0x03, 0x04, 0x24],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBP)),
      &[0x48, 0x03, 0x45, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R10)),
      &[0x49, 0x03, 0x02],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R12)),
      &[0x49, 0x03, 0x04, 0x24],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R13)),
      &[0x49, 0x03, 0x45, 0x00],
    );

    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RAX)),
      &[0x4c, 0x03, 0x20],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBX)),
      &[0x4c, 0x03, 0x23],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RSP)),
      &[0x4c, 0x03, 0x24, 0x24],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBP)),
      &[0x4c, 0x03, 0x65, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R10)),
      &[0x4d, 0x03, 0x22],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R12)),
      &[0x4d, 0x03, 0x24, 0x24],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R13)),
      &[0x4d, 0x03, 0x65, 0x00],
    );

    // reg, [base+imm8]
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + 0x1b)),
      &[0x48, 0x03, 0x40, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBX + 0x1b)),
      &[0x48, 0x03, 0x43, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RSP + 0x1b)),
      &[0x48, 0x03, 0x44, 0x24, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBP + 0x1b)),
      &[0x48, 0x03, 0x45, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R10 + 0x1b)),
      &[0x49, 0x03, 0x42, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R12 + 0x1b)),
      &[0x49, 0x03, 0x44, 0x24, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R13 + 0x1b)),
      &[0x49, 0x03, 0x45, 0x1b],
    );

    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RAX + 0x1b)),
      &[0x4c, 0x03, 0x60, 0x1b],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBX + 0x1b)),
      &[0x4c, 0x03, 0x63, 0x1b],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RSP + 0x1b)),
      &[0x4c, 0x03, 0x64, 0x24, 0x1b],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBP + 0x1b)),
      &[0x4c, 0x03, 0x65, 0x1b],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R10 + 0x1b)),
      &[0x4d, 0x03, 0x62, 0x1b],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R12 + 0x1b)),
      &[0x4d, 0x03, 0x64, 0x24, 0x1b],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R13 + 0x1b)),
      &[0x4d, 0x03, 0x65, 0x1b],
    );

    // reg, [base+imm32]
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + 0xabab)),
      &[0x48, 0x03, 0x80, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBX + 0xabab)),
      &[0x48, 0x03, 0x83, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RSP + 0xabab)),
      &[0x48, 0x03, 0x84, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBP + 0xabab)),
      &[0x48, 0x03, 0x85, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R10 + 0xabab)),
      &[0x49, 0x03, 0x82, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R12 + 0xabab)),
      &[0x49, 0x03, 0x84, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R13 + 0xabab)),
      &[0x49, 0x03, 0x85, 0xab, 0xab, 0x00, 0x00],
    );

    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RAX + 0xabab)),
      &[0x4c, 0x03, 0xa0, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBX + 0xabab)),
      &[0x4c, 0x03, 0xa3, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RSP + 0xabab)),
      &[0x4c, 0x03, 0xa4, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBP + 0xabab)),
      &[0x4c, 0x03, 0xa5, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R10 + 0xabab)),
      &[0x4d, 0x03, 0xa2, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R12 + 0xabab)),
      &[0x4d, 0x03, 0xa4, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R13 + 0xabab)),
      &[0x4d, 0x03, 0xa5, 0xab, 0xab, 0x00, 0x00],
    );

    // reg, [index*scale]
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX * 2)),
      &[0x48, 0x03, 0x04, 0x45, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBX * 2)),
      &[0x48, 0x03, 0x04, 0x5d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RBP * 2)),
      &[0x48, 0x03, 0x04, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R10 * 2)),
      &[0x4a, 0x03, 0x04, 0x55, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R12 * 2)),
      &[0x4a, 0x03, 0x04, 0x65, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::R13 * 2)),
      &[0x4a, 0x03, 0x04, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );

    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RAX * 2)),
      &[0x4c, 0x03, 0x24, 0x45, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBX * 2)),
      &[0x4c, 0x03, 0x24, 0x5d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RBP * 2)),
      &[0x4c, 0x03, 0x24, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R10 * 2)),
      &[0x4e, 0x03, 0x24, 0x55, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R12 * 2)),
      &[0x4e, 0x03, 0x24, 0x65, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::R13 * 2)),
      &[0x4e, 0x03, 0x24, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );

    // reg, [base+index*scale+imm]
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::RAX * 2)),
      &[0x48, 0x03, 0x04, 0x40],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::RBX * 2 + 0x1b)),
      &[0x48, 0x03, 0x44, 0x58, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::RBP * 2)),
      &[0x48, 0x03, 0x04, 0x68],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::RBP + 0xabab)),
      &[0x48, 0x03, 0x84, 0x28, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::R12 + 0x1b)),
      &[0x4a, 0x03, 0x44, 0x20, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::R12 * 4 + 0xabab)),
      &[0x4a, 0x03, 0x84, 0xa0, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::R13 * 2 + 0x1b)),
      &[0x4a, 0x03, 0x44, 0x68, 0x1b],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, R::RAX + R::R13 + 0xabab)),
      &[0x4a, 0x03, 0x84, 0x28, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RAX + R::R12 * 2)),
      &[0x4e, 0x03, 0x24, 0x60],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RAX + R::R13 + 0xabab)),
      &[0x4e, 0x03, 0xa4, 0x28, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(R::R12.into(), idx(QWORD, R::RAX + R::RBP * 2 + 0x1b)),
      &[0x4c, 0x03, 0x64, 0x68, 0x1b],
    );

    // reg, [imm32]
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, 0)),
      &[0x48, 0x03, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.add(R::RAX.into(), idx(QWORD, 0xabab)),
      &[0x48, 0x03, 0x04, 0x25, 0xab, 0xab, 0x00, 0x00],
    );

    // [addr], reg
    check(
      |b| b.add(idx(QWORD, R::RAX), R::RAX.into()),
      &[0x48, 0x01, 0x00],
    );
    check(
      |b| b.add(idx(QWORD, R::RAX + R::RAX * 4 + 0xabab), R::RAX.into()),
      &[0x48, 0x01, 0x84, 0x80, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(idx(QWORD, R::RBX + R::RAX * 2 + 0x1b), R::RAX.into()),
      &[0x48, 0x01, 0x44, 0x43, 0x1b],
    );
    check(
      |b| b.add(idx(QWORD, R::RBX + R::RBP * 2 + 0x1b), R::RAX.into()),
      &[0x48, 0x01, 0x44, 0x6b, 0x1b],
    );
    check(
      |b| b.add(idx(QWORD, R::RBP + R::RBP * 4 + 0xabab), R::RAX.into()),
      &[0x48, 0x01, 0x84, 0xad, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(idx(QWORD, R::RBP + R::R12 + 0x1b), R::RAX.into()),
      &[0x4a, 0x01, 0x44, 0x25, 0x1b],
    );
    check(
      |b| b.add(idx(QWORD, R::R12), R::RAX.into()),
      &[0x49, 0x01, 0x04, 0x24],
    );
    check(
      |b| b.add(idx(QWORD, R::R13 + R::RBX + 0xabab), R::RAX.into()),
      &[0x49, 0x01, 0x84, 0x1d, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(idx(QWORD, R::RAX + R::R13 * 2 + 0x1b), R::RSI.into()),
      &[0x4a, 0x01, 0x74, 0x68, 0x1b],
    );
    check(
      |b| b.add(idx(QWORD, R::RBP + R::RBX * 2), R::RSI.into()),
      &[0x48, 0x01, 0x74, 0x5d, 0x00],
    );
    check(
      |b| b.add(idx(QWORD, R::RSP + R::R10 * 2 + 0x1b), R::R10.into()),
      &[0x4e, 0x01, 0x54, 0x54, 0x1b],
    );

    // [addr], imm
    check(
      |b| b.add(idx(BYTE, R::RAX), 2i32.into()),
      &[0x80, 0x00, 0x02],
    );
    check(
      |b| b.add(idx(DWORD, R::RAX), 2i32.into()),
      &[0x83, 0x00, 0x02],
    );
    check(
      |b| b.add(idx(DWORD, R::RAX), 0xabcdi32.into()),
      &[0x81, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.add(idx(QWORD, R::RAX), 2i32.into()),
      &[0x48, 0x83, 0x00, 0x02],
    );
    check(
      |b| b.add(idx(QWORD, R::RAX), 0xabcdi32.into()),
      &[0x48, 0x81, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
  }
}

mod assembly_builder_x_64_base_unary_instruction_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_base_unary_instruction_forms() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{BYTE, OperandX64, QWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(|b| b.div(R::RCX.into()), &[0x48, 0xf7, 0xf1]);
    check(|b| b.idiv(idx(QWORD, R::RAX)), &[0x48, 0xf7, 0x38]);
    check(
      |b| b.mul(idx(QWORD, R::RAX + R::RBX)),
      &[0x48, 0xf7, 0x24, 0x18],
    );
    check(|b| b.imul_operand_x_64(R::R9.into()), &[0x49, 0xf7, 0xe9]);
    check(|b| b.neg(R::R9.into()), &[0x49, 0xf7, 0xd9]);
    check(|b| b.not_(R::R12.into()), &[0x49, 0xf7, 0xd4]);
    check(|b| b.inc(R::R12.into()), &[0x49, 0xff, 0xc4]);
    check(|b| b.dec(R::ECX.into()), &[0xff, 0xc9]);
    check(|b| b.dec(idx(BYTE, R::RDX)), &[0xfe, 0x0a]);
  }
}

mod assembly_builder_x_64_constant_caching {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_constant_caching() {
    use ulua_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    let two = build.f64(2.0);

    // Force data relocation
    for i in 0..4096 {
      build.f64(i as f64);
    }

    assert_eq!(build.f64(2.0).imm, two.imm);

    build.finalize();
  }
}

mod assembly_builder_x_64_constant_storage {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_constant_storage() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64 as R,
    };

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    for i in 0..=3000i32 {
      let c = build.i32(i);
      build.vaddss(R::XMM0.into(), R::XMM0.into(), c);
    }

    build.finalize();

    assert_eq!(build.data.len(), 12004);

    for i in 0..=3000i32 {
      let u = i as usize;
      assert_eq!(build.data[u * 4], ((3000 - i) & 0xff) as u8);
      assert_eq!(build.data[u * 4 + 1], ((3000 - i) >> 8) as u8);
      assert_eq!(build.data[u * 4 + 2], 0x00);
      assert_eq!(build.data[u * 4 + 3], 0x00);
    }
  }
}

mod assembly_builder_x_64_constant_storage_dedup {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_constant_storage_dedup() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64 as R,
    };

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    for _ in 0..=3000 {
      let c = build.f32(1.0);
      build.vaddss(R::XMM0.into(), R::XMM0.into(), c);
    }

    build.finalize();

    assert_eq!(build.data.len(), 4);
    assert_eq!(build.data[0], 0x00);
    assert_eq!(build.data[1], 0x00);
    assert_eq!(build.data[2], 0x80);
    assert_eq!(build.data[3], 0x3f);
  }
}

mod assembly_builder_x_64_constants {
  use super::*;
  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_constants() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64 as R,
    };

    // C++ `AssemblyBuilderX64Fixture::check` with both code and data expectations.
    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8], data: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "code mismatch");
      assert_eq!(&build.data[..], data, "data mismatch");
    }

    check(
      |b| {
        b.xor_(R::RAX.into(), R::RAX.into());
        let c = b.i64(0x1234567887654321i64);
        b.add(R::RAX.into(), c);
        let c = b.f32(1.0);
        b.vmovss_operand_x_64_operand_x_64(R::XMM2.into(), c);
        let c = b.f64(1.0);
        b.vmovsd_operand_x_64_operand_x_64(R::XMM3.into(), c);
        let c = b.f32x4(1.0, 2.0, 4.0, 8.0);
        b.vmovaps(R::XMM4.into(), c);
        let arr: [u8; 16] = *b"hello world!123\0";
        let c = b.bytes(arr.as_ptr() as *const c_void, 16, 8);
        b.vmovupd(R::XMM5.into(), c);
        let c = b.f64x2(5.0, 6.0);
        b.vmovapd(R::XMM5.into(), c);
        b.ret();
      },
      &[
        0x48, 0x33, 0xc0, 0x48, 0x03, 0x05, 0xee, 0xff, 0xff, 0xff, 0xc4, 0xe1, 0x7a, 0x10, 0x15,
        0xe1, 0xff, 0xff, 0xff, 0xc4, 0xe1, 0x7b, 0x10, 0x1d, 0xcc, 0xff, 0xff, 0xff, 0xc4, 0xe1,
        0x78, 0x28, 0x25, 0xab, 0xff, 0xff, 0xff, 0xc4, 0xe1, 0x79, 0x10, 0x2d, 0x92, 0xff, 0xff,
        0xff, 0xc4, 0xe1, 0x79, 0x28, 0x2d, 0x79, 0xff, 0xff, 0xff, 0xc3,
      ],
      &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18,
        0x40, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x31, 0x32,
        0x33, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x80, 0x40, 0x00,
        0x00, 0x00, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xf0, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3f, 0x21, 0x43, 0x65,
        0x87, 0x78, 0x56, 0x34, 0x12,
      ],
    );
  }
}

mod assembly_builder_x_64_control_flow {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_control_flow() {
    use ulua_code_gen::{
      enums::condition_x_64::ConditionX64,
      records::{
        assembly_builder_x_64::AssemblyBuilderX64, label::Label, register_x_64::RegisterX64 as R,
      },
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    // Jump back (C++ `Label start = build.setLabel();` — no-arg form).
    check(
      |b| {
        let mut start = Label::default();
        b.set_label(&mut start);
        b.add(R::RSI.into(), 1i32.into());
        b.cmp(R::RSI.into(), R::RDI.into());
        b.jcc(ConditionX64::Equal, &mut start);
      },
      &[
        0x48, 0x83, 0xc6, 0x01, 0x48, 0x3b, 0xf7, 0x0f, 0x84, 0xf3, 0xff, 0xff, 0xff,
      ],
    );

    // Jump back, label set before use (C++ in-place `setLabel(start)`).
    check(
      |b| {
        let mut start = Label::default();
        b.add(R::RSI.into(), 1i32.into());
        b.set_label_label(&mut start);
        b.cmp(R::RSI.into(), R::RDI.into());
        b.jcc(ConditionX64::Equal, &mut start);
      },
      &[
        0x48, 0x83, 0xc6, 0x01, 0x48, 0x3b, 0xf7, 0x0f, 0x84, 0xf7, 0xff, 0xff, 0xff,
      ],
    );

    // Jump forward
    check(
      |b| {
        let mut skip = Label::default();
        b.cmp(R::RSI.into(), R::RDI.into());
        b.jcc(ConditionX64::Greater, &mut skip);
        b.or_(R::RDI.into(), 0x3ei32.into());
        b.set_label_label(&mut skip);
      },
      &[
        0x48, 0x3b, 0xf7, 0x0f, 0x8f, 0x04, 0x00, 0x00, 0x00, 0x48, 0x83, 0xcf, 0x3e,
      ],
    );

    // Regular jump
    check(
      |b| {
        let mut skip = Label::default();
        b.jmp_label(&mut skip);
        b.and_(R::RDI.into(), 0x3ei32.into());
        b.set_label_label(&mut skip);
      },
      &[0xe9, 0x04, 0x00, 0x00, 0x00, 0x48, 0x83, 0xe7, 0x3e],
    );
  }
}

mod assembly_builder_x_64_forms_of_absolute_jumps {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_absolute_jumps() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{OperandX64, QWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, address: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(address.into())
    }

    check(|b| b.jmp_operand_x_64(R::RAX.into()), &[0xff, 0xe0]);
    check(|b| b.jmp_operand_x_64(R::R14.into()), &[0x41, 0xff, 0xe6]);
    check(
      |b| b.jmp_operand_x_64(idx(QWORD, R::R14 + R::RDX * 4)),
      &[0x41, 0xff, 0x24, 0x96],
    );
    check(|b| b.call_operand_x_64(R::RAX.into()), &[0xff, 0xd0]);
    check(|b| b.call_operand_x_64(R::R14.into()), &[0x41, 0xff, 0xd6]);
    check(
      |b| b.call_operand_x_64(idx(QWORD, R::R14 + R::RDX * 4)),
      &[0x41, 0xff, 0x14, 0x96],
    );
  }
}

mod assembly_builder_x_64_forms_of_cmov {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_cmov() {
    use ulua_code_gen::{
      enums::condition_x_64::ConditionX64,
      records::{
        assembly_builder_x_64::AssemblyBuilderX64,
        operand_x_64::{OperandX64, QWORD},
        register_x_64::RegisterX64 as R,
      },
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| b.cmov(ConditionX64::LessEqual, R::EBX, R::EAX.into()),
      &[0x0f, 0x4e, 0xd8],
    );
    check(
      |b| b.cmov(ConditionX64::NotZero, R::RBX, idx(QWORD, R::RAX)),
      &[0x48, 0x0f, 0x45, 0x18],
    );
    check(
      |b| b.cmov(ConditionX64::Zero, R::RBX, idx(QWORD, R::RAX + R::RCX)),
      &[0x48, 0x0f, 0x44, 0x1c, 0x08],
    );
    check(
      |b| b.cmov(ConditionX64::BelowEqual, R::R14D, R::R15D.into()),
      &[0x45, 0x0f, 0x46, 0xf7],
    );
  }
}

mod assembly_builder_x_64_forms_of_imul {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_imul() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{OperandX64, QWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| b.imul_operand_x_64_operand_x_64(R::ECX.into(), R::ESI.into()),
      &[0x0f, 0xaf, 0xce],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64(R::R12.into(), R::RAX.into()),
      &[0x4c, 0x0f, 0xaf, 0xe0],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64(R::R12.into(), idx(QWORD, R::RDX + R::RDI)),
      &[0x4c, 0x0f, 0xaf, 0x24, 0x3a],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64_i32(R::ECX.into(), R::EDX.into(), 8),
      &[0x6b, 0xca, 0x08],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64_i32(R::ECX.into(), R::R9D.into(), 0xabcd),
      &[0x41, 0x69, 0xc9, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64_i32(R::R8D.into(), R::EAX.into(), -9),
      &[0x44, 0x6b, 0xc0, 0xf7],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64_i32(R::RCX.into(), R::RDX.into(), 17),
      &[0x48, 0x6b, 0xca, 0x11],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64_i32(R::RCX.into(), R::R12.into(), 0xabcd),
      &[0x49, 0x69, 0xcc, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.imul_operand_x_64_operand_x_64_i32(R::R12.into(), R::RAX.into(), -13),
      &[0x4c, 0x6b, 0xe0, 0xf3],
    );
  }
}

mod assembly_builder_x_64_forms_of_lea {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_lea() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{ADDR, OperandX64},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, address: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(address.into())
    }

    check(
      |b| b.lea_operand_x_64_operand_x_64(R::RAX.into(), idx(ADDR, R::RDX + R::RCX)),
      &[0x48, 0x8d, 0x04, 0x0a],
    );
    check(
      |b| b.lea_operand_x_64_operand_x_64(R::RAX.into(), idx(ADDR, R::RDX + R::RAX * 4)),
      &[0x48, 0x8d, 0x04, 0x82],
    );
    check(
      |b| b.lea_operand_x_64_operand_x_64(R::RAX.into(), idx(ADDR, R::R13 + R::R12 * 4 + 4)),
      &[0x4b, 0x8d, 0x44, 0xa5, 0x04],
    );
  }
}

mod assembly_builder_x_64_forms_of_mov {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_mov() {
    use ulua_code_gen::{
      functions::word_reg::word_reg,
      records::{
        assembly_builder_x_64::AssemblyBuilderX64,
        operand_x_64::{BYTE, DWORD, OperandX64, QWORD, WORD},
        register_x_64::RegisterX64 as R,
      },
    };

    // C++ `AssemblyBuilderX64Fixture::check`: a FRESH builder per case, run one
    // instruction, finalize, assert `build.code` matches the expected encoding.
    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    // `size[reg]` addressing: `operator[]` stamps the size prefix onto the address.
    fn mem(prefix: OperandX64, reg: R) -> OperandX64 {
      prefix.operator_bracket(OperandX64::from(reg))
    }

    check(
      |b| b.mov(R::RCX.into(), 1i32.into()),
      &[0x48, 0xb9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.mov64(R::RCX, 0x1234567812345678i64),
      &[0x48, 0xb9, 0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12],
    );
    check(
      |b| b.mov(R::ECX.into(), 2i32.into()),
      &[0xb9, 0x02, 0x00, 0x00, 0x00],
    );
    check(|b| b.mov(R::CL.into(), 2i32.into()), &[0xb1, 0x02]);
    check(|b| b.mov(R::SIL.into(), 2i32.into()), &[0x40, 0xb6, 0x02]);
    check(|b| b.mov(R::R9B.into(), 2i32.into()), &[0x41, 0xb1, 0x02]);
    check(
      |b| b.mov(R::RCX.into(), mem(QWORD, R::RDI)),
      &[0x48, 0x8b, 0x0f],
    );
    check(
      |b| b.mov(mem(DWORD, R::RAX), 0xabcdi32.into()),
      &[0xc7, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.mov(R::R13.into(), 1i32.into()),
      &[0x49, 0xbd, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.mov64(R::R13, 0x1234567812345678i64),
      &[0x49, 0xbd, 0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12],
    );
    check(
      |b| b.mov(R::R13D.into(), 2i32.into()),
      &[0x41, 0xbd, 0x02, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.mov(R::R13.into(), mem(QWORD, R::R12)),
      &[0x4d, 0x8b, 0x2c, 0x24],
    );
    check(
      |b| b.mov(mem(DWORD, R::R13), 0xabcdi32.into()),
      &[0x41, 0xc7, 0x45, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.mov(mem(QWORD, R::RDX), R::R9.into()),
      &[0x4c, 0x89, 0x0a],
    );
    check(
      |b| b.mov(mem(BYTE, R::RSI), 0x3i32.into()),
      &[0xc6, 0x06, 0x03],
    );
    check(|b| b.mov(mem(BYTE, R::RSI), R::AL.into()), &[0x88, 0x06]);
    check(
      |b| b.mov(mem(BYTE, R::RSI), R::DIL.into()),
      &[0x40, 0x88, 0x3e],
    );
    check(
      |b| b.mov(mem(BYTE, R::RSI), R::R10B.into()),
      &[0x44, 0x88, 0x16],
    );
    check(
      |b| b.mov(word_reg(R::EBX).into(), 0x3a3di32.into()),
      &[0x66, 0xbb, 0x3d, 0x3a],
    );
    check(
      |b| b.mov(mem(WORD, R::RSI), 0x3a3di32.into()),
      &[0x66, 0xc7, 0x06, 0x3d, 0x3a],
    );
    check(
      |b| b.mov(mem(WORD, R::RSI), word_reg(R::EAX).into()),
      &[0x66, 0x89, 0x06],
    );
    check(
      |b| b.mov(mem(WORD, R::RSI), word_reg(R::EDI).into()),
      &[0x66, 0x89, 0x3e],
    );
    check(
      |b| b.mov(mem(WORD, R::RSI), word_reg(R::R10).into()),
      &[0x66, 0x44, 0x89, 0x16],
    );
  }
}

mod assembly_builder_x_64_forms_of_mov_extended {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_mov_extended() {
    use ulua_code_gen::{
      functions::word_reg::word_reg,
      records::{
        assembly_builder_x_64::AssemblyBuilderX64,
        operand_x_64::{BYTE, OperandX64, WORD},
        register_x_64::RegisterX64 as R,
      },
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn mem(prefix: OperandX64, reg: R) -> OperandX64 {
      prefix.operator_bracket(OperandX64::from(reg))
    }

    check(|b| b.movsx(R::EAX, mem(BYTE, R::RCX)), &[0x0f, 0xbe, 0x01]);
    check(
      |b| b.movsx(R::R12, mem(BYTE, R::R10)),
      &[0x4d, 0x0f, 0xbe, 0x22],
    );
    check(
      |b| b.movsx(R::EBX, mem(WORD, R::R11)),
      &[0x41, 0x0f, 0xbf, 0x1b],
    );
    check(
      |b| b.movsx(R::RDX, mem(WORD, R::RCX)),
      &[0x48, 0x0f, 0xbf, 0x11],
    );
    check(|b| b.movsx(R::EDX, R::CL.into()), &[0x0f, 0xbe, 0xd1]);
    check(
      |b| b.movsx(R::EDX, R::R12B.into()),
      &[0x41, 0x0f, 0xbe, 0xd4],
    );
    check(
      |b| b.movsx(R::EDX, word_reg(R::ECX).into()),
      &[0x0f, 0xbf, 0xd1],
    );
    check(
      |b| b.movsx(R::EDX, word_reg(R::R12D).into()),
      &[0x41, 0x0f, 0xbf, 0xd4],
    );
    check(|b| b.movzx(R::EAX, mem(BYTE, R::RCX)), &[0x0f, 0xb6, 0x01]);
    check(
      |b| b.movzx(R::R12, mem(BYTE, R::R10)),
      &[0x4d, 0x0f, 0xb6, 0x22],
    );
    check(
      |b| b.movzx(R::EBX, mem(WORD, R::R11)),
      &[0x41, 0x0f, 0xb7, 0x1b],
    );
    check(
      |b| b.movzx(R::RDX, mem(WORD, R::RCX)),
      &[0x48, 0x0f, 0xb7, 0x11],
    );
    check(|b| b.movzx(R::EDX, R::CL.into()), &[0x0f, 0xb6, 0xd1]);
    check(
      |b| b.movzx(R::EDX, R::R12B.into()),
      &[0x41, 0x0f, 0xb6, 0xd4],
    );
    check(
      |b| b.movzx(R::EDX, word_reg(R::ECX).into()),
      &[0x0f, 0xb7, 0xd1],
    );
    check(
      |b| b.movzx(R::EDX, word_reg(R::R12D).into()),
      &[0x41, 0x0f, 0xb7, 0xd4],
    );
  }
}

mod assembly_builder_x_64_forms_of_setcc {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_setcc() {
    use ulua_code_gen::{
      enums::condition_x_64::ConditionX64,
      records::{
        assembly_builder_x_64::AssemblyBuilderX64,
        operand_x_64::{BYTE, OperandX64},
        register_x_64::RegisterX64 as R,
      },
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(
      |b| b.setcc(ConditionX64::NotEqual, R::BL.into()),
      &[0x0f, 0x95, 0xc3],
    );
    check(
      |b| b.setcc(ConditionX64::NotEqual, R::DIL.into()),
      &[0x40, 0x0f, 0x95, 0xc7],
    );
    check(
      |b| b.setcc(ConditionX64::BelowEqual, idx(BYTE, R::RCX)),
      &[0x0f, 0x96, 0x01],
    );
  }
}

mod assembly_builder_x_64_forms_of_shift {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_shift() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(|b| b.shl(R::AL.into(), 1i32.into()), &[0xd0, 0xe0]);
    check(|b| b.shl(R::AL.into(), R::CL.into()), &[0xd2, 0xe0]);
    check(|b| b.shl(R::SIL.into(), R::CL.into()), &[0x40, 0xd2, 0xe6]);
    check(|b| b.shl(R::R10B.into(), R::CL.into()), &[0x41, 0xd2, 0xe2]);
    check(|b| b.shr(R::AL.into(), 4i32.into()), &[0xc0, 0xe8, 0x04]);
    check(|b| b.shr(R::EAX.into(), 1i32.into()), &[0xd1, 0xe8]);
    check(|b| b.sal(R::EAX.into(), R::CL.into()), &[0xd3, 0xe0]);
    check(|b| b.sal(R::EAX.into(), 4i32.into()), &[0xc1, 0xe0, 0x04]);
    check(
      |b| b.sar(R::RAX.into(), 4i32.into()),
      &[0x48, 0xc1, 0xf8, 0x04],
    );
    check(|b| b.sar(R::R11.into(), 1i32.into()), &[0x49, 0xd1, 0xfb]);
    check(|b| b.rol(R::EAX.into(), 1i32.into()), &[0xd1, 0xc0]);
    check(|b| b.rol(R::EAX.into(), R::CL.into()), &[0xd3, 0xc0]);
    check(|b| b.ror(R::EAX.into(), 1i32.into()), &[0xd1, 0xc8]);
    check(|b| b.ror(R::EAX.into(), R::CL.into()), &[0xd3, 0xc8]);
  }
}

mod assembly_builder_x_64_forms_of_test {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_forms_of_test() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      operand_x_64::{OperandX64, QWORD},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(addr.into())
    }

    check(|b| b.test(R::AL.into(), 8i32.into()), &[0xf6, 0xc0, 0x08]);
    check(
      |b| b.test(R::EAX.into(), 8i32.into()),
      &[0xf7, 0xc0, 0x08, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.test(R::RAX.into(), 8i32.into()),
      &[0x48, 0xf7, 0xc0, 0x08, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.test(R::RCX.into(), 0xababi32.into()),
      &[0x48, 0xf7, 0xc1, 0xab, 0xab, 0x00, 0x00],
    );
    check(
      |b| b.test(R::RCX.into(), R::RAX.into()),
      &[0x48, 0x85, 0xc8],
    );
    check(
      |b| b.test(R::RAX.into(), idx(QWORD, R::RCX)),
      &[0x48, 0x85, 0x01],
    );
    check(|b| b.test(R::AL.into(), R::CL.into()), &[0x84, 0xc1]);
    check(|b| b.test(R::AL.into(), R::SIL.into()), &[0x40, 0x84, 0xc6]);
    check(
      |b| b.test(R::CL.into(), R::R12B.into()),
      &[0x41, 0x84, 0xcc],
    );
    check(
      |b| b.test(R::SIL.into(), R::DIL.into()),
      &[0x40, 0x84, 0xf7],
    );
  }
}

mod assembly_builder_x_64_label_call {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_label_call() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64,
      label::Label,
      operand_x_64::{ADDR, OperandX64},
      register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, address: impl Into<OperandX64>) -> OperandX64 {
      prefix.operator_bracket(address.into())
    }

    check(
      |b| {
        let mut fn_b = Label::default();
        b.and_(R::RCX.into(), 0x3ei32.into());
        b.call_label(&mut fn_b);
        b.ret();
        b.set_label_label(&mut fn_b);
        b.lea_operand_x_64_operand_x_64(R::RAX.into(), idx(ADDR, R::RCX + 0x1f));
        b.ret();
      },
      &[
        0x48, 0x83, 0xe1, 0x3e, 0xe8, 0x01, 0x00, 0x00, 0x00, 0xc3, 0x48, 0x8d, 0x41, 0x1f, 0xc3,
      ],
    );
  }
}

mod assembly_builder_x_64_label_lea {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_label_lea() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64, label::Label, register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(
      |b| {
        let mut f = Label::default();
        b.lea_register_x_64_label(R::RAX, &mut f);
        b.ret();
        b.set_label_label(&mut f);
        b.ret();
      },
      &[0x48, 0x8d, 0x05, 0x01, 0x00, 0x00, 0x00, 0xc3, 0xc3],
    );
  }
}

mod assembly_builder_x_64_log_test {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_log_test() {
    use ulua_code_gen::{
      enums::{
        alignment_data_x_64::AlignmentDataX64, condition_x_64::ConditionX64,
        rounding_mode_x_64::RoundingModeX64,
      },
      records::{
        assembly_builder_x_64::AssemblyBuilderX64,
        label::Label,
        operand_x_64::{ADDR, BYTE, DWORD, OperandX64, QWORD, WORD, XMMWORD, YMMWORD},
        register_x_64::RegisterX64 as R,
      },
    };

    // `size[addr]` — the `[]` stamps the size prefix onto the address expression.
    fn idx(prefix: OperandX64, address: OperandX64) -> OperandX64 {
      prefix.operator_bracket(address)
    }

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(true, 0); // logText = true

    build.push(R::R12.into());
    build.align(8, AlignmentDataX64::Nop);
    build.align(8, AlignmentDataX64::Int3);
    build.align(8, AlignmentDataX64::Ud2);

    build.add(R::RAX.into(), R::RDI.into());
    build.add(R::RCX.into(), 8i32.into());
    build.sub(idx(DWORD, R::RAX.into()), 0x1fdci32.into());
    build.and_(idx(DWORD, R::RCX.into()), 0x37i32.into());
    build.mov(R::RDI.into(), idx(QWORD, R::RAX + R::RSI * 2));
    build.vaddss(
      R::XMM0.into(),
      R::XMM0.into(),
      idx(DWORD, R::RAX + R::R14 * 2 + 0x1c),
    );

    // C++ `Label start = build.setLabel();` — no-arg form: fresh label, set here.
    let mut start = Label::default();
    build.set_label(&mut start);
    build.cmp(R::RSI.into(), R::RDI.into());
    build.jcc(ConditionX64::Equal, &mut start);
    build.lea_register_x_64_label(R::RCX, &mut start);
    build.lea_operand_x_64_operand_x_64(R::RCX.into(), idx(ADDR, R::RDX.into()));

    build.jmp_operand_x_64(idx(QWORD, R::RDX.into()));
    build.vaddps(R::YMM9.into(), R::YMM12.into(), idx(YMMWORD, R::RBP + 0xc));
    let c = build.f64(2.5);
    build.vaddpd(R::YMM2.into(), R::YMM7.into(), c);
    build.neg(idx(QWORD, R::RBP + R::R12 * 2));
    build.mov64(R::R10, 0x1234567812345678i64);
    build.vmovapd(idx(XMMWORD, R::RAX.into()), R::XMM11.into());
    build.movzx(R::EAX, idx(BYTE, R::RCX.into()));
    build.movsx(R::RSI, idx(WORD, R::R12.into()));
    build.imul_operand_x_64_operand_x_64(R::RCX.into(), R::RDX.into());
    build.imul_operand_x_64_operand_x_64_i32(R::RCX.into(), R::RDX.into(), 8);
    build.vroundsd(
      R::XMM1.into(),
      R::XMM2.into(),
      R::XMM3.into(),
      RoundingModeX64::RoundToNearestEven,
    );
    build.vroundps(
      R::XMM1.into(),
      R::XMM12.into(),
      RoundingModeX64::RoundToNegativeInfinity,
    );
    build.add(R::RDX.into(), idx(QWORD, R::RCX - 12));
    build.pop(R::R12.into());
    build.cmov(ConditionX64::AboveEqual, R::RAX, R::RBX.into());
    build.vpextrd(R::ECX, R::XMM5, 2);
    build.ret();
    build.int3();

    build.nop(1);
    build.nop(2);
    build.nop(3);
    build.nop(4);
    build.nop(5);
    build.nop(6);
    build.nop(7);
    build.nop(8);
    build.nop(9);

    build.finalize();

    let expected = "\n push        r12\n; align 8\n nop         word ptr[rax+rax] ; 6-byte nop\n; align 8 using int3\n; align 8 using ud2\n add         rax,rdi\n add         rcx,8\n sub         dword ptr [rax],1FDCh\n and         dword ptr [rcx],37h\n mov         rdi,qword ptr [rax+rsi*2]\n vaddss      xmm0,xmm0,dword ptr [rax+r14*2+01Ch]\n.L1:\n cmp         rsi,rdi\n je          .L1\n lea         rcx,.L1\n lea         rcx,[rdx]\n jmp         qword ptr [rdx]\n vaddps      ymm9,ymm12,ymmword ptr [rbp+0Ch]\n vaddpd      ymm2,ymm7,qword ptr [.start-8]\n neg         qword ptr [rbp+r12*2]\n mov         r10,1234567812345678h\n vmovapd     xmmword ptr [rax],xmm11\n movzx       eax,byte ptr [rcx]\n movsx       rsi,word ptr [r12]\n imul        rcx,rdx\n imul        rcx,rdx,8\n vroundsd    xmm1,xmm2,xmm3,8\n vroundps    xmm1,xmm12,9\n add         rdx,qword ptr [rcx-0Ch]\n pop         r12\n cmovae      rax,rbx\n vpextrd     ecx,xmm5,2\n ret\n int3\n nop\n xchg        ax, ax ; 2-byte nop\n nop         dword ptr[rax] ; 3-byte nop\n nop         dword ptr[rax] ; 4-byte nop\n nop         dword ptr[rax+rax] ; 5-byte nop\n nop         word ptr[rax+rax] ; 6-byte nop\n nop         dword ptr[rax] ; 7-byte nop\n nop         dword ptr[rax+rax] ; 8-byte nop\n nop         word ptr[rax+rax] ; 9-byte nop\n";

    assert_eq!(
      format!("\n{}", build.text),
      expected,
      "disasm text mismatch"
    );
  }
}

mod assembly_builder_x_64_misc_instructions {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_misc_instructions() {
    use ulua_code_gen::records::{
      assembly_builder_x_64::AssemblyBuilderX64, register_x_64::RegisterX64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(|b| b.int3(), &[0xcc]);
    check(|b| b.ud_2(), &[0x0f, 0x0b]);
    check(|b| b.bsr(R::EAX, R::EDX.into()), &[0x0f, 0xbd, 0xc2]);
    check(|b| b.bsf(R::EAX, R::EDX.into()), &[0x0f, 0xbc, 0xc2]);
    check(|b| b.bswap(R::EAX), &[0x0f, 0xc8]);
    check(|b| b.bswap(R::R12D), &[0x41, 0x0f, 0xcc]);
    check(|b| b.bswap(R::RAX), &[0x48, 0x0f, 0xc8]);
    check(|b| b.bswap(R::R12), &[0x49, 0x0f, 0xcc]);
  }
}

mod assembly_builder_x_64_nop_forms {

  #[cfg(test)]
  #[test]
  fn assembly_builder_x_64_nop_forms() {
    use ulua_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
      let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(|b| b.nop(1), &[0x90]);
    check(|b| b.nop(2), &[0x66, 0x90]);
    check(|b| b.nop(3), &[0x0f, 0x1f, 0x00]);
    check(|b| b.nop(4), &[0x0f, 0x1f, 0x40, 0x00]);
    check(|b| b.nop(5), &[0x0f, 0x1f, 0x44, 0x00, 0x00]);
    check(|b| b.nop(6), &[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00]);
    check(|b| b.nop(7), &[0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00]);
    check(
      |b| b.nop(8),
      &[0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.nop(9),
      &[0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    check(
      |b| b.nop(15),
      &[
        0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00,
      ],
    );
  }
}
