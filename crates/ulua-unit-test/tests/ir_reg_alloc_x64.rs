extern crate alloc;

mod ir_reg_alloc_x_64_relocate_fix {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/IrRegAllocX64.test.cpp:32:ir_reg_alloc_x_64_relocate_fix`
  //! Source: `tests/IrRegAllocX64.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/IrRegAllocX64.test.cpp
  //! - source_includes:
  //!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
  //! - incoming:
  //!   - declares <- source_file tests/IrRegAllocX64.test.cpp
  //! - outgoing:
  //!   - type_ref -> record IrInst (CodeGen/include/Luau/IrData.h)
  //!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
  //!   - calls -> method IrRegAllocX64::preserve (CodeGen/src/IrRegAllocX64.cpp)
  //!   - calls -> method IrRegAllocX64Fixture::checkMatch (tests/IrRegAllocX64.test.cpp)
  //!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
  //!   - translates_to -> rust_item ir_reg_alloc_x_64_relocate_fix

  #[cfg(test)]
  #[test]
  fn ir_reg_alloc_x64_relocate_fix() {
    use alloc::string::String;

    use ulua_code_gen::{
      enums::ir_cmd::IrCmd,
      records::{ir_inst::IrInst, register_x_64::RegisterX64},
    };
    use ulua_unit_test::records::ir_reg_alloc_x_64_fixture::IrRegAllocX64Fixture;

    let mut fixture = IrRegAllocX64Fixture::new();

    let ir_inst0 = IrInst {
      cmd: IrCmd::LoadDouble,
      last_use: 2,
      ..Default::default()
    };
    fixture.function.instructions.push(ir_inst0);

    let ir_inst1 = IrInst {
      cmd: IrCmd::LoadDouble,
      last_use: 2,
      ..Default::default()
    };
    fixture.function.instructions.push(ir_inst1);

    let reg0 = fixture.regs.take_reg(RegisterX64::RAX, 0);
    fixture.function.instructions[0].reg_x64 = reg0;
    fixture.regs.preserve(&mut fixture.function.instructions[0]);

    let reg1 = fixture.regs.take_reg(RegisterX64::RAX, 1);
    fixture.function.instructions[1].reg_x64 = reg1;
    fixture
      .regs
      .restore(&mut fixture.function.instructions[0], true);

    assert_eq!(fixture.function.instructions[0].reg_x64, RegisterX64::RAX);
    assert!(fixture.function.instructions[1].spilled);

    fixture.check_match(String::from(
        "\n vmovsd      qword ptr [rsp+048h],rax\n vmovsd      qword ptr [rsp+050h],rax\n vmovsd      rax,qword ptr [rsp+048h]\n",
    ));
  }
}
