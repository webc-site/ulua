extern crate alloc;

mod assembly_builder_a_64_address_of_label {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_address_of_label() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: &[u32]) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(
      |b| {
        let mut label = Label::default();
        b.adr_register_a_64_label(R::X0, &mut label);
        b.add_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X0, R::X0, 0);
        b.set_label_label(&mut label);
      },
      &[0x10000040, 0x8b000000],
    );
  }
}

mod assembly_builder_a_64_address_offset_size {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_address_offset_size() {
    use ulua_code_gen::{
      records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
      type_aliases::mem::mem,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
      |b| b.ldr(RegisterA64::W0, mem(RegisterA64::X1, 16)),
      0xB9401020,
    );
    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, 16)),
      0xF9400820,
    );
    check(
      |b| b.ldr(RegisterA64::D0, mem(RegisterA64::X1, 16)),
      0xFD400820,
    );
    check(
      |b| b.ldr(RegisterA64::Q0, mem(RegisterA64::X1, 16)),
      0x3DC00420,
    );

    check(
      |b| b.str(RegisterA64::W0, mem(RegisterA64::X1, 16)),
      0xB9001020,
    );
    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::X1, 16)),
      0xF9000820,
    );
    check(
      |b| b.str(RegisterA64::D0, mem(RegisterA64::X1, 16)),
      0xFD000820,
    );
    check(
      |b| b.str(RegisterA64::Q0, mem(RegisterA64::X1, 16)),
      0x3D800420,
    );
  }
}

mod assembly_builder_a_64_binary {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_binary() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    // reg, reg
    check(
      |b| b.add_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 0),
      0x8B020020,
    );
    check(
      |b| b.add_register_a_64_register_a_64_register_a_64_i32(R::W0, R::W1, R::W2, 0),
      0x0B020020,
    );
    check(
      |b| b.add_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 7),
      0x8B021C20,
    );
    check(
      |b| b.add_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, -7),
      0x8B421C20,
    );
    check(
      |b| b.sub_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 0),
      0xCB020020,
    );
    check(
      |b| b.and_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 0),
      0x8A020020,
    );
    check(
      |b| b.and_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 7),
      0x8A021C20,
    );
    check(
      |b| b.and_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, -7),
      0x8A421C20,
    );
    check(|b| b.bic(R::X0, R::X1, R::X2, 0), 0x8A220020);
    check(
      |b| b.orr_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 0),
      0xAA020020,
    );
    check(
      |b| b.eor_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 0),
      0xCA020020,
    );
    check(
      |b| b.lsl_register_a_64_register_a_64_register_a_64(R::X0, R::X1, R::X2),
      0x9AC22020,
    );
    check(
      |b| b.lsl_register_a_64_register_a_64_register_a_64(R::W0, R::W1, R::W2),
      0x1AC22020,
    );
    check(
      |b| b.lsr_register_a_64_register_a_64_register_a_64(R::X0, R::X1, R::X2),
      0x9AC22420,
    );
    check(
      |b| b.asr_register_a_64_register_a_64_register_a_64(R::X0, R::X1, R::X2),
      0x9AC22820,
    );
    check(
      |b| b.ror_register_a_64_register_a_64_register_a_64(R::X0, R::X1, R::X2),
      0x9AC22C20,
    );
    check(
      |b| b.cmp_register_a_64_register_a_64(R::X0, R::X1),
      0xEB01001F,
    );
    check(
      |b| b.tst_register_a_64_register_a_64_i32(R::X0, R::X1, 0),
      0xEA01001F,
    );

    // reg, imm
    check(
      |b| b.add_register_a_64_register_a_64_u16(R::X3, R::X7, 78),
      0x910138E3,
    );
    check(
      |b| b.add_register_a_64_register_a_64_u16(R::W3, R::W7, 78),
      0x110138E3,
    );
    check(
      |b| b.sub_register_a_64_register_a_64_u16(R::W3, R::W7, 78),
      0x510138E3,
    );
    check(|b| b.cmp_register_a_64_u16(R::W0, 42), 0x7100A81F);
  }
}

mod assembly_builder_a_64_binary_extended {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_binary_extended() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.add_register_a_64_register_a_64_register_a_64_i32(
      RegisterA64::X0,
      RegisterA64::X1,
      RegisterA64::W2,
      3,
    );
    build.finalize();
    assert_eq!(build.code[0], 0x8B224C20, "instruction word mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.sub_register_a_64_register_a_64_register_a_64_i32(
      RegisterA64::X0,
      RegisterA64::X1,
      RegisterA64::W2,
      3,
    );
    build.finalize();
    assert_eq!(build.code[0], 0xCB224C20, "instruction word mismatch");
  }
}

mod assembly_builder_a_64_binary_imm {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_binary_imm() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    // instructions
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W1, R::W2, 1),
      0x12000041,
    );
    check(
      |b| b.orr_register_a_64_register_a_64_u32(R::W1, R::W2, 1),
      0x32000041,
    );
    check(
      |b| b.eor_register_a_64_register_a_64_u32(R::W1, R::W2, 1),
      0x52000041,
    );
    check(|b| b.tst_register_a_64_u32(R::W1, 1), 0x7200003f);

    // various mask forms
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W0, R::W0, 1),
      0x12000000,
    );
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W0, R::W0, 3),
      0x12000400,
    );
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W0, R::W0, 7),
      0x12000800,
    );
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W0, R::W0, 2147483647),
      0x12007800,
    );
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W0, R::W0, 6),
      0x121F0400,
    );
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W0, R::W0, 12),
      0x121E0400,
    );
    check(
      |b| b.and_register_a_64_register_a_64_u32(R::W0, R::W0, 2147483648),
      0x12010000,
    );

    // shifts
    check(
      |b| b.lsl_register_a_64_register_a_64_u8(R::W1, R::W2, 1),
      0x531F7841,
    );
    check(
      |b| b.lsl_register_a_64_register_a_64_u8(R::X1, R::X2, 1),
      0xD37FF841,
    );
    check(
      |b| b.lsr_register_a_64_register_a_64_u8(R::W1, R::W2, 1),
      0x53017C41,
    );
    check(
      |b| b.lsr_register_a_64_register_a_64_u8(R::X1, R::X2, 1),
      0xD341FC41,
    );
    check(
      |b| b.asr_register_a_64_register_a_64_u8(R::W1, R::W2, 1),
      0x13017C41,
    );
    check(
      |b| b.asr_register_a_64_register_a_64_u8(R::X1, R::X2, 1),
      0x9341FC41,
    );
    check(
      |b| b.ror_register_a_64_register_a_64_u8(R::W1, R::W2, 1),
      0x13820441,
    );
    check(
      |b| b.ror_register_a_64_register_a_64_u8(R::X1, R::X2, 1),
      0x93C20441,
    );
  }
}

mod assembly_builder_a_64_bitfield {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_bitfield() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
      |b| b.ubfiz(RegisterA64::X1, RegisterA64::X2, 37, 5),
      0xD35B1041,
    );
    check(
      |b| b.ubfx(RegisterA64::X1, RegisterA64::X2, 37, 5),
      0xD365A441,
    );
    check(
      |b| b.sbfiz(RegisterA64::X1, RegisterA64::X2, 37, 5),
      0x935B1041,
    );
    check(
      |b| b.sbfx_register_a_64_register_a_64_u8_u8(RegisterA64::X1, RegisterA64::X2, 37, 5),
      0x9365A441,
    );

    check(
      |b| b.ubfiz(RegisterA64::W1, RegisterA64::W2, 17, 5),
      0x530F1041,
    );
    check(
      |b| b.ubfx(RegisterA64::W1, RegisterA64::W2, 17, 5),
      0x53115441,
    );
    check(
      |b| b.sbfiz(RegisterA64::W1, RegisterA64::W2, 17, 5),
      0x130F1041,
    );
    check(
      |b| b.sbfx_register_a_64_register_a_64_u8_u8(RegisterA64::W1, RegisterA64::W2, 17, 5),
      0x13115441,
    );
  }
}

mod assembly_builder_a_64_conditionals {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_conditionals() {
    use ulua_code_gen::{
      enums::condition_a_64::ConditionA64,
      records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64 as R},
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
      |b| b.csel(R::X0, R::X1, R::X2, ConditionA64::Equal),
      0x9A820020,
    );
    check(
      |b| b.csel(R::W0, R::W1, R::W2, ConditionA64::Equal),
      0x1A820020,
    );
    check(
      |b| b.fcsel(R::D0, R::D1, R::D2, ConditionA64::Equal),
      0x1E620C20,
    );

    check(|b| b.cset(R::X1, ConditionA64::Less), 0x9A9FA7E1);
  }
}

mod assembly_builder_a_64_constants {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_constants() {
    use alloc::vec::Vec;
    use core::ffi::c_void;

    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64 as R,
    };
    use ulua_unit_test::records::assembly_builder_a_64_fixture::AssemblyBuilderA64Fixture;

    let mut fixture = AssemblyBuilderA64Fixture::default();

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);

    let arr: [u8; 12] = *b"hello world\0";
    let ptr = arr.as_ptr() as *const c_void;
    build.adr_register_a_64_void_usize(R::X0, ptr, 12);

    build.adr_register_a_64_u64(R::X0, 0x1234567887654321u64);

    build.adr_register_a_64_f64(R::X0, 1.0f64);

    build.finalize();

    let expected_code: Vec<u32> = vec![0x10ffffa0, 0x10ffff20, 0x10fffec0];
    let expected_data: Vec<u8> = vec![
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, 0x21, 0x43, 0x65, 0x87, 0x78, 0x56, 0x34,
      0x12, 0x00, 0x00, 0x00, 0x00, b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l',
      b'd', 0x00,
    ];

    let result = fixture.check(
      |b| {
        let arr: [u8; 12] = *b"hello world\0";
        let ptr = arr.as_ptr() as *const c_void;
        b.adr_register_a_64_void_usize(R::X0, ptr, 12);
        b.adr_register_a_64_u64(R::X0, 0x1234567887654321u64);
        b.adr_register_a_64_f64(R::X0, 1.0f64);
      },
      expected_code,
      expected_data,
      0,
    );

    assert!(result, "Constants check failed");
  }
}

mod assembly_builder_a_64_control_flow {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_control_flow() {
    use ulua_code_gen::{
      enums::condition_a_64::ConditionA64,
      records::{
        assembly_builder_a_64::AssemblyBuilderA64, label::Label, register_a_64::RegisterA64 as R,
      },
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: &[u32]) -> bool {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      &build.code[..] == code
    }

    // Jump back
    let mut start = Label::default();
    let result0 = check(
      |b| {
        start = b.set_label();
        b.mov_register_a_64_register_a_64(R::X0, R::X1);
        b.b_condition_a_64_label(ConditionA64::Equal, &mut start);
      },
      &[0xAA0103E0, 0x54FFFFE0],
    );
    assert!(result0, "Jump back check failed");

    // Jump forward
    let mut skip = Label::default();
    let result1 = check(
      |b| {
        b.b_condition_a_64_label(ConditionA64::Equal, &mut skip);
        b.mov_register_a_64_register_a_64(R::X0, R::X1);
        b.set_label_label(&mut skip);
      },
      &[0x54000040, 0xAA0103E0],
    );
    assert!(result1, "Jump forward check failed");

    // Jumps
    let mut skip2 = Label::default();
    let result2 = check(
      |b| {
        b.b_condition_a_64_label(ConditionA64::Equal, &mut skip2);
        b.cbz(R::X0, &mut skip2);
        b.cbnz(R::X0, &mut skip2);
        b.tbz(R::X0, 5, &mut skip2);
        b.tbnz(R::X0, 5, &mut skip2);
        b.set_label_label(&mut skip2);
        b.b_label(&mut skip2);
        b.bl(&mut skip2);
      },
      &[
        0x540000A0, 0xB4000080, 0xB5000060, 0x36280040, 0x37280020, 0x14000000, 0x97ffffff,
      ],
    );
    assert!(result2, "Jumps check failed");

    // Basic control flow
    let mut build3 = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build3.br(R::X0);
    build3.finalize();
    assert_eq!(build3.code[0], 0xD61F0000u32, "br(x0) instruction mismatch");

    let mut build4 = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build4.blr(R::X0);
    build4.finalize();
    assert_eq!(
      build4.code[0], 0xD63F0000u32,
      "blr(x0) instruction mismatch"
    );

    let mut build5 = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build5.ret();
    build5.finalize();
    assert_eq!(build5.code[0], 0xD65F03C0u32, "ret() instruction mismatch");
  }
}

mod assembly_builder_a_64_fp_basic {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_fp_basic() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(build: &mut AssemblyBuilderA64, expected: u32) {
      build.finalize();
      assert_eq!(build.code[0], expected, "instruction byte mismatch");
    }

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    let d0 = RegisterA64::D0;
    let d1 = RegisterA64::D1;
    let x1 = RegisterA64::X1;
    let x3 = RegisterA64::X3;
    let d2 = RegisterA64::D2;

    build.fmov_register_a_64_register_a_64(d0, d1);
    check(&mut build, 0x1E604020);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_register_a_64(d0, x1);
    check(&mut build, 0x9E670020);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_register_a_64(x3, d2);
    check(&mut build, 0x9E660043);
  }
}

mod assembly_builder_a_64_fp_compare {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_fp_compare() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), expected: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code.len(), 1, "expected exactly one instruction");
      assert_eq!(build.code[0], expected, "instruction word mismatch");
    }

    check(|b| b.fcmp(RegisterA64::D0, RegisterA64::D1), 0x1E612000);
    check(|b| b.fcmpz(RegisterA64::D1), 0x1E602028);

    check(
      |b| b.fcmeq_4s(RegisterA64::Q1, RegisterA64::Q2, RegisterA64::Q3),
      0x4E23E441,
    );
    check(
      |b| b.fcmgt_4s(RegisterA64::Q1, RegisterA64::Q2, RegisterA64::Q3),
      0x6EA3E441,
    );
  }
}

mod assembly_builder_a_64_fp_imm {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_fp_imm() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(build: &mut AssemblyBuilderA64, expected: u32) {
      build.finalize();
      assert_eq!(build.code[0], expected, "instruction byte mismatch");
    }

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::D0, 0.0);
    check(&mut build, 0x2F00E400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::D0, 0.125);
    check(&mut build, 0x1E681000);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::D0, -0.125);
    check(&mut build, 0x1E781000);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::D0, 1.9375);
    check(&mut build, 0x1E6FF000);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::Q0, 0.0);
    check(&mut build, 0x4F000400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::Q0, 0.125);
    check(&mut build, 0x4F02F400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::Q0, -0.125);
    check(&mut build, 0x4F06F400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::Q0, 1.9375);
    check(&mut build, 0x4F03F7E0);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    assert!(!build.is_fmov_supported_fp_64(-0.0));

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    assert!(!build.is_fmov_supported_fp_64(0.12389));
  }
}

mod assembly_builder_a_64_fp_insert_extract {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_fp_insert_extract() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(
      |b| b.ins_4_s_register_a_64_register_a_64_u8(RegisterA64::Q29, RegisterA64::W17, 3),
      0x4E1C1E3D,
    );
    check(
      |b| b.ins_4_s_register_a_64_u8_register_a_64_u8(RegisterA64::Q31, 0, RegisterA64::Q29, 0),
      0x6E0407BF,
    );
    check(
      |b| b.dup_4s(RegisterA64::S29, RegisterA64::Q31, 2),
      0x5E1407FD,
    );
    check(
      |b| b.dup_4s(RegisterA64::Q29, RegisterA64::Q30, 0),
      0x4E0407DD,
    );
    check(
      |b| b.umov_4s(RegisterA64::W1, RegisterA64::Q30, 3),
      0x0E1C3FC1,
    );
    check(
      |b| b.umov_4s(RegisterA64::W13, RegisterA64::Q1, 1),
      0x0E0C3C2D,
    );

    check(
      |b| b.bit(RegisterA64::Q1, RegisterA64::Q2, RegisterA64::Q3),
      0x6EA31C41,
    );
    check(
      |b| b.bif(RegisterA64::Q1, RegisterA64::Q2, RegisterA64::Q3),
      0x6EE31C41,
    );
  }
}

mod assembly_builder_a_64_fp_load_store {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_fp_load_store() {
    use ulua_code_gen::{
      records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
      type_aliases::mem::mem,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    // address forms
    check(
      |b| b.ldr(RegisterA64::D0, mem(RegisterA64::X1, 0)),
      0xFD400020,
    );
    check(
      |b| b.ldr(RegisterA64::D0, mem(RegisterA64::X1, 8)),
      0xFD400420,
    );
    check(
      |b| b.ldr(RegisterA64::D0, mem(RegisterA64::X1, RegisterA64::X7)),
      0xFC676820,
    );
    check(
      |b| b.ldr(RegisterA64::D0, mem(RegisterA64::X1, -7)),
      0xFC5F9020,
    );
    check(
      |b| b.str(RegisterA64::D0, mem(RegisterA64::X1, 0)),
      0xFD000020,
    );
    check(
      |b| b.str(RegisterA64::D0, mem(RegisterA64::X1, 8)),
      0xFD000420,
    );
    check(
      |b| b.str(RegisterA64::D0, mem(RegisterA64::X1, RegisterA64::X7)),
      0xFC276820,
    );
    check(
      |b| b.str(RegisterA64::D0, mem(RegisterA64::X1, -7)),
      0xFC1F9020,
    );

    // load/store sizes
    check(
      |b| b.ldr(RegisterA64::S0, mem(RegisterA64::X1, 0)),
      0xBD400020,
    );
    check(
      |b| b.ldr(RegisterA64::D0, mem(RegisterA64::X1, 0)),
      0xFD400020,
    );
    check(
      |b| b.ldr(RegisterA64::Q0, mem(RegisterA64::X1, 0)),
      0x3DC00020,
    );
    check(
      |b| b.str(RegisterA64::S0, mem(RegisterA64::X1, 0)),
      0xBD000020,
    );
    check(
      |b| b.str(RegisterA64::D0, mem(RegisterA64::X1, 0)),
      0xFD000020,
    );
    check(
      |b| b.str(RegisterA64::Q0, mem(RegisterA64::X1, 0)),
      0x3D800020,
    );

    // load/store sizes x offset scaling
    check(
      |b| b.ldr(RegisterA64::Q0, mem(RegisterA64::X1, 16)),
      0x3DC00420,
    );
    check(
      |b| b.ldr(RegisterA64::D0, mem(RegisterA64::X1, 16)),
      0xFD400820,
    );
    check(
      |b| b.ldr(RegisterA64::S0, mem(RegisterA64::X1, 16)),
      0xBD401020,
    );
    check(
      |b| b.str(RegisterA64::Q0, mem(RegisterA64::X1, 16)),
      0x3D800420,
    );
    check(
      |b| b.str(RegisterA64::D0, mem(RegisterA64::X1, 16)),
      0xFD000820,
    );
    check(
      |b| b.str(RegisterA64::S0, mem(RegisterA64::X1, 16)),
      0xBD001020,
    );
  }
}

mod assembly_builder_a_64_fp_math {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_fp_math() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), expected: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], expected, "instruction word mismatch");
    }

    let d1 = RegisterA64::D1;
    let d2 = RegisterA64::D2;
    let d3 = RegisterA64::D3;
    let s1 = RegisterA64::S1;
    let s2 = RegisterA64::S2;
    let s29 = RegisterA64::S29;
    let s28 = RegisterA64::S28;
    let q1 = RegisterA64::Q1;
    let q2 = RegisterA64::Q2;
    let q29 = RegisterA64::Q29;
    let q28 = RegisterA64::Q28;
    let w1 = RegisterA64::W1;
    let w2 = RegisterA64::W2;
    let x1 = RegisterA64::X1;
    let x2 = RegisterA64::X2;
    let s30 = RegisterA64::S30;
    let q30 = RegisterA64::Q30;
    let d28 = RegisterA64::D28;
    let d29 = RegisterA64::D29;

    check(|b| b.fabs(d1, d2), 0x1E60C041);
    check(|b| b.fabs(s1, s2), 0x1E20C041);
    check(|b| b.fabs(q1, q2), 0x4EA0F841);
    check(|b| b.fadd(d1, d2, d3), 0x1E632841);
    check(|b| b.fadd(s29, s29, s28), 0x1E3C2BBD);
    check(|b| b.fadd(q29, q29, q28), 0x4E3CD7BD);
    check(|b| b.fdiv(d1, d2, d3), 0x1E631841);
    check(|b| b.fdiv(s29, s29, s28), 0x1E3C1BBD);
    check(|b| b.fdiv(q29, q29, q28), 0x6E3CFFBD);
    check(|b| b.fmul(d1, d2, d3), 0x1E630841);
    check(|b| b.fmul(s29, s29, s28), 0x1E3C0BBD);
    check(|b| b.fmul(q29, q29, q28), 0x6E3CDFBD);
    check(|b| b.fneg(d1, d2), 0x1E614041);
    check(|b| b.fneg(s30, s30), 0x1E2143DE);
    check(|b| b.fneg(q30, q30), 0x6EA0FBDE);
    check(|b| b.fsqrt(d1, d2), 0x1E61C041);
    check(|b| b.fsub(d1, d2, d3), 0x1E633841);
    check(|b| b.fsub(s29, s29, s28), 0x1E3C3BBD);
    check(|b| b.fsub(q29, q29, q28), 0x4EBCD7BD);

    check(|b| b.faddp(s29, s28), 0x7E30DB9D);
    check(|b| b.faddp(d29, d28), 0x7E70DB9D);

    check(|b| b.frinta(d1, d2), 0x1E664041);
    check(|b| b.frintm(d1, d2), 0x1E654041);
    check(|b| b.frintp(d1, d2), 0x1E64C041);

    check(|b| b.frinta(s1, s2), 0x1E264041);
    check(|b| b.frintm(s1, s2), 0x1E254041);
    check(|b| b.frintp(s1, s2), 0x1E24C041);

    check(|b| b.frinta(q1, q2), 0x6E218841);
    check(|b| b.frintm(q1, q2), 0x4E219841);
    check(|b| b.frintp(q1, q2), 0x4EA18841);

    check(|b| b.fcvt(s1, d2), 0x1E624041);
    check(|b| b.fcvt(d1, s2), 0x1E22C041);

    check(|b| b.fcvtzs(w1, d2), 0x1E780041);
    check(|b| b.fcvtzs(x1, d2), 0x9E780041);
    check(|b| b.fcvtzu(w1, d2), 0x1E790041);
    check(|b| b.fcvtzu(x1, d2), 0x9E790041);

    check(|b| b.scvtf(d1, w2), 0x1E620041);
    check(|b| b.scvtf(d1, x2), 0x9E620041);

    check(|b| b.ucvtf(d1, w2), 0x1E630041);
    check(|b| b.ucvtf(d1, x2), 0x9E630041);
    check(|b| b.ucvtf(s1, w2), 0x1E230041);
    check(|b| b.ucvtf(s1, x2), 0x9E230041);

    // upstream passes A64::Feature_JSCVT to check() for this instruction
    use ulua_code_gen::enums::features_a_64::FeaturesA64;
    let mut build =
      AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, FeaturesA64::FEATURE_JSCVT as u32);
    build.fjcvtzs(w1, d2);
    build.finalize();
    assert_eq!(
      build.code[0], 0x1E7E0041,
      "instruction word mismatch for fjcvtzs"
    );
  }
}

mod assembly_builder_a_64_loads {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_loads() {
    use ulua_code_gen::{
      records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
      type_aliases::mem::mem,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    // address forms
    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, 0)),
      0xF9400020,
    );
    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, 8)),
      0xF9400420,
    );
    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, RegisterA64::X7)),
      0xF8676820,
    );
    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, -7)),
      0xF85F9020,
    );

    // load sizes
    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, 0)),
      0xF9400020,
    );
    check(
      |b| b.ldr(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0xB9400020,
    );
    check(
      |b| b.ldrb(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0x39400020,
    );
    check(
      |b| b.ldrh(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0x79400020,
    );
    check(
      |b| b.ldrsb(RegisterA64::X0, mem(RegisterA64::X1, 0)),
      0x39800020,
    );
    check(
      |b| b.ldrsb(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0x39C00020,
    );
    check(
      |b| b.ldrsh(RegisterA64::X0, mem(RegisterA64::X1, 0)),
      0x79800020,
    );
    check(
      |b| b.ldrsh(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0x79C00020,
    );
    check(
      |b| b.ldrsw(RegisterA64::X0, mem(RegisterA64::X1, 0)),
      0xB9800020,
    );

    // load sizes x offset scaling
    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, 8)),
      0xF9400420,
    );
    check(
      |b| b.ldr(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0xB9400820,
    );
    check(
      |b| b.ldrb(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0x39402020,
    );
    check(
      |b| b.ldrh(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0x79401020,
    );
    check(
      |b| b.ldrsb(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0x39C02020,
    );
    check(
      |b| b.ldrsh(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0x79C01020,
    );

    // paired loads
    check(
      |b| b.ldp(RegisterA64::X0, RegisterA64::X1, mem(RegisterA64::X2, 8)),
      0xA9408440,
    );
    check(
      |b| b.ldp(RegisterA64::W0, RegisterA64::W1, mem(RegisterA64::X2, -8)),
      0x297F0440,
    );
  }
}

mod assembly_builder_a_64_log_test {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_log_test() {
    use ulua_code_gen::{
      enums::{address_kind_a_64::AddressKindA64, condition_a_64::ConditionA64},
      records::{
        address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64, label::Label,
        register_a_64::RegisterA64 as R,
      },
      type_aliases::mem::mem,
    };

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(true, 0);

    build.add_register_a_64_register_a_64_u16(R::SP, R::SP, 4);
    build.add_register_a_64_register_a_64_register_a_64_i32(R::W0, R::W1, R::W2, 0);
    build.add_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, 2);
    build.add_register_a_64_register_a_64_register_a_64_i32(R::X0, R::X1, R::X2, -2);
    build.add_register_a_64_register_a_64_u16(R::W7, R::W8, 5);
    build.add_register_a_64_register_a_64_u16(R::X7, R::X8, 5);
    build.ldr(R::X7, mem(R::X8, 0));
    build.ldr(R::X7, mem(R::X8, 8));
    build.ldr(R::X7, mem(R::X8, R::X9));
    build.mov_register_a_64_register_a_64(R::X1, R::X2);
    build.movk(R::X1, 42, 16);
    build.cmp_register_a_64_register_a_64(R::X1, R::X2);
    build.blr(R::X0);

    let mut l = Label::default();
    build.b_condition_a_64_label(ConditionA64::Plus, &mut l);
    build.cbz(R::X7, &mut l);

    build.ldp(R::X0, R::X1, mem(R::X8, 8));
    build.adr_register_a_64_label(R::X0, &mut l);

    build.fabs(R::D1, R::D2);
    build.ldr(R::Q1, mem(R::X2, 0));

    build.csel(R::X0, R::X1, R::X2, ConditionA64::Equal);
    build.cset(R::X0, ConditionA64::Equal);

    build.fcmp(R::D0, R::D1);
    build.fcmpz(R::D0);

    build.fmov_register_a_64_f64(R::D0, 0.25);
    build.tbz(R::X0, 5, &mut l);

    build.fcvt(R::S1, R::D2);

    build.ubfx(R::X1, R::X2, 37, 5);

    build.ldr(R::X0, mem(R::X1, 1));
    build.ldr(
      R::X0,
      AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(R::X1, 1, AddressKindA64::Pre),
    );
    build.ldr(
      R::X0,
      AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(R::X1, 1, AddressKindA64::Post),
    );

    build.add_register_a_64_register_a_64_register_a_64_i32(R::X1, R::X2, R::W3, 3);

    build.ins_4_s_register_a_64_register_a_64_u8(R::Q29, R::W17, 3);
    build.ins_4_s_register_a_64_u8_register_a_64_u8(R::Q31, 1, R::Q29, 2);
    build.dup_4s(R::S29, R::Q31, 2);
    build.dup_4s(R::Q29, R::Q30, 0);
    build.umov_4s(R::W1, R::Q30, 3);
    build.fmul(R::Q0, R::Q1, R::Q2);

    build.fcmeq_4s(R::Q2, R::Q0, R::Q1);
    build.bit(R::Q1, R::Q0, R::Q2);

    build.set_label_label(&mut l);
    build.ret();

    build.finalize();

    let expected = "\n add         sp,sp,#4\n add         w0,w1,w2\n add         x0,x1,x2 LSL #2\n add         x0,x1,x2 LSR #2\n add         w7,w8,#5\n add         x7,x8,#5\n ldr         x7,[x8]\n ldr         x7,[x8,#8]\n ldr         x7,[x8,x9]\n mov         x1,x2\n movk        x1,#42 LSL #16\n cmp         x1,x2\n blr         x0\n b.pl        .L1\n cbz         x7,.L1\n ldp         x0,x1,[x8,#8]\n adr         x0,.L1\n fabs        d1,d2\n ldr         q1,[x2]\n csel        x0,x1,x2,eq\n cset        x0,eq\n fcmp        d0,d1\n fcmp        d0,#0\n fmov        d0,#0.25\n tbz         x0,#5,.L1\n fcvt        s1,d2\n ubfx        x1,x2,#3705\n ldr         x0,[x1,#1]\n ldr         x0,[x1,#1]!\n ldr         x0,[x1]!,#1\n add         x1,x2,w3 UXTW #3\n ins         v29.s[3],w17\n ins         v31.s[1],v29.s[2]\n dup         s29,v31.s[2]\n dup         v29.4s,v30.s[0]\n umov        w1,v30.s[3]\n fmul        v0.4s,v1.4s,v2.4s\n fcmeq       v2.4s,v0.4s,v1.4s\n bit         v1.16b,v0.16b,v2.16b\n.L1:\n ret\n";

    assert_eq!(
      format!("\n{}", build.text),
      expected,
      "disasm text mismatch"
    );
  }
}

mod assembly_builder_a_64_moves {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_moves() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_register_a_64(RegisterA64::X0, RegisterA64::X1);
    build.finalize();
    assert_eq!(build.code[0], 0xAA0103E0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_register_a_64(RegisterA64::W0, RegisterA64::W1);
    build.finalize();
    assert_eq!(build.code[0], 0x2A0103E0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_register_a_64(RegisterA64::Q0, RegisterA64::Q1);
    build.finalize();
    assert_eq!(build.code[0], 0x4EA11C20, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movz(RegisterA64::X0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0xD2800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movz(RegisterA64::W0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0x52800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movn(RegisterA64::X0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0x92800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movn(RegisterA64::W0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0x12800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movk(RegisterA64::X0, 42, 16);
    build.finalize();
    assert_eq!(build.code[0], 0xF2A00540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::X0, 42);
    build.finalize();
    assert_eq!(build.code[0], 0xD2800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::X0, 424242);
    build.finalize();
    assert_eq!(build.code[0], 0xD28F2640, "instruction byte mismatch");
    assert_eq!(build.code[1], 0xF2A000C0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::X0, -42);
    build.finalize();
    assert_eq!(build.code[0], 0x92800520, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::X0, -424242);
    build.finalize();
    assert_eq!(build.code[0], 0x928F2620, "instruction byte mismatch");
    assert_eq!(build.code[1], 0xF2BFFF20, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::X0, -65536);
    build.finalize();
    assert_eq!(build.code[0], 0x929FFFE0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::X0, -65537);
    build.finalize();
    assert_eq!(build.code[0], 0x92800000, "instruction byte mismatch");
    assert_eq!(build.code[1], 0xF2BFFFC0, "instruction byte mismatch");
  }
}

mod assembly_builder_a_64_mul_div {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_mul_div() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64 as R,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(|b| b.mul(R::X0, R::X1, R::X2), 0x9B027C20);
    check(|b| b.mul(R::W0, R::W1, R::W2), 0x1B027C20);
    check(|b| b.sdiv(R::X0, R::X1, R::X2), 0x9AC20C20);
    check(|b| b.sdiv(R::W0, R::W1, R::W2), 0x1AC20C20);
    check(|b| b.udiv(R::X0, R::X1, R::X2), 0x9AC20820);
    check(|b| b.udiv(R::W0, R::W1, R::W2), 0x1AC20820);
  }
}

mod assembly_builder_a_64_nop {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_nop() {
    use alloc::vec::Vec;

    use ulua_unit_test::records::assembly_builder_a_64_fixture::AssemblyBuilderA64Fixture;

    let mut fixture = AssemblyBuilderA64Fixture::default();

    // 0 bytes: no instructions emitted
    assert!(fixture.check(
      |b| {
        b.nop(0);
      },
      Vec::new(),
      Vec::new(),
      0,
    ));

    // Non-multiple of 4: rounds down to nearest multiple (7 -> 1 NOP = 4 bytes)
    assert!(fixture.check(
      |b| {
        b.nop(7);
      },
      vec![0xD503201F],
      Vec::new(),
      0,
    ));

    // Exact multiples: 4 -> 1 NOP, 8 -> 2 NOPs, 12 -> 3 NOPs
    assert!(fixture.check(
      |b| {
        b.nop(4);
      },
      vec![0xD503201F],
      Vec::new(),
      0,
    ));

    assert!(fixture.check(
      |b| {
        b.nop(8);
      },
      vec![0xD503201F, 0xD503201F],
      Vec::new(),
      0,
    ));

    assert!(fixture.check(
      |b| {
        b.nop(12);
      },
      vec![0xD503201F, 0xD503201F, 0xD503201F],
      Vec::new(),
      0,
    ));
  }
}

mod assembly_builder_a_64_pre_post_indexing {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_pre_post_indexing() {
    use ulua_code_gen::{
      enums::address_kind_a_64::AddressKindA64,
      records::{
        address_a_64::AddressA64, assembly_builder_a_64::AssemblyBuilderA64,
        register_a_64::RegisterA64,
      },
      type_aliases::mem::mem,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::X1, 1)),
      0xF8401020,
    );
    check(
      |b| {
        b.ldr(
          RegisterA64::X0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Pre,
          ),
        )
      },
      0xF8401C20,
    );
    check(
      |b| {
        b.ldr(
          RegisterA64::X0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Post,
          ),
        )
      },
      0xF8401420,
    );

    check(
      |b| b.ldr(RegisterA64::Q0, mem(RegisterA64::X1, 1)),
      0x3CC01020,
    );
    check(
      |b| {
        b.ldr(
          RegisterA64::Q0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Pre,
          ),
        )
      },
      0x3CC01C20,
    );
    check(
      |b| {
        b.ldr(
          RegisterA64::Q0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Post,
          ),
        )
      },
      0x3CC01420,
    );

    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::X1, 1)),
      0xF8001020,
    );
    check(
      |b| {
        b.str(
          RegisterA64::X0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Pre,
          ),
        )
      },
      0xF8001C20,
    );
    check(
      |b| {
        b.str(
          RegisterA64::X0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Post,
          ),
        )
      },
      0xF8001420,
    );

    check(
      |b| b.str(RegisterA64::Q0, mem(RegisterA64::X1, 1)),
      0x3C801020,
    );
    check(
      |b| {
        b.str(
          RegisterA64::Q0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Pre,
          ),
        )
      },
      0x3C801C20,
    );
    check(
      |b| {
        b.str(
          RegisterA64::Q0,
          AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            RegisterA64::X1,
            1,
            AddressKindA64::Post,
          ),
        )
      },
      0x3C801420,
    );
  }
}

mod assembly_builder_a_64_simd_math {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_simd_math() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), expected: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], expected, "instruction word mismatch");
    }

    let q0 = RegisterA64::Q0;
    let q1 = RegisterA64::Q1;
    let q2 = RegisterA64::Q2;

    check(|b| b.fadd(q0, q1, q2), 0x4E22D420);
    check(|b| b.fsub(q0, q1, q2), 0x4EA2D420);
    check(|b| b.fmul(q0, q1, q2), 0x6E22DC20);
    check(|b| b.fdiv(q0, q1, q2), 0x6E22FC20);
    check(|b| b.fneg(q0, q1), 0x6EA0F820);
  }
}

mod assembly_builder_a_64_stack_ops {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_stack_ops() {
    use ulua_code_gen::{
      records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
      type_aliases::mem::mem,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
      |b| b.mov_register_a_64_register_a_64(RegisterA64::X0, RegisterA64::SP),
      0x910003E0,
    );
    check(
      |b| b.mov_register_a_64_register_a_64(RegisterA64::SP, RegisterA64::X0),
      0x9100001F,
    );

    check(
      |b| b.add_register_a_64_register_a_64_u16(RegisterA64::SP, RegisterA64::SP, 4),
      0x910013FF,
    );
    check(
      |b| b.sub_register_a_64_register_a_64_u16(RegisterA64::SP, RegisterA64::SP, 4),
      0xD10013FF,
    );

    check(
      |b| b.add_register_a_64_register_a_64_u16(RegisterA64::X0, RegisterA64::SP, 4),
      0x910013E0,
    );
    check(
      |b| b.sub_register_a_64_register_a_64_u16(RegisterA64::SP, RegisterA64::X0, 4),
      0xD100101F,
    );

    check(
      |b| b.ldr(RegisterA64::X0, mem(RegisterA64::SP, 8)),
      0xF94007E0,
    );
    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::SP, 8)),
      0xF90007E0,
    );
  }
}

mod assembly_builder_a_64_stores {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_stores() {
    use ulua_code_gen::{
      records::{assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64},
      type_aliases::mem::mem,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    // address forms
    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::X1, 0)),
      0xF9000020,
    );
    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::X1, 8)),
      0xF9000420,
    );
    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::X1, RegisterA64::X7)),
      0xF8276820,
    );
    check(
      |b| b.strh(RegisterA64::W0, mem(RegisterA64::X1, -7)),
      0x781F9020,
    );

    // store sizes
    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::X1, 0)),
      0xF9000020,
    );
    check(
      |b| b.str(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0xB9000020,
    );
    check(
      |b| b.strb(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0x39000020,
    );
    check(
      |b| b.strh(RegisterA64::W0, mem(RegisterA64::X1, 0)),
      0x79000020,
    );

    // store sizes x offset scaling
    check(
      |b| b.str(RegisterA64::X0, mem(RegisterA64::X1, 8)),
      0xF9000420,
    );
    check(
      |b| b.str(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0xB9000820,
    );
    check(
      |b| b.strb(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0x39002020,
    );
    check(
      |b| b.strh(RegisterA64::W0, mem(RegisterA64::X1, 8)),
      0x79001020,
    );

    // paired stores
    check(
      |b| b.stp(RegisterA64::X0, RegisterA64::X1, mem(RegisterA64::X2, 8)),
      0xA9008440,
    );
    check(
      |b| b.stp(RegisterA64::W0, RegisterA64::W1, mem(RegisterA64::X2, -8)),
      0x293F0440,
    );
  }
}

mod assembly_builder_a_64_ternary {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_ternary() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(
      |b| {
        b.msub(
          RegisterA64::X0,
          RegisterA64::X1,
          RegisterA64::X2,
          RegisterA64::X3,
        )
      },
      0x9B028C20,
    );
    check(
      |b| {
        b.msub(
          RegisterA64::W0,
          RegisterA64::W1,
          RegisterA64::W2,
          RegisterA64::W3,
        )
      },
      0x1B028C20,
    );
  }
}

mod assembly_builder_a_64_unary {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_unary() {
    use ulua_code_gen::records::{
      assembly_builder_a_64::AssemblyBuilderA64, register_a_64::RegisterA64,
    };

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
      let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
      f(&mut build);
      build.finalize();
      assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(|b| b.neg(RegisterA64::X0, RegisterA64::X1), 0xCB0103E0);
    check(|b| b.neg(RegisterA64::W0, RegisterA64::W1), 0x4B0103E0);
    check(|b| b.mvn_(RegisterA64::X0, RegisterA64::X1), 0xAA2103E0);

    check(|b| b.clz(RegisterA64::X0, RegisterA64::X1), 0xDAC01020);
    check(|b| b.clz(RegisterA64::W0, RegisterA64::W1), 0x5AC01020);
    check(|b| b.rbit(RegisterA64::X0, RegisterA64::X1), 0xDAC00020);
    check(|b| b.rbit(RegisterA64::W0, RegisterA64::W1), 0x5AC00020);
    check(|b| b.rev(RegisterA64::W0, RegisterA64::W1), 0x5AC00820);
    check(|b| b.rev(RegisterA64::X0, RegisterA64::X1), 0xDAC00C20);
  }
}

mod assembly_builder_a_64_undefined {
  #[cfg(test)]
  #[test]
  fn assembly_builder_a_64_undefined() {
    use ulua_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.udf();
    build.finalize();
    assert_eq!(build.code[0], 0x00000000, "instruction byte mismatch");
  }
}
